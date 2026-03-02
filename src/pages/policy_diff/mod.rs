use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::policy_diff::{diff_policies, DiffChange, PolicyDiff};

/// Badge class for a significance level.
fn significance_badge(significance: &str) -> &'static str {
    match significance {
        "high" => "badge badge-error",
        "medium" => "badge badge-warning",
        "low" => "badge badge-info",
        _ => "badge badge-ghost",
    }
}

/// Badge class for a change type.
fn change_type_badge(change_type: &str) -> &'static str {
    match change_type {
        "added" => "badge badge-success",
        "removed" => "badge badge-error",
        "modified" => "badge badge-warning",
        "reworded" => "badge badge-info",
        _ => "badge badge-ghost",
    }
}

#[component]
pub fn PolicyDiffPage() -> Element {
    let mut doc_a_title = use_signal(|| "Document A".to_string());
    let mut doc_a_text = use_signal(String::new);
    let mut doc_b_title = use_signal(|| "Document B".to_string());
    let mut doc_b_text = use_signal(String::new);

    let mut is_comparing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut diff_result = use_signal(|| Option::<PolicyDiff>::None);

    let on_compare = move |_| {
        let a_title = doc_a_title().trim().to_string();
        let a_text = doc_a_text().trim().to_string();
        let b_title = doc_b_title().trim().to_string();
        let b_text = doc_b_text().trim().to_string();

        if a_text.is_empty() || b_text.is_empty() {
            error_msg.set(Some("Both documents must have content.".to_string()));
            return;
        }

        is_comparing.set(true);
        error_msg.set(None);

        spawn(async move {
            match diff_policies(a_text, a_title, b_text, b_title).await {
                Ok(result) => {
                    diff_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Comparison failed: {e}")));
                }
            }
            is_comparing.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Header
            div {
                h1 { class: "text-3xl font-bold", "Policy Diff" }
                p { class: "text-base-content/70",
                    "Compare two policy documents side-by-side to identify changes, additions, and removals with significance ratings."
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

            // Input: two side-by-side textareas
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
                // Document A
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-3",
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Document A Title" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "text",
                                placeholder: "e.g., Original Healthcare Bill",
                                value: "{doc_a_title}",
                                oninput: move |evt| doc_a_title.set(evt.value()),
                            }
                        }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Document A Text" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Paste the first policy document here...",
                                rows: "12",
                                value: "{doc_a_text}",
                                oninput: move |evt| doc_a_text.set(evt.value()),
                            }
                        }
                    }
                }

                // Document B
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-3",
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Document B Title" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "text",
                                placeholder: "e.g., Amended Healthcare Bill",
                                value: "{doc_b_title}",
                                oninput: move |evt| doc_b_title.set(evt.value()),
                            }
                        }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Document B Text" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Paste the second policy document here...",
                                rows: "12",
                                value: "{doc_b_text}",
                                oninput: move |evt| doc_b_text.set(evt.value()),
                            }
                        }
                    }
                }
            }

            // Compare button
            div { class: "flex justify-center",
                button {
                    class: "btn btn-primary btn-wide",
                    disabled: *is_comparing.read(),
                    onclick: on_compare,
                    if *is_comparing.read() {
                        span { class: "loading loading-spinner loading-sm" }
                        "Comparing..."
                    } else {
                        "Compare Documents"
                    }
                }
            }

            // Results
            if *is_comparing.read() {
                div { class: "flex items-center justify-center py-12",
                    div { class: "text-center space-y-4",
                        LoadingSpinner {}
                        p { class: "text-base-content/60", "Analyzing policy differences..." }
                    }
                }
            } else if let Some(diff) = diff_result() {
                // Summary card
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        h2 { class: "card-title text-lg",
                            "Summary: {diff.doc_a_title} vs {diff.doc_b_title}"
                        }
                        p { class: "text-base-content/80", "{diff.summary}" }
                        div { class: "flex gap-2 mt-2",
                            div { class: "badge badge-outline", "{diff.changes.len()} changes found" }
                            div { class: "badge badge-ghost", "Analyzed {diff.created_at}" }
                        }
                    }
                }

                // Changes list
                div { class: "space-y-3",
                    h3 { class: "text-xl font-semibold", "Changes" }
                    for (idx, change) in diff.changes.iter().enumerate() {
                        DiffChangeCard { index: idx, change: change.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn DiffChangeCard(index: usize, change: DiffChange) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-sm",
            div { class: "card-body",
                div { class: "flex items-center gap-2 mb-2",
                    span { class: "font-medium text-base-content/80", "#{index + 1}" }
                    span { class: "font-semibold", "{change.section}" }
                    span { class: change_type_badge(&change.change_type), "{change.change_type}" }
                    span { class: significance_badge(&change.significance),
                        "{change.significance} significance"
                    }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                    if !change.old_text.is_empty() {
                        div { class: "bg-error/10 rounded-lg p-3 border border-error/20",
                            p { class: "text-xs font-medium text-error mb-1", "Document A" }
                            p { class: "text-sm", "{change.old_text}" }
                        }
                    }
                    if !change.new_text.is_empty() {
                        div { class: "bg-success/10 rounded-lg p-3 border border-success/20",
                            p { class: "text-xs font-medium text-success mb-1", "Document B" }
                            p { class: "text-sm", "{change.new_text}" }
                        }
                    }
                }
            }
        }
    }
}
