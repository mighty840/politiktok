use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::call_intelligence::{analyze_call, CallAnalysis};

/// Badge class for a sentiment value.
fn sentiment_badge_class(sentiment: &str) -> &'static str {
    match sentiment {
        "positive" => "badge badge-success",
        "negative" => "badge badge-error",
        "mixed" => "badge badge-warning",
        _ => "badge badge-info",
    }
}

/// Format satisfaction score as a percentage string.
fn format_satisfaction(score: f64) -> String {
    format!("{:.0}%", score * 100.0)
}

#[component]
pub fn CallIntelPage() -> Element {
    // Form state
    let mut transcript = use_signal(String::new);

    // Analysis state
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis = use_signal(|| Option::<CallAnalysis>::None);

    // Handle analyze
    let on_analyze = move |_| {
        let transcript_val = transcript().trim().to_string();

        if transcript_val.is_empty() {
            error_msg.set(Some("Please enter a call transcript to analyze.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);

        spawn(async move {
            match analyze_call(transcript_val).await {
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
                h1 { class: "text-3xl font-bold", "Call Intelligence" }
                p { class: "text-base-content/70",
                    "Analyze constituent call transcripts to extract sentiment, key issues, action items, and satisfaction scores."
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

            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Input
                div { class: "w-full lg:w-1/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Call Transcript" }

                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Transcript" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Paste the call transcript here...\n\nExample:\nCaller: Hi, I'm calling about the pothole on Main Street...\nAgent: Thank you for calling. Let me look into that...",
                                    rows: "12",
                                    value: "{transcript}",
                                    oninput: move |evt| transcript.set(evt.value()),
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
                                    "Analyze"
                                }
                            }
                        }
                    }
                }

                // Right: Results
                div { class: "w-full lg:w-2/3",
                    div { class: "card bg-base-100 shadow-sm min-h-[400px]",
                        div { class: "card-body",
                            h2 { class: "card-title text-lg mb-2", "Analysis Results" }

                            if *is_analyzing.read() {
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center space-y-4",
                                        LoadingSpinner {}
                                        p { class: "text-base-content/60", "Analyzing call transcript..." }
                                    }
                                }
                            } else if let Some(result) = analysis() {
                                div { class: "space-y-6",
                                    // Summary
                                    div {
                                        h3 { class: "font-semibold text-base mb-2", "Summary" }
                                        p { class: "text-base-content/80 bg-base-200 rounded-lg p-4",
                                            "{result.summary}"
                                        }
                                    }

                                    // Sentiment & Satisfaction row
                                    div { class: "flex flex-wrap gap-4",
                                        div { class: "flex items-center gap-2",
                                            span { class: "font-semibold text-sm", "Sentiment:" }
                                            span {
                                                class: sentiment_badge_class(&result.sentiment),
                                                "{result.sentiment}"
                                            }
                                        }
                                        div { class: "flex items-center gap-2",
                                            span { class: "font-semibold text-sm", "Caller Satisfaction:" }
                                            div { class: "flex items-center gap-2",
                                                progress {
                                                    class: "progress progress-primary w-24",
                                                    value: "{result.caller_satisfaction}",
                                                    max: "1",
                                                }
                                                span { class: "text-sm font-mono",
                                                    "{format_satisfaction(result.caller_satisfaction)}"
                                                }
                                            }
                                        }
                                    }

                                    // Key Issues
                                    div {
                                        h3 { class: "font-semibold text-base mb-2", "Key Issues" }
                                        if result.key_issues.is_empty() {
                                            p { class: "text-base-content/50 text-sm", "No key issues identified." }
                                        } else {
                                            ul { class: "list-disc list-inside space-y-1",
                                                for issue in result.key_issues.iter() {
                                                    li { class: "text-base-content/80", "{issue}" }
                                                }
                                            }
                                        }
                                    }

                                    // Action Items
                                    div {
                                        h3 { class: "font-semibold text-base mb-2", "Action Items" }
                                        if result.action_items.is_empty() {
                                            p { class: "text-base-content/50 text-sm", "No action items identified." }
                                        } else {
                                            div { class: "space-y-2",
                                                for item in result.action_items.iter() {
                                                    label { class: "flex items-start gap-2 cursor-pointer",
                                                        input {
                                                            r#type: "checkbox",
                                                            class: "checkbox checkbox-primary checkbox-sm mt-0.5",
                                                        }
                                                        span { class: "text-base-content/80 text-sm", "{item}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center",
                                        p { class: "text-lg font-medium text-base-content/50 mb-2",
                                            "No analysis yet"
                                        }
                                        p { class: "text-sm text-base-content/40",
                                            "Paste a call transcript and click Analyze to get started."
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
