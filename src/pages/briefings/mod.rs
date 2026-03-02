use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::candidate_briefing::{generate_briefing, Briefing};

/// Available event types for briefing generation.
const EVENT_TYPE_OPTIONS: &[&str] = &[
    "Speech",
    "Town Hall",
    "Press Conference",
    "Debate",
];

#[component]
pub fn BriefingsPage() -> Element {
    // Form state
    let mut event_type = use_signal(|| "Speech".to_string());
    let mut topics_text = use_signal(String::new);
    let mut audience = use_signal(String::new);
    let mut context = use_signal(String::new);

    // Generation state
    let mut is_generating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut briefing = use_signal(|| Option::<Briefing>::None);

    // Handle generate
    let on_generate = move |_| {
        let event_type_val = event_type().clone();
        let audience_val = audience().trim().to_string();
        let context_val = context().trim().to_string();
        let topics: Vec<String> = topics_text()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        is_generating.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_briefing(event_type_val, topics, audience_val, context_val).await {
                Ok(b) => {
                    briefing.set(Some(b));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Generation failed: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Candidate Briefings" }
                p { class: "text-slate-400",
                    "Auto-generated briefing documents for candidates covering events, meetings, and media appearances."
                }
            }

            // Error alert
            if let Some(err) = error_msg() {
                div { class: "alert alert-error shadow-sm",
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

                // Left: Input form
                div { class: "w-full lg:w-1/3",
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Generate Briefing" }

                            // Event type
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Event Type" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{event_type}",
                                    onchange: move |evt: Event<FormData>| event_type.set(evt.value()),
                                    for option in EVENT_TYPE_OPTIONS.iter() {
                                        option { value: "{option}", "{option}" }
                                    }
                                }
                            }

                            // Topics
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Topics" }
                                    span { class: "label-text-alt text-slate-500", "One per line" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "e.g., Healthcare reform\nInfrastructure spending\nEducation policy",
                                    rows: "4",
                                    value: "{topics_text}",
                                    oninput: move |evt| topics_text.set(evt.value()),
                                }
                            }

                            // Audience
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Audience" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "e.g., Union workers, suburban parents, college students",
                                    value: "{audience}",
                                    oninput: move |evt| audience.set(evt.value()),
                                }
                            }

                            // Context
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Additional Context" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Any additional context: recent news, opponent statements, venue details, etc.",
                                    rows: "3",
                                    value: "{context}",
                                    oninput: move |evt| context.set(evt.value()),
                                }
                            }

                            // Generate button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_generating.read(),
                                onclick: on_generate,
                                if *is_generating.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Generating Briefing..."
                                } else {
                                    "Generate Briefing"
                                }
                            }
                        }
                    }
                }

                // Right: Output panel
                div { class: "w-full lg:w-2/3",
                    if *is_generating.read() {
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center space-y-4",
                                    LoadingSpinner {}
                                    p { class: "text-slate-400", "Generating briefing document..." }
                                }
                            }
                        }
                    } else if let Some(ref b) = *briefing.read() {
                        div { class: "space-y-4",
                            // Briefing header
                            div { class: "glass-card gradient-border",
                                div { class: "card-body py-4",
                                    div { class: "flex items-center justify-between",
                                        div {
                                            h2 { class: "text-xl font-bold", "{b.title}" }
                                            if let Some(ref created) = b.created_at {
                                                p { class: "text-sm text-slate-500", "Generated: {created}" }
                                            }
                                        }
                                        div { class: "badge badge-primary", "{b.sections.len()} sections" }
                                    }
                                }
                            }

                            // Briefing sections as accordion/collapse
                            for (idx, section) in b.sections.iter().enumerate() {
                                {
                                    let priority_badge = match section.priority.as_str() {
                                        "high" => "badge badge-error badge-sm",
                                        "medium" => "badge badge-warning badge-sm",
                                        "low" => "badge badge-info badge-sm",
                                        _ => "badge badge-ghost badge-sm",
                                    };
                                    let priority_label = match section.priority.as_str() {
                                        "high" => "HIGH",
                                        "medium" => "MED",
                                        "low" => "LOW",
                                        other => other,
                                    };
                                    let bg_class = match section.priority.as_str() {
                                        "high" => "collapse collapse-arrow bg-error/5 border border-error/20 rounded-lg",
                                        "medium" => "collapse collapse-arrow bg-warning/5 border border-warning/20 rounded-lg",
                                        _ => "collapse collapse-arrow bg-base-100 border border-base-300 rounded-lg",
                                    };
                                    // Auto-expand high priority sections
                                    let is_high = section.priority == "high";
                                    rsx! {
                                        div { key: "{idx}", class: "{bg_class}",
                                            input {
                                                r#type: "checkbox",
                                                checked: is_high,
                                            }
                                            div { class: "collapse-title font-medium flex items-center gap-2",
                                                span { class: "{priority_badge}", "{priority_label}" }
                                                span { "{section.heading}" }
                                            }
                                            div { class: "collapse-content",
                                                div { class: "prose prose-sm max-w-none pt-2",
                                                    p { class: "whitespace-pre-wrap", "{section.content}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Empty state
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    p { class: "text-lg font-medium text-slate-500 mb-2",
                                        "No briefing generated yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Fill in the event details and click Generate to create a candidate briefing."
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
