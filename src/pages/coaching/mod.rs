use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::coaching::{
    get_coaching_feedback, respond_to_coaching, start_coaching_session, CoachingFeedback,
    CoachingMessage, CoachingSession,
};

/// Available coaching modes.
const MODE_OPTIONS: &[(&str, &str)] = &[
    ("journalist", "Press Interview"),
    ("debate", "Debate Opponent"),
    ("townhall", "Town Hall Q&A"),
];

/// Available difficulty levels.
const DIFFICULTY_OPTIONS: &[(&str, &str)] = &[
    ("easy", "Easy"),
    ("medium", "Medium"),
    ("hard", "Hard"),
];

#[component]
pub fn CoachingPage() -> Element {
    // Session state
    let mut session = use_signal(|| Option::<CoachingSession>::None);
    let mut feedback = use_signal(|| Option::<CoachingFeedback>::None);

    // Setup form state
    let mut mode = use_signal(|| "journalist".to_string());
    let mut topics_text = use_signal(String::new);
    let mut difficulty = use_signal(|| "medium".to_string());

    // Interaction state
    let mut input_text = use_signal(String::new);
    let mut is_starting = use_signal(|| false);
    let mut is_responding = use_signal(|| false);
    let mut is_getting_feedback = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    // Start a new coaching session
    let on_start = move |_| {
        let mode_val = mode().clone();
        let difficulty_val = difficulty().clone();
        let topics: Vec<String> = topics_text()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        is_starting.set(true);
        error_msg.set(None);
        feedback.set(None);

        spawn(async move {
            match start_coaching_session(mode_val, topics, difficulty_val).await {
                Ok(new_session) => {
                    session.set(Some(new_session));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to start session: {e}")));
                }
            }
            is_starting.set(false);
        });
    };

    // Send a candidate response
    let mut on_respond = move |_: ()| {
        let response_text = input_text().trim().to_string();
        if response_text.is_empty() || *is_responding.read() {
            return;
        }

        let current_session = match session() {
            Some(s) => s,
            None => return,
        };

        input_text.set(String::new());
        is_responding.set(true);
        error_msg.set(None);

        // Optimistically add candidate message
        {
            let mut s = session.write();
            if let Some(ref mut sess) = *s {
                sess.messages.push(CoachingMessage {
                    role: "candidate".to_string(),
                    content: response_text.clone(),
                });
            }
        }

        spawn(async move {
            match respond_to_coaching(
                current_session.id.clone(),
                current_session.mode.clone(),
                current_session.topics.clone(),
                current_session.difficulty.clone(),
                current_session.messages.clone(),
                response_text,
            )
            .await
            {
                Ok(updated_messages) => {
                    let mut s = session.write();
                    if let Some(ref mut sess) = *s {
                        sess.messages = updated_messages;
                    }
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to get response: {e}")));
                }
            }
            is_responding.set(false);
        });
    };

    // Get feedback on the session
    let on_get_feedback = move |_| {
        let current_session = match session() {
            Some(s) => s,
            None => return,
        };

        is_getting_feedback.set(true);
        error_msg.set(None);

        spawn(async move {
            match get_coaching_feedback(
                current_session.messages.clone(),
                current_session.mode.clone(),
            )
            .await
            {
                Ok(fb) => {
                    feedback.set(Some(fb));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to get feedback: {e}")));
                }
            }
            is_getting_feedback.set(false);
        });
    };

    // Reset to setup form
    let on_new_session = move |_| {
        session.set(None);
        feedback.set(None);
        input_text.set(String::new());
        error_msg.set(None);
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold", "Coaching & Debate Prep" }
                    p { class: "text-slate-400",
                        "AI-powered debate preparation with simulated opponents, response coaching, and performance feedback."
                    }
                }
                if session().is_some() {
                    button {
                        class: "btn btn-outline btn-sm",
                        onclick: on_new_session,
                        "New Session"
                    }
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

            if session().is_none() {
                // Setup form
                div { class: "max-w-2xl mx-auto",
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Session Setup" }

                            // Mode selector
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Coaching Mode" }
                                }
                                div { class: "grid grid-cols-3 gap-2",
                                    for (value, label) in MODE_OPTIONS.iter() {
                                        {
                                            let val = value.to_string();
                                            let is_selected = mode() == val;
                                            let btn_class = if is_selected {
                                                "btn btn-primary btn-sm"
                                            } else {
                                                "btn btn-outline btn-sm"
                                            };
                                            let val_clone = val.clone();
                                            rsx! {
                                                button {
                                                    class: "{btn_class}",
                                                    r#type: "button",
                                                    onclick: move |_| mode.set(val_clone.clone()),
                                                    "{label}"
                                                }
                                            }
                                        }
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
                                    placeholder: "e.g., Healthcare reform\nClimate policy\nEconomic growth",
                                    rows: "4",
                                    value: "{topics_text}",
                                    oninput: move |evt| topics_text.set(evt.value()),
                                }
                            }

                            // Difficulty
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Difficulty" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{difficulty}",
                                    onchange: move |evt: Event<FormData>| difficulty.set(evt.value()),
                                    for (value, label) in DIFFICULTY_OPTIONS.iter() {
                                        option { value: "{value}", "{label}" }
                                    }
                                }
                            }

                            // Start button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_starting.read(),
                                onclick: on_start,
                                if *is_starting.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Starting Session..."
                                } else {
                                    "Start Session"
                                }
                            }
                        }
                    }
                }
            } else {
                // Active session
                div { class: "flex flex-col lg:flex-row gap-6",
                    // Chat area
                    div { class: if feedback().is_some() { "w-full lg:w-1/2" } else { "w-full lg:w-2/3 mx-auto" },
                        div { class: "glass-card gradient-border",
                            div { class: "card-body p-0",
                                // Session info bar
                                if let Some(ref sess) = *session.read() {
                                    div { class: "flex items-center gap-2 px-4 py-2 border-b border-base-300 bg-slate-800/30 rounded-t-2xl",
                                        span { class: "badge badge-primary badge-sm",
                                            {match sess.mode.as_str() {
                                                "journalist" => "Press Interview",
                                                "debate" => "Debate",
                                                "townhall" => "Town Hall",
                                                _ => &sess.mode,
                                            }}
                                        }
                                        span { class: "badge badge-outline badge-sm",
                                            {match sess.difficulty.as_str() {
                                                "easy" => "Easy",
                                                "medium" => "Medium",
                                                "hard" => "Hard",
                                                _ => &sess.difficulty,
                                            }}
                                        }
                                        if !sess.topics.is_empty() {
                                            span { class: "text-xs text-slate-500 truncate",
                                                {sess.topics.join(", ")}
                                            }
                                        }
                                    }
                                }

                                // Messages
                                div { class: "p-4 space-y-4 overflow-y-auto max-h-[500px]",
                                    if let Some(ref sess) = *session.read() {
                                        for (idx, msg) in sess.messages.iter().enumerate() {
                                            {
                                                let is_candidate = msg.role == "candidate";
                                                let chat_class = if is_candidate {
                                                    "chat chat-end"
                                                } else {
                                                    "chat chat-start"
                                                };
                                                let bubble_class = if is_candidate {
                                                    "chat-bubble chat-bubble-primary"
                                                } else {
                                                    "chat-bubble chat-bubble-secondary"
                                                };
                                                let label = if is_candidate {
                                                    "You (Candidate)"
                                                } else {
                                                    match sess.mode.as_str() {
                                                        "journalist" => "Journalist",
                                                        "debate" => "Debate Opponent",
                                                        "townhall" => "Voter",
                                                        _ => "Interviewer",
                                                    }
                                                };
                                                rsx! {
                                                    div { key: "{idx}", class: "{chat_class}",
                                                        div { class: "chat-header text-xs text-slate-500 mb-1",
                                                            "{label}"
                                                        }
                                                        div { class: "{bubble_class}",
                                                            p { "{msg.content}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Loading indicator for AI response
                                    if *is_responding.read() {
                                        div { class: "chat chat-start",
                                            div { class: "chat-bubble chat-bubble-secondary",
                                                span { class: "loading loading-dots loading-sm" }
                                            }
                                        }
                                    }
                                }

                                // Input area
                                if feedback().is_none() {
                                    div { class: "border-t border-base-300 p-4",
                                        form {
                                            class: "flex gap-2",
                                            onsubmit: move |evt| {
                                                evt.prevent_default();
                                                on_respond(());
                                            },
                                            input {
                                                class: "input input-bordered flex-1",
                                                r#type: "text",
                                                placeholder: "Type your response...",
                                                value: "{input_text}",
                                                disabled: *is_responding.read(),
                                                oninput: move |evt| input_text.set(evt.value()),
                                            }
                                            button {
                                                class: "btn btn-primary",
                                                r#type: "submit",
                                                disabled: *is_responding.read() || input_text().trim().is_empty(),
                                                if *is_responding.read() {
                                                    span { class: "loading loading-spinner loading-sm" }
                                                } else {
                                                    "Send"
                                                }
                                            }
                                        }

                                        // End session button
                                        div { class: "mt-3 text-center",
                                            button {
                                                class: "btn btn-warning btn-sm",
                                                disabled: *is_getting_feedback.read() || *is_responding.read(),
                                                onclick: on_get_feedback,
                                                if *is_getting_feedback.read() {
                                                    span { class: "loading loading-spinner loading-sm" }
                                                    "Analyzing Performance..."
                                                } else {
                                                    "End & Get Feedback"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Feedback panel
                    if let Some(ref fb) = *feedback.read() {
                        div { class: "w-full lg:w-1/2",
                            FeedbackCard { feedback: fb.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Displays the coaching feedback report as a card.
#[component]
fn FeedbackCard(feedback: CoachingFeedback) -> Element {
    let score = feedback.overall_score;
    let score_color = if score >= 80.0 {
        "text-success"
    } else if score >= 60.0 {
        "text-warning"
    } else {
        "text-error"
    };

    let progress_color = if score >= 80.0 {
        "progress-success"
    } else if score >= 60.0 {
        "progress-warning"
    } else {
        "progress-error"
    };

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "card-body space-y-4",
                h2 { class: "card-title text-lg", "Performance Report" }

                // Overall score
                div { class: "text-center py-4",
                    div { class: "text-5xl font-bold {score_color}",
                        "{score:.0}"
                    }
                    p { class: "text-sm text-slate-400 mt-1", "out of 100" }
                    progress {
                        class: "progress {progress_color} w-56 mx-auto mt-2",
                        value: "{score}",
                        max: "100",
                    }
                }

                // Strengths
                div { class: "collapse collapse-arrow bg-success/10 rounded-lg",
                    input { r#type: "checkbox", checked: true }
                    div { class: "collapse-title font-medium text-success",
                        "Strengths ({feedback.strengths.len()})"
                    }
                    div { class: "collapse-content",
                        ul { class: "list-disc list-inside space-y-1 text-sm",
                            for item in feedback.strengths.iter() {
                                li { "{item}" }
                            }
                        }
                    }
                }

                // Areas for improvement
                div { class: "collapse collapse-arrow bg-warning/10 rounded-lg",
                    input { r#type: "checkbox", checked: true }
                    div { class: "collapse-title font-medium text-warning",
                        "Areas for Improvement ({feedback.areas_for_improvement.len()})"
                    }
                    div { class: "collapse-content",
                        ul { class: "list-disc list-inside space-y-1 text-sm",
                            for item in feedback.areas_for_improvement.iter() {
                                li { "{item}" }
                            }
                        }
                    }
                }

                // Specific feedback
                div { class: "collapse collapse-arrow bg-info/10 rounded-lg",
                    input { r#type: "checkbox", checked: true }
                    div { class: "collapse-title font-medium text-info",
                        "Specific Feedback ({feedback.specific_feedback.len()})"
                    }
                    div { class: "collapse-content",
                        ul { class: "list-disc list-inside space-y-1 text-sm",
                            for item in feedback.specific_feedback.iter() {
                                li { "{item}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
