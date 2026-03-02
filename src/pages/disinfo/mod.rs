use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::disinfo_warning::{
    analyze_disinfo, generate_response, CounterResponse, DisinfoAnalysis, DisinfoIndicator,
};

/// Badge class for risk level.
fn risk_badge(level: &str) -> &'static str {
    match level {
        "high" => "badge badge-error badge-lg",
        "medium" => "badge badge-warning badge-lg",
        "low" => "badge badge-success badge-lg",
        _ => "badge badge-ghost badge-lg",
    }
}

/// Progress bar color for confidence.
fn confidence_color(confidence: f64) -> &'static str {
    if confidence >= 0.8 {
        "progress-error"
    } else if confidence >= 0.5 {
        "progress-warning"
    } else {
        "progress-info"
    }
}

#[component]
pub fn DisinfoPage() -> Element {
    // Input state
    let mut content_text = use_signal(String::new);
    let mut source_context = use_signal(String::new);

    // Analysis state
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis_result = use_signal(|| Option::<DisinfoAnalysis>::None);

    // Counter-response state
    let mut target_audience = use_signal(|| "General public".to_string());
    let mut is_generating = use_signal(|| false);
    let mut counter_result = use_signal(|| Option::<CounterResponse>::None);

    let on_analyze = move |_| {
        let text = content_text().trim().to_string();
        let ctx = source_context().trim().to_string();

        if text.is_empty() {
            error_msg.set(Some("Content is required for analysis.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);
        counter_result.set(None);

        spawn(async move {
            match analyze_disinfo(text, ctx).await {
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

    let on_generate_response = move |_| {
        let current_analysis = match analysis_result() {
            Some(a) => a,
            None => return,
        };

        let audience = target_audience().trim().to_string();

        is_generating.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_response(current_analysis.content.clone(), audience).await {
                Ok(result) => {
                    counter_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Response generation failed: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Header
            div {
                h1 { class: "text-3xl font-bold", "Disinformation Watch" }
                p { class: "text-slate-400",
                    "Detect disinformation indicators in content and generate strategic counter-messaging."
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

            // Main layout
            div { class: "flex flex-col lg:flex-row gap-6",
                // Left: Input
                div { class: "w-full lg:w-1/3",
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Content to Analyze" }

                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Suspicious Content" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Paste the content you want to analyze for disinformation...",
                                    rows: "8",
                                    value: "{content_text}",
                                    oninput: move |evt| content_text.set(evt.value()),
                                }
                            }

                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Source Context" }
                                    span { class: "label-text-alt text-slate-500", "Optional" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Where did you find this? Social media, email chain, website...",
                                    rows: "3",
                                    value: "{source_context}",
                                    oninput: move |evt| source_context.set(evt.value()),
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
                                    "Analyze for Disinformation"
                                }
                            }
                        }
                    }
                }

                // Right: Results
                div { class: "w-full lg:w-2/3",
                    if *is_analyzing.read() {
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center space-y-4",
                                    LoadingSpinner {}
                                    p { class: "text-slate-400", "Scanning for disinformation indicators..." }
                                }
                            }
                        }
                    } else if let Some(result) = analysis_result() {
                        div { class: "space-y-4",
                            // Risk level card
                            div { class: "glass-card gradient-border",
                                div { class: "p-6",
                                    div { class: "flex items-center justify-between",
                                        h3 { class: "card-title text-lg", "Risk Assessment" }
                                        span { class: risk_badge(&result.risk_level),
                                            "{result.risk_level} risk"
                                        }
                                    }
                                    div { class: "mt-3 bg-slate-800/30 rounded-lg p-4",
                                        p { class: "font-medium mb-1", "Recommended Response" }
                                        p { class: "text-sm", "{result.recommended_response}" }
                                    }
                                }
                            }

                            // Indicators list
                            if !result.indicators.is_empty() {
                                div { class: "glass-card gradient-border",
                                    div { class: "p-6",
                                        h3 { class: "card-title text-lg",
                                            "Indicators ({result.indicators.len()})"
                                        }
                                        div { class: "space-y-3",
                                            for indicator in result.indicators.iter() {
                                                IndicatorCard { indicator: indicator.clone() }
                                            }
                                        }
                                    }
                                }
                            }

                            // Counter-response generation
                            div { class: "glass-card gradient-border",
                                div { class: "card-body space-y-4",
                                    h3 { class: "card-title text-lg", "Generate Counter-Messaging" }

                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text font-medium", "Target Audience" }
                                        }
                                        select {
                                            class: "select select-bordered w-full",
                                            value: "{target_audience}",
                                            onchange: move |evt: Event<FormData>| target_audience.set(evt.value()),
                                            option { value: "General public", "General Public" }
                                            option { value: "Social media users", "Social Media Users" }
                                            option { value: "Senior citizens", "Senior Citizens" }
                                            option { value: "Young voters (18-30)", "Young Voters (18-30)" }
                                            option { value: "Local community members", "Local Community Members" }
                                            option { value: "Party supporters", "Party Supporters" }
                                        }
                                    }

                                    button {
                                        class: "btn btn-secondary",
                                        disabled: *is_generating.read(),
                                        onclick: on_generate_response,
                                        if *is_generating.read() {
                                            span { class: "loading loading-spinner loading-sm" }
                                            "Generating..."
                                        } else {
                                            "Generate Response"
                                        }
                                    }

                                    if *is_generating.read() {
                                        div { class: "flex items-center justify-center py-4",
                                            LoadingSpinner {}
                                        }
                                    } else if let Some(counter) = counter_result() {
                                        div { class: "space-y-3",
                                            div { class: "badge badge-outline",
                                                "Audience: {counter.target_audience}"
                                            }
                                            div { class: "bg-success/10 rounded-lg p-4 border border-success/20",
                                                p { class: "font-medium text-success mb-2", "Counter-Messaging" }
                                                pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                                    "{counter.messaging}"
                                                }
                                            }
                                            if !counter.key_points.is_empty() {
                                                div {
                                                    p { class: "font-medium text-sm mb-2", "Key Talking Points" }
                                                    ul { class: "list-disc list-inside space-y-1",
                                                        for point in counter.key_points.iter() {
                                                            li { class: "text-sm", "{point}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    p { class: "text-lg font-medium text-slate-500 mb-2",
                                        "No analysis yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Paste suspicious content and click Analyze to detect disinformation indicators."
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

#[component]
fn IndicatorCard(indicator: DisinfoIndicator) -> Element {
    let confidence_pct = (indicator.confidence * 100.0) as u32;

    rsx! {
        div { class: "bg-slate-800/30 rounded-lg p-3",
            div { class: "flex items-center justify-between mb-1",
                span { class: "badge badge-outline badge-sm", "{indicator.indicator_type}" }
                span { class: "text-xs text-slate-400", "{confidence_pct}% confidence" }
            }
            p { class: "text-sm mb-2", "{indicator.description}" }
            progress {
                class: "progress {confidence_color(indicator.confidence)} w-full h-1",
                value: "{confidence_pct}",
                max: "100",
            }
        }
    }
}
