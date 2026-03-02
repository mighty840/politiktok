use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::canvassing::{
    export_script_text, generate_script, CanvassingScript, ScriptSection,
};

/// Voter segment options for the dropdown.
const VOTER_SEGMENTS: &[&str] = &[
    "Undecided Voters",
    "Likely Supporters",
    "Swing Voters",
    "First-Time Voters",
    "Senior Voters",
    "Young Professionals",
];

#[component]
pub fn CanvassingPage() -> Element {
    // Form state
    let mut voter_segment = use_signal(|| VOTER_SEGMENTS[0].to_string());
    let mut local_issues = use_signal(String::new);
    let mut candidate_name = use_signal(String::new);
    let mut key_asks = use_signal(String::new);
    let mut is_generating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    // Generated script
    let mut generated_script = use_signal(|| Option::<CanvassingScript>::None);
    let mut copy_status = use_signal(|| Option::<String>::None);

    // Handle script generation
    let on_generate = move |_| {
        let segment = voter_segment().trim().to_string();
        let name = candidate_name().trim().to_string();
        let issues: Vec<String> = local_issues()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        let asks: Vec<String> = key_asks()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if name.is_empty() {
            error_msg.set(Some("Candidate name is required.".to_string()));
            return;
        }
        if issues.is_empty() {
            error_msg.set(Some("At least one local issue is required.".to_string()));
            return;
        }

        is_generating.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_script(segment, issues, name, asks).await {
                Ok(script) => {
                    generated_script.set(Some(script));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to generate script: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    // Handle copy full script
    let on_copy = move |_| {
        if let Some(script) = generated_script() {
            spawn(async move {
                match export_script_text(script).await {
                    Ok(text) => {
                        let escaped = text
                            .replace('\\', "\\\\")
                            .replace('`', "\\`")
                            .replace('$', "\\$");
                        dioxus::prelude::document::eval(&format!(
                            "navigator.clipboard.writeText(`{escaped}`).then(() => {{}}).catch(() => {{}});"
                        ));
                        copy_status.set(Some("Copied to clipboard!".to_string()));
                    }
                    Err(e) => {
                        copy_status.set(Some(format!("Copy failed: {e}")));
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Canvassing Script Generator" }
                p { class: "text-slate-400",
                    "Generate dynamic door-to-door canvassing scripts tailored to voter segments and local issues."
                }
            }

            // Script generation form
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    h2 { class: "card-title text-lg mb-4", "Generate New Script" }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        // Voter Segment
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Voter Segment" }
                            }
                            select {
                                class: "select select-bordered w-full",
                                value: "{voter_segment}",
                                onchange: move |evt: Event<FormData>| {
                                    voter_segment.set(evt.value());
                                },
                                for segment in VOTER_SEGMENTS {
                                    option { value: "{segment}", "{segment}" }
                                }
                            }
                        }

                        // Candidate Name
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Candidate Name" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "text",
                                placeholder: "e.g. Jane Smith",
                                value: "{candidate_name}",
                                oninput: move |evt| candidate_name.set(evt.value()),
                            }
                        }

                        // Local Issues
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Local Issues" }
                                span { class: "label-text-alt text-slate-500", "One per line" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                rows: "4",
                                placeholder: "Road infrastructure\nSchool funding\nPublic safety",
                                value: "{local_issues}",
                                oninput: move |evt| local_issues.set(evt.value()),
                            }
                        }

                        // Key Asks
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Key Asks" }
                                span { class: "label-text-alt text-slate-500", "One per line" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                rows: "4",
                                placeholder: "Vote on election day\nSign up to volunteer\nAttend town hall",
                                value: "{key_asks}",
                                oninput: move |evt| key_asks.set(evt.value()),
                            }
                        }
                    }

                    // Error message
                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error mt-4",
                            span { "{err}" }
                        }
                    }

                    // Generate button
                    div { class: "mt-4",
                        button {
                            class: "btn btn-primary",
                            disabled: *is_generating.read(),
                            onclick: on_generate,
                            if *is_generating.read() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Generating Script..."
                            } else {
                                "Generate Script"
                            }
                        }
                    }
                }
            }

            // Loading state
            if *is_generating.read() {
                LoadingSpinner {}
            }

            // Script viewer
            if let Some(script) = generated_script() {
                ScriptViewer {
                    script,
                    on_copy,
                    copy_status: copy_status(),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Script viewer
// ---------------------------------------------------------------------------

#[component]
fn ScriptViewer(
    script: CanvassingScript,
    on_copy: EventHandler<MouseEvent>,
    copy_status: Option<String>,
) -> Element {
    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                // Header with metadata and copy button
                div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 mb-4",
                    div {
                        h2 { class: "card-title text-lg", "Generated Script" }
                        div { class: "flex flex-wrap gap-2 mt-1",
                            span { class: "badge badge-outline badge-sm",
                                "Segment: {script.voter_segment}"
                            }
                            span { class: "badge badge-outline badge-sm",
                                "Candidate: {script.candidate_name}"
                            }
                            if let Some(ref created) = script.created_at {
                                span { class: "badge badge-ghost badge-sm",
                                    "{created}"
                                }
                            }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        if let Some(status) = copy_status {
                            span { class: "text-sm text-success", "{status}" }
                        }
                        button {
                            class: "btn btn-outline btn-sm",
                            onclick: move |evt| on_copy.call(evt),
                            "Copy Full Script"
                        }
                    }
                }

                // Script sections as collapsible cards
                div { class: "space-y-3",
                    for (i, section) in script.script_sections.iter().enumerate() {
                        ScriptSectionCard {
                            key: "{i}",
                            section: section.clone(),
                            default_open: i == 0,
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Script section card (collapsible)
// ---------------------------------------------------------------------------

/// Return the DaisyUI badge class for a section type.
fn section_badge_class(section_type: &str) -> &'static str {
    match section_type {
        "opening" => "badge badge-primary badge-sm",
        "issue_discussion" => "badge badge-info badge-sm",
        "objection_handling" => "badge badge-warning badge-sm",
        "closing" => "badge badge-success badge-sm",
        _ => "badge badge-ghost badge-sm",
    }
}

/// Human-readable label for a section type.
fn section_type_label(section_type: &str) -> &'static str {
    match section_type {
        "opening" => "Opening",
        "issue_discussion" => "Issue Discussion",
        "objection_handling" => "Objection Handling",
        "closing" => "Closing",
        _ => "Section",
    }
}

#[component]
fn ScriptSectionCard(section: ScriptSection, default_open: bool) -> Element {
    let mut is_open = use_signal(move || default_open);
    let badge_class = section_badge_class(&section.section_type);
    let type_label = section_type_label(&section.section_type);

    rsx! {
        div { class: "collapse collapse-arrow bg-slate-800/30 rounded-lg",
            input {
                r#type: "checkbox",
                checked: *is_open.read(),
                onchange: move |_| is_open.toggle(),
            }
            div { class: "collapse-title font-medium flex items-center gap-2",
                span { class: "{badge_class}", "{type_label}" }
                span { "{section.title}" }
            }
            div { class: "collapse-content",
                // Script content
                div { class: "prose prose-sm max-w-none mt-2",
                    p { class: "whitespace-pre-wrap text-base-content/90", "{section.content}" }
                }

                // Talking points
                if !section.talking_points.is_empty() {
                    div { class: "mt-3",
                        h4 { class: "text-sm font-semibold text-slate-400 mb-1",
                            "Talking Points"
                        }
                        ul { class: "list-disc list-inside space-y-1",
                            for point in section.talking_points.iter() {
                                li { class: "text-sm text-base-content/80", "{point}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
