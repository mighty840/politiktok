use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::campaign_copy::{generate_copy, regenerate_variant, CopyJob, GeneratedCopy};

/// Human-readable label for a format key.
fn format_label(format: &str) -> &str {
    match format {
        "email" => "Email",
        "social_post" => "Social Post",
        "press_release" => "Press Release",
        "speech" => "Speech",
        "blog_post" => "Blog Post",
        _ => format,
    }
}

/// All available format options for the checkbox group.
const FORMAT_OPTIONS: &[(&str, &str)] = &[
    ("email", "Email"),
    ("social_post", "Social Post"),
    ("press_release", "Press Release"),
    ("speech", "Speech"),
    ("blog_post", "Blog Post"),
];

/// Available audience options.
const AUDIENCE_OPTIONS: &[&str] = &[
    "General Public",
    "Young Voters (18-30)",
    "Senior Citizens",
    "Suburban Families",
    "Rural Communities",
    "Urban Professionals",
];

/// Available tone options.
const TONE_OPTIONS: &[&str] = &[
    "Professional",
    "Casual",
    "Urgent",
    "Inspirational",
    "Empathetic",
];

#[component]
pub fn CampaignCopyPage() -> Element {
    // Form state
    let mut topic = use_signal(String::new);
    let mut key_messages_text = use_signal(String::new);
    let mut audience = use_signal(|| "General Public".to_string());
    let mut tone = use_signal(|| "Professional".to_string());
    let mut selected_formats =
        use_signal(|| vec!["email".to_string(), "social_post".to_string()]);

    // Generation state
    let mut is_generating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut copy_job = use_signal(|| Option::<CopyJob>::None);

    // Output tab state
    let mut active_tab = use_signal(|| 0_usize);

    // Regeneration state (per-tab feedback)
    let mut regen_feedback = use_signal(String::new);
    let mut is_regenerating = use_signal(|| false);

    // Toggle a format in the selected list
    let mut toggle_format = move |format: String| {
        let mut formats = selected_formats.write();
        if let Some(pos) = formats.iter().position(|f| f == &format) {
            formats.remove(pos);
        } else {
            formats.push(format);
        }
    };

    // Handle generate
    let on_generate = move |_| {
        let topic_val = topic().trim().to_string();
        let messages_text = key_messages_text().clone();
        let audience_val = audience().clone();
        let tone_val = tone().clone();
        let formats_val = selected_formats().clone();

        if topic_val.is_empty() {
            error_msg.set(Some("Topic is required.".to_string()));
            return;
        }

        let key_msgs: Vec<String> = messages_text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if key_msgs.is_empty() {
            error_msg.set(Some("At least one key message is required.".to_string()));
            return;
        }

        if formats_val.is_empty() {
            error_msg.set(Some("Select at least one format.".to_string()));
            return;
        }

        is_generating.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_copy(
                topic_val,
                key_msgs,
                audience_val,
                tone_val,
                formats_val,
                None,
            )
            .await
            {
                Ok(job) => {
                    copy_job.set(Some(job));
                    active_tab.set(0);
                    regen_feedback.set(String::new());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Generation failed: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    // Handle regenerate for the active tab
    let on_regenerate = move |_| {
        let job = match copy_job() {
            Some(j) => j,
            None => return,
        };

        let tab_idx = active_tab();
        let result = match job.results.get(tab_idx) {
            Some(r) => r.clone(),
            None => return,
        };

        let feedback_val = regen_feedback().trim().to_string();
        let feedback = if feedback_val.is_empty() {
            None
        } else {
            Some(feedback_val)
        };

        let req = job.request.clone();

        is_regenerating.set(true);
        error_msg.set(None);

        spawn(async move {
            match regenerate_variant(
                result.format.clone(),
                req.topic,
                req.key_messages,
                req.audience,
                req.tone,
                feedback,
            )
            .await
            {
                Ok(new_copy) => {
                    // Replace the result at the active tab index
                    let mut job_val = copy_job().unwrap();
                    if let Some(entry) = job_val.results.get_mut(tab_idx) {
                        *entry = new_copy;
                    }
                    copy_job.set(Some(job_val));
                    regen_feedback.set(String::new());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Regeneration failed: {e}")));
                }
            }
            is_regenerating.set(false);
        });
    };

    // Copy to clipboard handler
    let on_copy_clipboard = move |content: String| {
        spawn(async move {
            let js = format!(
                "navigator.clipboard.writeText({}).catch(err => console.error('Copy failed:', err))",
                serde_json::to_string(&content).unwrap_or_default()
            );
            let _ = document::eval(&js).await;
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Campaign Copy Generator" }
                p { class: "text-base-content/70",
                    "Generate targeted campaign messaging with AI-assisted content creation for emails, social posts, press releases, speeches, and blog posts."
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

            // Main layout: two-column on desktop, stacked on mobile
            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Input Form
                div { class: "w-full lg:w-1/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Input" }

                            // Topic
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Topic" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "e.g., Infrastructure investment plan",
                                    value: "{topic}",
                                    oninput: move |evt| topic.set(evt.value()),
                                }
                            }

                            // Key Messages
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Key Messages" }
                                    span { class: "label-text-alt text-base-content/50", "One per line" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Enter key messages, one per line...",
                                    rows: "4",
                                    value: "{key_messages_text}",
                                    oninput: move |evt| key_messages_text.set(evt.value()),
                                }
                            }

                            // Target Audience
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Target Audience" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{audience}",
                                    onchange: move |evt: Event<FormData>| audience.set(evt.value()),
                                    for option in AUDIENCE_OPTIONS {
                                        option { value: "{option}", "{option}" }
                                    }
                                }
                            }

                            // Tone
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Tone" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{tone}",
                                    onchange: move |evt: Event<FormData>| tone.set(evt.value()),
                                    for option in TONE_OPTIONS {
                                        option { value: "{option}", "{option}" }
                                    }
                                }
                            }

                            // Formats (checkbox group)
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Formats" }
                                }
                                div { class: "space-y-2",
                                    for (value, label) in FORMAT_OPTIONS.iter() {
                                        {
                                            let val = value.to_string();
                                            let is_checked = selected_formats().contains(&val);
                                            let val_for_toggle = val.clone();
                                            rsx! {
                                                label { class: "flex items-center gap-2 cursor-pointer",
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "checkbox checkbox-primary checkbox-sm",
                                                        checked: is_checked,
                                                        onchange: move |_| toggle_format(val_for_toggle.clone()),
                                                    }
                                                    span { class: "label-text", "{label}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Generate button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_generating.read(),
                                onclick: on_generate,
                                if *is_generating.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Generating..."
                                } else {
                                    "Generate Copy"
                                }
                            }
                        }
                    }
                }

                // Right: Output Panel
                div { class: "w-full lg:w-2/3",
                    div { class: "card bg-base-100 shadow-sm min-h-[400px]",
                        div { class: "card-body",
                            h2 { class: "card-title text-lg mb-2", "Generated Copy" }

                            if *is_generating.read() {
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center space-y-4",
                                        LoadingSpinner {}
                                        p { class: "text-base-content/60", "Generating campaign copy..." }
                                    }
                                }
                            } else if let Some(job) = copy_job() {
                                if job.results.is_empty() {
                                    div { class: "flex-1 flex items-center justify-center py-12",
                                        p { class: "text-base-content/50", "No results were generated." }
                                    }
                                } else {
                                    // Tabs
                                    div { class: "tabs tabs-bordered mb-4",
                                        for (idx, result) in job.results.iter().enumerate() {
                                            {
                                                let is_active = active_tab() == idx;
                                                let tab_class = if is_active {
                                                    "tab tab-active"
                                                } else {
                                                    "tab"
                                                };
                                                rsx! {
                                                    button {
                                                        class: "{tab_class}",
                                                        onclick: move |_| {
                                                            active_tab.set(idx);
                                                            regen_feedback.set(String::new());
                                                        },
                                                        "{format_label(&result.format)}"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Active tab content
                                    {
                                        let tab_idx = active_tab();
                                        if let Some(result) = job.results.get(tab_idx) {
                                            let content_for_copy = result.content.clone();
                                            rsx! {
                                                OutputCard {
                                                    result: result.clone(),
                                                    feedback: regen_feedback(),
                                                    is_regenerating: *is_regenerating.read(),
                                                    on_feedback_change: move |val: String| regen_feedback.set(val),
                                                    on_regenerate: on_regenerate,
                                                    on_copy: move |_| on_copy_clipboard(content_for_copy.clone()),
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                p { class: "text-base-content/50", "Select a tab to view generated content." }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // No results yet
                                div { class: "flex-1 flex items-center justify-center py-12",
                                    div { class: "text-center",
                                        div { class: "text-6xl mb-4 opacity-20", "📝" }
                                        p { class: "text-lg font-medium text-base-content/50 mb-2",
                                            "No results yet"
                                        }
                                        p { class: "text-sm text-base-content/40",
                                            "Fill in the form and click Generate to create campaign copy."
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

/// Card displaying a single generated copy result with regenerate and copy controls.
#[component]
fn OutputCard(
    result: GeneratedCopy,
    feedback: String,
    is_regenerating: bool,
    on_feedback_change: EventHandler<String>,
    on_regenerate: EventHandler<MouseEvent>,
    on_copy: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "space-y-4",
            // Content display
            div { class: "bg-base-200 rounded-lg p-4 overflow-auto max-h-[500px]",
                pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                    "{result.content}"
                }
            }

            // Action buttons
            div { class: "flex flex-wrap gap-2",
                button {
                    class: "btn btn-outline btn-sm",
                    onclick: move |evt| on_copy.call(evt),
                    "Copy to Clipboard"
                }
            }

            // Regenerate section
            div { class: "border-t border-base-300 pt-4 space-y-2",
                label { class: "label",
                    span { class: "label-text text-sm font-medium", "Feedback for regeneration (optional)" }
                }
                textarea {
                    class: "textarea textarea-bordered w-full textarea-sm",
                    placeholder: "e.g., Make it more concise, add a stronger call to action...",
                    rows: "2",
                    value: "{feedback}",
                    disabled: is_regenerating,
                    oninput: move |evt| on_feedback_change.call(evt.value()),
                }
                button {
                    class: "btn btn-secondary btn-sm",
                    disabled: is_regenerating,
                    onclick: move |evt| on_regenerate.call(evt),
                    if is_regenerating {
                        span { class: "loading loading-spinner loading-xs" }
                        "Regenerating..."
                    } else {
                        "Regenerate"
                    }
                }
            }
        }
    }
}
