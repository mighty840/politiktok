use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::coalition_detector::{
    analyze_tensions, get_default_segments, CoalitionSegment, Tension, TensionAnalysis,
};

#[component]
pub fn CoalitionPage() -> Element {
    // Input state
    let mut policy_text = use_signal(String::new);
    let mut selected_segments = use_signal(Vec::<CoalitionSegment>::new);

    // Analysis state
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis = use_signal(|| Option::<TensionAnalysis>::None);

    // Fetch default segments on mount
    let segments_resource = use_resource(|| async move {
        get_default_segments().await.unwrap_or_default()
    });

    // Toggle a segment in the selected list
    let mut toggle_segment = move |segment: CoalitionSegment| {
        let mut selected = selected_segments.write();
        if let Some(pos) = selected.iter().position(|s| s.name == segment.name) {
            selected.remove(pos);
        } else {
            selected.push(segment);
        }
    };

    // Select all segments
    let select_all = move |_| {
        if let Some(all) = segments_resource.read().as_ref() {
            selected_segments.set(all.clone());
        }
    };

    // Clear all segments
    let clear_all = move |_| {
        selected_segments.set(Vec::new());
    };

    // Handle analyze
    let on_analyze = move |_| {
        let text = policy_text().trim().to_string();
        if text.is_empty() {
            error_msg.set(Some("Please enter policy text to analyze.".to_string()));
            return;
        }

        let segments = selected_segments();
        if segments.len() < 2 {
            error_msg.set(Some(
                "Please select at least two coalition segments.".to_string(),
            ));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);
        analysis.set(None);

        spawn(async move {
            match analyze_tensions(text, segments).await {
                Ok(result) => {
                    analysis.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Analysis failed: {e}")));
                }
            }
            is_analyzing.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Coalition Tension Detector" }
                p { class: "text-base-content/70",
                    "Analyze how policies affect political coalitions and identify emerging tensions between constituent groups."
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

            // Input section
            div { class: "flex flex-col lg:flex-row gap-6",
                // Left: Policy text input
                div { class: "w-full lg:w-1/2",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Policy Text" }
                            p { class: "text-sm text-base-content/60",
                                "Enter a policy proposal, legislative text, or platform position to analyze."
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Enter policy text to analyze for coalition tensions...",
                                rows: "8",
                                value: "{policy_text}",
                                oninput: move |evt| policy_text.set(evt.value()),
                            }
                        }
                    }
                }

                // Right: Segment selection
                div { class: "w-full lg:w-1/2",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            div { class: "flex items-center justify-between",
                                h2 { class: "card-title text-lg", "Coalition Segments" }
                                div { class: "flex gap-2",
                                    button {
                                        class: "btn btn-ghost btn-xs",
                                        onclick: select_all,
                                        "Select All"
                                    }
                                    button {
                                        class: "btn btn-ghost btn-xs",
                                        onclick: clear_all,
                                        "Clear"
                                    }
                                }
                            }

                            match segments_resource.read().as_ref() {
                                None => rsx! { LoadingSpinner {} },
                                Some(segments) => {
                                    rsx! {
                                        div { class: "space-y-2",
                                            for segment in segments.iter() {
                                                {
                                                    let is_checked = selected_segments()
                                                        .iter()
                                                        .any(|s| s.name == segment.name);
                                                    let seg_for_toggle = segment.clone();
                                                    rsx! {
                                                        label { class: "flex items-start gap-3 cursor-pointer p-2 rounded-lg hover:bg-base-200 transition-colors",
                                                            input {
                                                                r#type: "checkbox",
                                                                class: "checkbox checkbox-primary checkbox-sm mt-0.5",
                                                                checked: is_checked,
                                                                onchange: move |_| toggle_segment(seg_for_toggle.clone()),
                                                            }
                                                            div {
                                                                p { class: "font-medium text-sm", "{segment.name}" }
                                                                p { class: "text-xs text-base-content/50", "{segment.description}" }
                                                                div { class: "flex flex-wrap gap-1 mt-1",
                                                                    for priority in segment.key_priorities.iter() {
                                                                        span { class: "badge badge-ghost badge-xs", "{priority}" }
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

            // Analyze button
            div { class: "flex justify-center",
                button {
                    class: "btn btn-primary btn-lg",
                    disabled: *is_analyzing.read(),
                    onclick: on_analyze,
                    if *is_analyzing.read() {
                        span { class: "loading loading-spinner loading-sm" }
                        "Analyzing Tensions..."
                    } else {
                        "Analyze Tensions"
                    }
                }
            }

            // Loading spinner
            if *is_analyzing.read() {
                div { class: "flex justify-center py-8",
                    LoadingSpinner {}
                }
            }

            // Results
            if let Some(result) = analysis() {
                // Stress score gauge
                StressScoreGauge { score: result.overall_stress_score }

                // Tension list
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-4",
                        h2 { class: "card-title text-lg",
                            "Detected Tensions"
                            span { class: "badge badge-neutral ml-2", "{result.tensions.len()}" }
                        }

                        if result.tensions.is_empty() {
                            p { class: "text-base-content/60 py-4",
                                "No significant tensions detected between the selected segments."
                            }
                        } else {
                            div { class: "space-y-3",
                                for tension in result.tensions.iter() {
                                    TensionCard { tension: tension.clone() }
                                }
                            }
                        }
                    }
                }

                // Recommendations
                if !result.recommendations.is_empty() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h2 { class: "card-title text-lg", "Recommendations" }
                            div { class: "space-y-2 mt-2",
                                for (idx, rec) in result.recommendations.iter().enumerate() {
                                    div { class: "flex gap-3 items-start",
                                        span { class: "badge badge-primary badge-sm mt-0.5",
                                            "{idx + 1}"
                                        }
                                        p { class: "text-sm", "{rec}" }
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

/// Displays the overall coalition stress score as a visual gauge.
#[component]
fn StressScoreGauge(score: f64) -> Element {
    let pct = (score * 100.0) as u32;
    let (color, label) = if score >= 0.75 {
        ("progress-error", "Critical")
    } else if score >= 0.5 {
        ("progress-warning", "High")
    } else if score >= 0.25 {
        ("progress-info", "Moderate")
    } else {
        ("progress-success", "Low")
    };

    rsx! {
        div { class: "card bg-base-100 shadow-sm",
            div { class: "card-body",
                div { class: "flex items-center justify-between mb-2",
                    h2 { class: "card-title text-lg", "Overall Coalition Stress" }
                    div { class: "flex items-center gap-2",
                        span { class: "text-2xl font-bold font-mono", "{pct}%" }
                        span { class: "badge badge-outline badge-sm", "{label}" }
                    }
                }
                progress {
                    class: "progress {color} w-full h-4",
                    value: "{pct}",
                    max: "100",
                }
                p { class: "text-xs text-base-content/50 mt-1",
                    "0% = no strain on coalition, 100% = coalition breaking point"
                }
            }
        }
    }
}

/// Card displaying a single tension between two coalition segments.
#[component]
fn TensionCard(tension: Tension) -> Element {
    let severity_badge = match tension.severity.as_str() {
        "critical" => "badge badge-error",
        "high" => "badge badge-warning",
        "medium" => "badge badge-info",
        "low" => "badge badge-success",
        _ => "badge badge-ghost",
    };

    rsx! {
        div { class: "border border-base-300 rounded-lg p-4 space-y-2",
            div { class: "flex flex-wrap items-center gap-2",
                span { class: "font-medium text-sm", "{tension.segment_a}" }
                span { class: "text-base-content/40 text-xs", "vs" }
                span { class: "font-medium text-sm", "{tension.segment_b}" }
                span { class: "{severity_badge} badge-sm ml-auto", "{tension.severity}" }
            }
            p { class: "text-sm font-semibold text-base-content/80", "{tension.issue}" }
            p { class: "text-sm text-base-content/60", "{tension.explanation}" }
        }
    }
}
