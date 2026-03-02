use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::regulatory_monitor::{
    analyze_regulatory_impact, generate_regulatory_brief, list_regulatory_updates,
    RegulatoryBrief, RegulatoryUpdate,
};

/// Badge class for urgency level.
fn urgency_badge(urgency: &str) -> &'static str {
    match urgency {
        "urgent" => "badge badge-error",
        "important" => "badge badge-warning",
        "routine" => "badge badge-info",
        _ => "badge badge-ghost",
    }
}

#[component]
pub fn RegulatoryPage() -> Element {
    // Updates list state
    let mut urgency_filter = use_signal(|| "all".to_string());
    let mut updates = use_signal(|| Vec::<RegulatoryUpdate>::new());
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    // Selected updates for brief generation
    let mut selected_ids = use_signal(|| Vec::<String>::new());
    let mut is_generating_brief = use_signal(|| false);
    let mut brief_result = use_signal(|| Option::<RegulatoryBrief>::None);

    // Impact analysis state
    let mut impact_text = use_signal(String::new);
    let mut impact_context = use_signal(String::new);
    let mut is_analyzing = use_signal(|| false);
    let mut impact_result = use_signal(|| Option::<RegulatoryUpdate>::None);

    // Active section tab
    let mut active_section = use_signal(|| "updates".to_string());

    let load_updates = move |_| {
        let filter = urgency_filter().clone();
        let filter_opt = if filter == "all" {
            None
        } else {
            Some(filter)
        };

        is_loading.set(true);
        error_msg.set(None);

        spawn(async move {
            match list_regulatory_updates(filter_opt).await {
                Ok(result) => {
                    updates.set(result);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to load updates: {e}")));
                }
            }
            is_loading.set(false);
        });
    };

    let mut toggle_selection = move |id: String| {
        let mut ids = selected_ids.write();
        if let Some(pos) = ids.iter().position(|i| i == &id) {
            ids.remove(pos);
        } else {
            ids.push(id);
        }
    };

    let on_generate_brief = move |_| {
        let ids = selected_ids().clone();
        if ids.is_empty() {
            error_msg.set(Some("Select at least one update for the brief.".to_string()));
            return;
        }

        is_generating_brief.set(true);
        error_msg.set(None);

        spawn(async move {
            match generate_regulatory_brief(ids).await {
                Ok(result) => {
                    brief_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Brief generation failed: {e}")));
                }
            }
            is_generating_brief.set(false);
        });
    };

    let on_analyze_impact = move |_| {
        let text = impact_text().trim().to_string();
        let ctx = impact_context().trim().to_string();

        if text.is_empty() {
            error_msg.set(Some("Regulatory update text is required.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);

        spawn(async move {
            match analyze_regulatory_impact(text, ctx).await {
                Ok(result) => {
                    impact_result.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Impact analysis failed: {e}")));
                }
            }
            is_analyzing.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Header
            div {
                h1 { class: "text-3xl font-bold", "Regulatory Monitor" }
                p { class: "text-base-content/70",
                    "Track regulatory changes, analyze their impact, and generate executive briefs."
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

            // Section tabs
            div { class: "tabs tabs-bordered",
                button {
                    class: if active_section() == "updates" { "tab tab-active" } else { "tab" },
                    onclick: move |_| active_section.set("updates".to_string()),
                    "Updates"
                }
                button {
                    class: if active_section() == "analyze" { "tab tab-active" } else { "tab" },
                    onclick: move |_| active_section.set("analyze".to_string()),
                    "Impact Analysis"
                }
            }

            // Updates section
            if active_section() == "updates" {
                // Filter and load controls
                div { class: "flex flex-wrap items-end gap-3",
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Urgency Filter" }
                        }
                        select {
                            class: "select select-bordered select-sm",
                            value: "{urgency_filter}",
                            onchange: move |evt: Event<FormData>| urgency_filter.set(evt.value()),
                            option { value: "all", "All" }
                            option { value: "urgent", "Urgent" }
                            option { value: "important", "Important" }
                            option { value: "routine", "Routine" }
                        }
                    }
                    button {
                        class: "btn btn-primary btn-sm",
                        disabled: *is_loading.read(),
                        onclick: load_updates,
                        if *is_loading.read() {
                            span { class: "loading loading-spinner loading-xs" }
                            "Loading..."
                        } else {
                            "Load Updates"
                        }
                    }
                    if !selected_ids().is_empty() {
                        button {
                            class: "btn btn-secondary btn-sm",
                            disabled: *is_generating_brief.read(),
                            onclick: on_generate_brief,
                            if *is_generating_brief.read() {
                                span { class: "loading loading-spinner loading-xs" }
                                "Generating Brief..."
                            } else {
                                "Generate Brief ({selected_ids().len()} selected)"
                            }
                        }
                    }
                }

                // Updates table
                if *is_loading.read() {
                    div { class: "flex items-center justify-center py-12",
                        LoadingSpinner {}
                    }
                } else if updates().is_empty() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body text-center",
                            p { class: "text-base-content/50", "No regulatory updates found. Click \"Load Updates\" to fetch from the database." }
                        }
                    }
                } else {
                    div { class: "overflow-x-auto",
                        table { class: "table table-zebra w-full",
                            thead {
                                tr {
                                    th { "" }
                                    th { "Title" }
                                    th { "Urgency" }
                                    th { "Summary" }
                                    th { "Source" }
                                    th { "Date" }
                                }
                            }
                            tbody {
                                for update in updates().iter() {
                                    {
                                        let id = update.id.clone();
                                        let id_for_toggle = id.clone();
                                        let is_selected = selected_ids().contains(&id);
                                        rsx! {
                                            tr {
                                                td {
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "checkbox checkbox-sm",
                                                        checked: is_selected,
                                                        onchange: move |_| toggle_selection(id_for_toggle.clone()),
                                                    }
                                                }
                                                td { class: "font-medium max-w-xs truncate", "{update.title}" }
                                                td {
                                                    span { class: urgency_badge(&update.urgency), "{update.urgency}" }
                                                }
                                                td { class: "max-w-sm truncate text-sm", "{update.summary}" }
                                                td { class: "text-sm",
                                                    "{update.source_name.as_deref().unwrap_or(\"-\")}"
                                                }
                                                td { class: "text-sm text-base-content/60", "{update.created_at}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Brief result
                if *is_generating_brief.read() {
                    div { class: "flex items-center justify-center py-8",
                        div { class: "text-center space-y-4",
                            LoadingSpinner {}
                            p { class: "text-base-content/60", "Generating regulatory brief..." }
                        }
                    }
                } else if let Some(brief) = brief_result() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h3 { class: "card-title text-lg", "Regulatory Brief" }
                            div { class: "badge badge-outline mb-3",
                                "{brief.updates.len()} updates covered"
                            }
                            div { class: "prose max-w-none",
                                pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                    "{brief.analysis}"
                                }
                            }
                        }
                    }
                }
            }

            // Impact Analysis section
            if active_section() == "analyze" {
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-4",
                        h2 { class: "card-title text-lg", "Regulatory Impact Analysis" }

                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Regulatory Update Text" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Paste the regulatory text, rule change, or update here...",
                                rows: "6",
                                value: "{impact_text}",
                                oninput: move |evt| impact_text.set(evt.value()),
                            }
                        }

                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Context" }
                                span { class: "label-text-alt text-base-content/50", "Optional" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Provide context about your campaign, constituents, or relevant issues...",
                                rows: "3",
                                value: "{impact_context}",
                                oninput: move |evt| impact_context.set(evt.value()),
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: *is_analyzing.read(),
                            onclick: on_analyze_impact,
                            if *is_analyzing.read() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Analyzing..."
                            } else {
                                "Analyze Impact"
                            }
                        }
                    }
                }

                if *is_analyzing.read() {
                    div { class: "flex items-center justify-center py-8",
                        div { class: "text-center space-y-4",
                            LoadingSpinner {}
                            p { class: "text-base-content/60", "Analyzing regulatory impact..." }
                        }
                    }
                } else if let Some(result) = impact_result() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-3",
                            h3 { class: "card-title text-lg", "Impact Analysis Result" }
                            div { class: "flex gap-2",
                                span { class: urgency_badge(&result.urgency), "{result.urgency}" }
                            }
                            div { class: "bg-base-200 rounded-lg p-4",
                                p { class: "font-medium mb-1", "Summary" }
                                p { class: "text-sm", "{result.summary}" }
                            }
                            if let Some(ref assessment) = result.impact_assessment {
                                div { class: "bg-base-200 rounded-lg p-4",
                                    p { class: "font-medium mb-1", "Impact Assessment" }
                                    pre { class: "whitespace-pre-wrap text-sm font-sans", "{assessment}" }
                                }
                            }
                            if let Some(ref action) = result.action_required {
                                div { class: "bg-warning/10 rounded-lg p-4 border border-warning/20",
                                    p { class: "font-medium text-warning mb-1", "Action Required" }
                                    p { class: "text-sm", "{action}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
