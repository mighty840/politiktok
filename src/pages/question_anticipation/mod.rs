use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::question_anticipation::{
    anticipate_questions, generate_preparation_checklist, AnticipatedQuestion, QuestionReport,
};

/// Event type options for the dropdown.
const EVENT_TYPES: &[&str] = &[
    "Town Hall",
    "Press Conference",
    "Debate",
    "Radio Interview",
    "Door-to-door",
];

/// Badge class for a likelihood level.
fn likelihood_badge_class(likelihood: &str) -> &'static str {
    match likelihood {
        "high" => "badge badge-error",
        "medium" => "badge badge-warning",
        "low" => "badge badge-info",
        _ => "badge",
    }
}

#[component]
pub fn QuestionAnticipationPage() -> Element {
    // Form state
    let mut context = use_signal(String::new);
    let mut event_type = use_signal(|| "Town Hall".to_string());
    let mut hot_topics = use_signal(String::new);

    // Generation state
    let mut is_generating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut report = use_signal(|| Option::<QuestionReport>::None);

    // Checklist state
    let mut is_generating_checklist = use_signal(|| false);
    let mut checklist = use_signal(|| Option::<String>::None);

    // Collapsible answer state: track which questions have expanded answers
    let mut expanded = use_signal(|| Vec::<usize>::new());

    let mut toggle_expanded = move |idx: usize| {
        let mut items = expanded.write();
        if let Some(pos) = items.iter().position(|&i| i == idx) {
            items.remove(pos);
        } else {
            items.push(idx);
        }
    };

    // Handle generate questions
    let on_generate = move |_| {
        let context_val = context().trim().to_string();
        let event_type_val = event_type().clone();
        let hot_topics_val = hot_topics().clone();

        if context_val.is_empty() {
            error_msg.set(Some("Please provide context for the event.".to_string()));
            return;
        }

        is_generating.set(true);
        error_msg.set(None);
        checklist.set(None);

        spawn(async move {
            match anticipate_questions(context_val, event_type_val, hot_topics_val).await {
                Ok(result) => {
                    report.set(Some(result));
                    expanded.set(Vec::new());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Generation failed: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    // Handle generate checklist
    let on_generate_checklist = move |_| {
        let questions = match report() {
            Some(r) => r.questions,
            None => return,
        };

        is_generating_checklist.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_preparation_checklist(questions).await {
                Ok(result) => {
                    checklist.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Checklist generation failed: {e}")));
                }
            }
            is_generating_checklist.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Question Anticipation" }
                p { class: "text-base-content/70",
                    "Predict likely voter questions for upcoming events and prepare responses with AI-generated suggested answers."
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

                // Left: Input Form
                div { class: "w-full lg:w-1/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Event Details" }

                            // Context
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Context" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Describe the event context, your positions, recent news, constituency concerns...",
                                    rows: "6",
                                    value: "{context}",
                                    oninput: move |evt| context.set(evt.value()),
                                }
                            }

                            // Event Type
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Event Type" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{event_type}",
                                    onchange: move |evt: Event<FormData>| event_type.set(evt.value()),
                                    for option in EVENT_TYPES {
                                        option { value: "{option}", "{option}" }
                                    }
                                }
                            }

                            // Hot Topics
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Hot Topics" }
                                    span { class: "label-text-alt text-base-content/50", "Optional" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "List current hot topics, one per line...\n\nExample:\nInflation and cost of living\nSchool funding cuts\nLocal road repairs",
                                    rows: "4",
                                    value: "{hot_topics}",
                                    oninput: move |evt| hot_topics.set(evt.value()),
                                }
                            }

                            // Generate button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_generating.read(),
                                onclick: on_generate,
                                if *is_generating.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Generating Questions..."
                                } else {
                                    "Generate Questions"
                                }
                            }
                        }
                    }
                }

                // Right: Results
                div { class: "w-full lg:w-2/3 space-y-6",
                    // Questions card
                    div { class: "card bg-base-100 shadow-sm min-h-[400px]",
                        div { class: "card-body",
                            h2 { class: "card-title text-lg mb-2", "Anticipated Questions" }

                            if *is_generating.read() {
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center space-y-4",
                                        LoadingSpinner {}
                                        p { class: "text-base-content/60", "Generating anticipated questions..." }
                                    }
                                }
                            } else if let Some(rpt) = report() {
                                if rpt.questions.is_empty() {
                                    div { class: "flex-1 flex items-center justify-center py-12",
                                        p { class: "text-base-content/50", "No questions were generated." }
                                    }
                                } else {
                                    div { class: "space-y-4",
                                        for (idx, q) in rpt.questions.iter().enumerate() {
                                            {
                                                let is_open = expanded().contains(&idx);
                                                let q_clone = q.clone();
                                                rsx! {
                                                    div { class: "border border-base-300 rounded-lg",
                                                        // Question header
                                                        div {
                                                            class: "flex items-start gap-3 p-4 cursor-pointer hover:bg-base-200/50 transition-colors",
                                                            onclick: move |_| toggle_expanded(idx),

                                                            // Number
                                                            span { class: "font-bold text-primary min-w-[2rem] text-center",
                                                                "{idx + 1}."
                                                            }

                                                            // Question text and badges
                                                            div { class: "flex-1",
                                                                p { class: "font-medium text-base-content",
                                                                    "{q_clone.question}"
                                                                }
                                                                div { class: "flex flex-wrap gap-2 mt-2",
                                                                    span {
                                                                        class: likelihood_badge_class(&q_clone.likelihood),
                                                                        "{q_clone.likelihood}"
                                                                    }
                                                                    span { class: "badge badge-ghost badge-sm",
                                                                        "{q_clone.topic}"
                                                                    }
                                                                }
                                                            }

                                                            // Expand indicator
                                                            span { class: "text-base-content/40 text-sm mt-1",
                                                                if is_open { "▲" } else { "▼" }
                                                            }
                                                        }

                                                        // Collapsible content
                                                        if is_open {
                                                            div { class: "border-t border-base-300 p-4 space-y-3 bg-base-200/30",
                                                                div {
                                                                    h4 { class: "font-semibold text-sm text-primary mb-1",
                                                                        "Suggested Answer"
                                                                    }
                                                                    p { class: "text-base-content/80 text-sm",
                                                                        "{q_clone.suggested_answer}"
                                                                    }
                                                                }
                                                                div {
                                                                    h4 { class: "font-semibold text-sm text-secondary mb-1",
                                                                        "Preparation Notes"
                                                                    }
                                                                    p { class: "text-base-content/70 text-sm italic",
                                                                        "{q_clone.preparation_notes}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Generate checklist button
                                        div { class: "pt-4 border-t border-base-300",
                                            button {
                                                class: "btn btn-secondary w-full",
                                                disabled: *is_generating_checklist.read(),
                                                onclick: on_generate_checklist,
                                                if *is_generating_checklist.read() {
                                                    span { class: "loading loading-spinner loading-sm" }
                                                    "Generating Checklist..."
                                                } else {
                                                    "Generate Preparation Checklist"
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center",
                                        p { class: "text-lg font-medium text-base-content/50 mb-2",
                                            "No questions yet"
                                        }
                                        p { class: "text-sm text-base-content/40",
                                            "Fill in the event details and click Generate Questions to get started."
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Checklist card (shown when generated)
                    if let Some(cl) = checklist() {
                        div { class: "card bg-base-100 shadow-sm",
                            div { class: "card-body",
                                h2 { class: "card-title text-lg mb-2", "Preparation Checklist" }
                                div { class: "bg-base-200 rounded-lg p-4 overflow-auto max-h-[400px]",
                                    pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                        "{cl}"
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
