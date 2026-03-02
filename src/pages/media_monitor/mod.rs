use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::media_monitor::{
    analyze_media, compare_coverage, CoverageComparison, MediaAnalysis,
};

/// Badge class for coverage tone.
fn tone_badge(tone: &str) -> &'static str {
    match tone {
        "positive" => "badge badge-success",
        "negative" => "badge badge-error",
        "neutral" => "badge badge-info",
        "mixed" => "badge badge-warning",
        "alarmist" => "badge badge-error",
        "dismissive" => "badge badge-warning",
        _ => "badge badge-ghost",
    }
}

#[component]
pub fn MediaMonitorPage() -> Element {
    // Active section
    let mut active_section = use_signal(|| "analyze".to_string());

    // Single article analysis state
    let mut article_text = use_signal(String::new);
    let mut source_name = use_signal(String::new);
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis_result = use_signal(|| Option::<MediaAnalysis>::None);

    // Comparison state
    let mut compare_topic = use_signal(String::new);
    let mut compare_articles = use_signal(|| vec![
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
    ]);
    let mut is_comparing = use_signal(|| false);
    let mut comparison_result = use_signal(|| Option::<CoverageComparison>::None);

    let on_analyze = move |_| {
        let text = article_text().trim().to_string();
        let source = source_name().trim().to_string();

        if text.is_empty() {
            error_msg.set(Some("Article text is required.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);

        spawn(async move {
            match analyze_media(text, source).await {
                Ok(result) => {
                    analysis_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Analysis failed: {e}")));
                }
            }
            is_analyzing.set(false);
        });
    };

    let on_compare = move |_| {
        let topic = compare_topic().trim().to_string();
        let articles: Vec<(String, String)> = compare_articles()
            .iter()
            .filter(|(source, text)| !source.trim().is_empty() && !text.trim().is_empty())
            .map(|(s, t)| (s.trim().to_string(), t.trim().to_string()))
            .collect();

        if topic.is_empty() {
            error_msg.set(Some("Topic is required for comparison.".to_string()));
            return;
        }
        if articles.len() < 2 {
            error_msg.set(Some("At least two articles with sources are needed for comparison.".to_string()));
            return;
        }

        is_comparing.set(true);
        error_msg.set(None);

        spawn(async move {
            match compare_coverage(topic, articles).await {
                Ok(result) => {
                    comparison_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Comparison failed: {e}")));
                }
            }
            is_comparing.set(false);
        });
    };

    let add_article_slot = move |_| {
        let mut arts = compare_articles.write();
        arts.push(("".to_string(), "".to_string()));
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Header
            div {
                h1 { class: "text-3xl font-bold", "Media Monitor" }
                p { class: "text-slate-400",
                    "Analyze media coverage for bias, key claims, and tone. Compare how different sources cover the same topic."
                }
            }

            // Error alert
            if let Some(err) = error_msg() {
                div { class: "alert alert-error shadow-sm",
                    svg {
                        class: "stroke-current shrink-0 h-6 w-6",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { "{err}" }
                    button {
                        class: "btn btn-ghost btn-xs",
                        onclick: move |_| error_msg.set(None),
                        "Dismiss"
                    }
                }
            }

            // Section tabs
            div { class: "tabs tabs-bordered",
                button {
                    class: if active_section() == "analyze" { "tab tab-active" } else { "tab" },
                    onclick: move |_| active_section.set("analyze".to_string()),
                    "Article Analysis"
                }
                button {
                    class: if active_section() == "compare" { "tab tab-active" } else { "tab" },
                    onclick: move |_| active_section.set("compare".to_string()),
                    "Coverage Comparison"
                }
            }

            // Article Analysis section
            if active_section() == "analyze" {
                div { class: "flex flex-col lg:flex-row gap-6",
                    // Input
                    div { class: "w-full lg:w-1/3",
                        div { class: "glass-card gradient-border",
                            div { class: "card-body space-y-4",
                                h2 { class: "card-title text-lg", "Input" }

                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "Source Name" }
                                    }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        placeholder: "e.g., CNN, Fox News, Reuters",
                                        value: "{source_name}",
                                        oninput: move |evt| source_name.set(evt.value()),
                                    }
                                }

                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text font-medium", "Article Text" }
                                    }
                                    textarea {
                                        class: "textarea textarea-bordered w-full",
                                        placeholder: "Paste the full article text here...",
                                        rows: "10",
                                        value: "{article_text}",
                                        oninput: move |evt| article_text.set(evt.value()),
                                    }
                                }

                                button {
                                    class: "btn btn-primary w-full",
                                    disabled: *is_analyzing.read(),
                                    onclick: on_analyze,
                                    if *is_analyzing.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                        "Analyzing..."
                                    } else {
                                        "Analyze Article"
                                    }
                                }
                            }
                        }
                    }

                    // Results
                    div { class: "w-full lg:w-2/3",
                        if *is_analyzing.read() {
                            div { class: "glass-card gradient-border min-h-[300px]",
                                div { class: "card-body flex items-center justify-center",
                                    div { class: "text-center space-y-4",
                                        LoadingSpinner {}
                                        p { class: "text-slate-400", "Analyzing media coverage..." }
                                    }
                                }
                            }
                        } else if let Some(result) = analysis_result() {
                            div { class: "space-y-4",
                                // Summary card
                                div { class: "glass-card gradient-border",
                                    div { class: "p-6",
                                        div { class: "flex items-center gap-3 mb-3",
                                            h3 { class: "card-title text-lg", "{result.source}" }
                                            span { class: tone_badge(&result.coverage_tone),
                                                "{result.coverage_tone}"
                                            }
                                        }
                                        div { class: "bg-slate-800/30 rounded-lg p-4",
                                            p { class: "font-medium mb-1", "Bias Assessment" }
                                            p { class: "text-sm", "{result.bias_assessment}" }
                                        }
                                    }
                                }

                                // Key claims
                                if !result.key_claims.is_empty() {
                                    div { class: "glass-card gradient-border",
                                        div { class: "p-6",
                                            h3 { class: "card-title text-lg", "Key Claims" }
                                            ul { class: "list-disc list-inside space-y-1",
                                                for claim in result.key_claims.iter() {
                                                    li { class: "text-sm", "{claim}" }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Fact-check notes
                                if !result.fact_check_notes.is_empty() {
                                    div { class: "glass-card gradient-border",
                                        div { class: "p-6",
                                            h3 { class: "card-title text-lg", "Fact-Check Notes" }
                                            div { class: "space-y-2",
                                                for note in result.fact_check_notes.iter() {
                                                    div { class: "bg-warning/10 rounded-lg p-3 border border-warning/20",
                                                        p { class: "text-sm", "{note}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            div { class: "glass-card gradient-border min-h-[300px]",
                                div { class: "card-body flex items-center justify-center",
                                    p { class: "text-slate-500",
                                        "Paste an article and click Analyze to see bias assessment, key claims, and fact-check notes."
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Coverage Comparison section
            if active_section() == "compare" {
                div { class: "space-y-4",
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Compare Coverage" }

                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Topic" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "e.g., Immigration reform bill",
                                    value: "{compare_topic}",
                                    oninput: move |evt| compare_topic.set(evt.value()),
                                }
                            }

                            // Article inputs
                            for (idx, (_source, _text)) in compare_articles().iter().enumerate() {
                                {
                                    let source_val = compare_articles().get(idx).map(|a| a.0.clone()).unwrap_or_default();
                                    let text_val = compare_articles().get(idx).map(|a| a.1.clone()).unwrap_or_default();
                                    rsx! {
                                        div { class: "border border-base-300 rounded-lg p-4 space-y-2",
                                            p { class: "font-medium text-sm", "Article #{idx + 1}" }
                                            input {
                                                class: "input input-bordered input-sm w-full",
                                                r#type: "text",
                                                placeholder: "Source name",
                                                value: "{source_val}",
                                                oninput: move |evt: Event<FormData>| {
                                                    let mut arts = compare_articles.write();
                                                    if let Some(entry) = arts.get_mut(idx) {
                                                        entry.0 = evt.value();
                                                    }
                                                },
                                            }
                                            textarea {
                                                class: "textarea textarea-bordered textarea-sm w-full",
                                                placeholder: "Paste article text...",
                                                rows: "4",
                                                value: "{text_val}",
                                                oninput: move |evt: Event<FormData>| {
                                                    let mut arts = compare_articles.write();
                                                    if let Some(entry) = arts.get_mut(idx) {
                                                        entry.1 = evt.value();
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex gap-2",
                                button {
                                    class: "btn btn-outline btn-sm",
                                    onclick: add_article_slot,
                                    "+ Add Source"
                                }
                                button {
                                    class: "btn btn-primary",
                                    disabled: *is_comparing.read(),
                                    onclick: on_compare,
                                    if *is_comparing.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                        "Comparing..."
                                    } else {
                                        "Compare Coverage"
                                    }
                                }
                            }
                        }
                    }

                    if *is_comparing.read() {
                        div { class: "flex items-center justify-center py-8",
                            div { class: "text-center space-y-4",
                                LoadingSpinner {}
                                p { class: "text-slate-400", "Comparing coverage across sources..." }
                            }
                        }
                    } else if let Some(comparison) = comparison_result() {
                        // Overall assessment
                        div { class: "glass-card gradient-border",
                            div { class: "p-6",
                                h3 { class: "card-title text-lg", "Coverage Comparison: {comparison.topic}" }
                                div { class: "bg-slate-800/30 rounded-lg p-4 mb-3",
                                    p { class: "text-sm", "{comparison.overall_assessment}" }
                                }
                                div { class: "badge badge-outline", "Bias spectrum: {comparison.bias_spectrum}" }
                            }
                        }

                        // Per-source cards
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            for sa in comparison.source_analyses.iter() {
                                div { class: "glass-card gradient-border border border-base-300",
                                    div { class: "p-6",
                                        div { class: "flex items-center gap-2 mb-2",
                                            h4 { class: "font-semibold", "{sa.source}" }
                                            span { class: tone_badge(&sa.tone), "{sa.tone}" }
                                        }
                                        div { class: "text-sm space-y-2",
                                            div {
                                                p { class: "font-medium text-xs text-slate-400", "Framing" }
                                                p { "{sa.framing}" }
                                            }
                                            if !sa.key_omissions.is_empty() {
                                                div {
                                                    p { class: "font-medium text-xs text-slate-400", "Key Omissions" }
                                                    ul { class: "list-disc list-inside text-xs",
                                                        for omission in sa.key_omissions.iter() {
                                                            li { "{omission}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
