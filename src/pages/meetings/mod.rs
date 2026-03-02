use dioxus::prelude::*;

use crate::modules::meeting_summarizer::{summarize_meeting, ActionItem, MeetingSummary};

#[component]
pub fn MeetingsPage() -> Element {
    // Form state
    let mut title = use_signal(String::new);
    let mut transcript = use_signal(String::new);
    let mut attendees_text = use_signal(String::new);

    // Processing state
    let mut is_summarizing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut summary_result = use_signal(|| Option::<MeetingSummary>::None);

    // Handle summarization
    let on_summarize = move |_| {
        let t = title().trim().to_string();
        let tr = transcript().trim().to_string();
        let att_text = attendees_text().clone();

        if t.is_empty() {
            error_msg.set(Some("Meeting title is required.".to_string()));
            return;
        }
        if tr.is_empty() {
            error_msg.set(Some("Meeting transcript is required.".to_string()));
            return;
        }

        let attendees: Vec<String> = att_text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        is_summarizing.set(true);
        error_msg.set(None);

        spawn(async move {
            match summarize_meeting(t, tr, attendees).await {
                Ok(summary) => summary_result.set(Some(summary)),
                Err(e) => error_msg.set(Some(format!("Summarization failed: {e}"))),
            }
            is_summarizing.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Meeting Summarizer" }
                p { class: "text-base-content/70",
                    "Summarize campaign meetings with AI-powered extraction of key decisions and action items."
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

            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Input Form
                div { class: "w-full lg:w-1/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Meeting Details" }

                            // Title
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Meeting Title" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "e.g., Weekly Strategy Call",
                                    value: "{title}",
                                    oninput: move |evt| title.set(evt.value()),
                                }
                            }

                            // Transcript
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Transcript" }
                                    span { class: "label-text-alt text-base-content/50", "Paste full meeting transcript" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Paste the meeting transcript or notes here...",
                                    rows: "12",
                                    value: "{transcript}",
                                    oninput: move |evt| transcript.set(evt.value()),
                                }
                            }

                            // Attendees
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Attendees" }
                                    span { class: "label-text-alt text-base-content/50", "One per line" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "John Smith\nJane Doe\nBob Johnson",
                                    rows: "4",
                                    value: "{attendees_text}",
                                    oninput: move |evt| attendees_text.set(evt.value()),
                                }
                            }

                            // Summarize button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_summarizing.read(),
                                onclick: on_summarize,
                                if *is_summarizing.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Summarizing..."
                                } else {
                                    "Summarize Meeting"
                                }
                            }
                        }
                    }
                }

                // Right: Results
                div { class: "w-full lg:w-2/3 space-y-4",
                    if let Some(summary) = summary_result() {
                        // Summary Card
                        div { class: "card bg-base-100 shadow-sm",
                            div { class: "card-body",
                                div { class: "flex items-center justify-between mb-2",
                                    h2 { class: "card-title text-lg", "{summary.title}" }
                                    span { class: "text-sm text-base-content/50", "{summary.created_at}" }
                                }

                                if !summary.attendees.is_empty() {
                                    div { class: "flex flex-wrap gap-1 mb-3",
                                        for attendee in summary.attendees.iter() {
                                            span { class: "badge badge-ghost badge-sm", "{attendee}" }
                                        }
                                    }
                                }

                                div { class: "bg-base-200 rounded-lg p-4",
                                    p { class: "text-sm whitespace-pre-wrap leading-relaxed",
                                        "{summary.summary}"
                                    }
                                }
                            }
                        }

                        // Key Decisions
                        if !summary.key_decisions.is_empty() {
                            div { class: "card bg-base-100 shadow-sm",
                                div { class: "card-body",
                                    h3 { class: "card-title text-base", "Key Decisions" }
                                    ul { class: "space-y-2 mt-2",
                                        for (idx, decision) in summary.key_decisions.iter().enumerate() {
                                            li { class: "flex items-start gap-2",
                                                span { class: "badge badge-primary badge-sm mt-0.5",
                                                    "{idx + 1}"
                                                }
                                                span { class: "text-sm", "{decision}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Action Items Table
                        if !summary.action_items.is_empty() {
                            div { class: "card bg-base-100 shadow-sm",
                                div { class: "card-body",
                                    h3 { class: "card-title text-base", "Action Items" }
                                    div { class: "overflow-x-auto mt-2",
                                        table { class: "table table-sm",
                                            thead {
                                                tr {
                                                    th { "Description" }
                                                    th { "Assignee" }
                                                    th { "Deadline" }
                                                    th { "Status" }
                                                }
                                            }
                                            tbody {
                                                for item in summary.action_items.iter() {
                                                    ActionItemRow { item: item.clone() }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // No results yet
                        div { class: "card bg-base-100 shadow-sm min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    p { class: "text-lg font-medium text-base-content/50 mb-2",
                                        "No summary yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Paste a meeting transcript and click Summarize to extract key decisions and action items."
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

/// A single row in the action items table.
#[component]
fn ActionItemRow(item: ActionItem) -> Element {
    let status_class = match item.status.as_str() {
        "completed" => "badge badge-success badge-sm",
        "in_progress" => "badge badge-info badge-sm",
        "blocked" => "badge badge-error badge-sm",
        _ => "badge badge-warning badge-sm",
    };

    let status_label = match item.status.as_str() {
        "completed" => "Completed",
        "in_progress" => "In Progress",
        "blocked" => "Blocked",
        _ => "Pending",
    };

    rsx! {
        tr {
            td { class: "text-sm", "{item.description}" }
            td {
                span { class: "badge badge-ghost badge-sm", "{item.assignee}" }
            }
            td { class: "text-sm text-base-content/70", "{item.deadline}" }
            td {
                span { class: "{status_class}", "{status_label}" }
            }
        }
    }
}
