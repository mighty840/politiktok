use dioxus::prelude::*;

use crate::app::Route;
use crate::components::LoadingSpinner;
use crate::models::candidate::{Contradiction, Opponent};
use crate::modules::opposition_research::{
    create_opponent, detect_contradictions, generate_briefing, get_opponent, list_opponents,
    DebateBriefing,
};

// ---------------------------------------------------------------------------
// Opposition list page
// ---------------------------------------------------------------------------

#[component]
pub fn OppositionPage() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut refresh = use_signal(|| 0u32);

    let opponents = use_resource(move || {
        let _trigger = refresh();
        async move { list_opponents().await.unwrap_or_default() }
    });

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            // Header
            div { class: "flex items-center justify-between mb-6",
                div {
                    h1 { class: "text-3xl font-bold", "Opposition Research" }
                    p { class: "text-slate-400 mt-1",
                        "Track opponents, analyze policy positions, and prepare debate briefings."
                    }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_modal.set(true),
                    "+ Add Opponent"
                }
            }

            // Opponent cards
            match &*opponents.read() {
                None => rsx! { LoadingSpinner {} },
                Some(list) if list.is_empty() => rsx! {
                    div { class: "glass-card gradient-border",
                        div { class: "card-body items-center text-center py-16",
                            h3 { class: "text-lg font-semibold text-slate-400",
                                "No opponents added yet"
                            }
                            p { class: "text-slate-500 mt-1",
                                "Add an opponent to begin tracking their positions and generating briefings."
                            }
                            button {
                                class: "btn btn-primary btn-sm mt-4",
                                onclick: move |_| show_modal.set(true),
                                "Add Your First Opponent"
                            }
                        }
                    }
                },
                Some(list) => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        for opponent in list.iter() {
                            OpponentCard { opponent: opponent.clone() }
                        }
                    }
                },
            }

            // Add Opponent Modal
            if *show_modal.read() {
                AddOpponentModal {
                    on_close: move || show_modal.set(false),
                    on_created: move || {
                        show_modal.set(false);
                        refresh += 1;
                    },
                }
            }
        }
    }
}

/// A card displaying a single opponent in the list view.
#[component]
fn OpponentCard(opponent: Opponent) -> Element {
    let id = opponent.id.clone();
    let position_count = opponent
        .policy_positions
        .as_object()
        .map(|o| o.len())
        .unwrap_or(0);

    rsx! {
        Link {
            to: Route::OpponentDetailPage { id },
            class: "glass-card gradient-border hover:shadow-md transition-shadow cursor-pointer",
            div { class: "p-6",
                div { class: "flex items-start justify-between",
                    h3 { class: "card-title text-lg", "{opponent.name}" }
                    if let Some(ref party) = opponent.party {
                        span { class: "badge badge-outline badge-sm", "{party}" }
                    }
                }
                if let Some(ref district) = opponent.district {
                    p { class: "text-sm text-slate-400", "{district}" }
                }
                if let Some(ref bio) = opponent.bio {
                    p { class: "text-sm text-slate-400 mt-2 line-clamp-3", "{bio}" }
                }
                div { class: "flex items-center gap-2 mt-3",
                    span { class: "badge badge-ghost badge-sm",
                        "{position_count} policy positions"
                    }
                    if let Some(ref created) = opponent.created_at {
                        span { class: "text-xs text-base-content/40", "Added {created}" }
                    }
                }
            }
        }
    }
}

/// Modal form for adding a new opponent.
#[component]
fn AddOpponentModal(on_close: EventHandler, on_created: EventHandler) -> Element {
    let mut name = use_signal(String::new);
    let mut party = use_signal(String::new);
    let mut district = use_signal(String::new);
    let mut bio = use_signal(String::new);
    let mut positions = use_signal(String::new);
    let mut is_saving = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_submit = move |_| {
        let n = name().trim().to_string();
        let p = party().trim().to_string();
        let d = district().trim().to_string();
        let b = bio().trim().to_string();
        let pos = positions().trim().to_string();

        if n.is_empty() {
            error_msg.set(Some("Name is required.".to_string()));
            return;
        }

        is_saving.set(true);
        error_msg.set(None);

        spawn(async move {
            match create_opponent(n, p, d, b, pos).await {
                Ok(_) => on_created.call(()),
                Err(e) => error_msg.set(Some(format!("Failed to create opponent: {e}"))),
            }
            is_saving.set(false);
        });
    };

    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box max-w-lg",
                h3 { class: "font-bold text-lg mb-4", "Add Opponent" }

                div { class: "space-y-3",
                    // Name
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Name *" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "Opponent's full name",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    // Party
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Party" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "e.g. Democratic, Republican",
                            value: "{party}",
                            oninput: move |evt| party.set(evt.value()),
                        }
                    }
                    // District
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "District" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "e.g. CA-12, State Senate District 5",
                            value: "{district}",
                            oninput: move |evt| district.set(evt.value()),
                        }
                    }
                    // Bio
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Bio" } }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Background, career history, notable positions...",
                            rows: "3",
                            value: "{bio}",
                            oninput: move |evt| bio.set(evt.value()),
                        }
                    }
                    // Policy positions (JSON)
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text", "Policy Positions (JSON)" }
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full font-mono text-sm",
                            placeholder: "{{\"healthcare\": \"Supports Medicare for All\", \"taxes\": \"Opposes capital gains tax increase\"}}",
                            rows: "5",
                            value: "{positions}",
                            oninput: move |evt| positions.set(evt.value()),
                        }
                        label { class: "label",
                            span { class: "label-text-alt text-slate-500",
                                "Enter as JSON object or plain text"
                            }
                        }
                    }
                }

                if let Some(err) = error_msg() {
                    div { class: "alert alert-error mt-3",
                        span { "{err}" }
                    }
                }

                div { class: "modal-action",
                    button {
                        class: "btn",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: *is_saving.read(),
                        onclick: on_submit,
                        if *is_saving.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Saving..."
                        } else {
                            "Add Opponent"
                        }
                    }
                }
            }
            div {
                class: "modal-backdrop",
                onclick: move |_| on_close.call(()),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Opponent detail page
// ---------------------------------------------------------------------------

#[component]
pub fn OpponentDetailPage(id: String) -> Element {
    let opponent_id = id.clone();

    let opponent = use_resource(move || {
        let oid = opponent_id.clone();
        async move { get_opponent(oid).await.ok() }
    });

    // Briefing state
    let mut briefing_result = use_signal(|| Option::<DebateBriefing>::None);
    let mut briefing_loading = use_signal(|| false);
    let mut briefing_error = use_signal(|| Option::<String>::None);
    let mut topics_input = use_signal(String::new);
    let mut focus_input = use_signal(String::new);

    // Contradictions state
    let mut contradictions = use_signal(Vec::<Contradiction>::new);
    let mut contradictions_loading = use_signal(|| false);
    let mut contradictions_error = use_signal(|| Option::<String>::None);

    let opponent_data = opponent.read();
    match &*opponent_data {
        None => rsx! { LoadingSpinner {} },
        Some(None) => rsx! {
            div { class: "p-6 max-w-4xl mx-auto",
                div { class: "alert alert-error",
                    span { "Opponent not found." }
                }
                Link {
                    to: Route::OppositionPage {},
                    class: "btn btn-ghost mt-4",
                    "Back to Opponents"
                }
            }
        },
        Some(Some(opp)) => {
            let opp = opp.clone();
            let opp_id_briefing = id.clone();
            let opp_id_contradictions = id.clone();

            let on_generate_briefing = move |_| {
                let oid = opp_id_briefing.clone();
                let topics = topics_input().clone();
                let focus = focus_input().clone();

                briefing_loading.set(true);
                briefing_error.set(None);

                spawn(async move {
                    match generate_briefing(oid, topics, focus).await {
                        Ok(b) => briefing_result.set(Some(b)),
                        Err(e) => briefing_error.set(Some(format!("{e}"))),
                    }
                    briefing_loading.set(false);
                });
            };

            let on_detect_contradictions = move |_| {
                let oid = opp_id_contradictions.clone();

                contradictions_loading.set(true);
                contradictions_error.set(None);

                spawn(async move {
                    match detect_contradictions(oid).await {
                        Ok(c) => contradictions.set(c),
                        Err(e) => contradictions_error.set(Some(format!("{e}"))),
                    }
                    contradictions_loading.set(false);
                });
            };

            rsx! {
                div { class: "p-6 max-w-4xl mx-auto space-y-6",
                    // Back nav
                    Link {
                        to: Route::OppositionPage {},
                        class: "btn btn-ghost btn-sm gap-1",
                        "< Back to Opponents"
                    }

                    // Profile card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            div { class: "flex items-start justify-between",
                                div {
                                    h1 { class: "text-2xl font-bold", "{opp.name}" }
                                    div { class: "flex gap-2 mt-1",
                                        if let Some(ref party) = opp.party {
                                            span { class: "badge badge-primary badge-sm", "{party}" }
                                        }
                                        if let Some(ref district) = opp.district {
                                            span { class: "badge badge-outline badge-sm", "{district}" }
                                        }
                                    }
                                }
                                if let Some(ref created) = opp.created_at {
                                    span { class: "text-xs text-base-content/40", "Added {created}" }
                                }
                            }
                            if let Some(ref bio) = opp.bio {
                                p { class: "mt-3 text-base-content/80", "{bio}" }
                            }
                        }
                    }

                    // Policy positions card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Policy Positions" }
                            PolicyPositionsDisplay { positions: opp.policy_positions.clone() }
                        }
                    }

                    // Briefing generation card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Debate Briefing" }
                            p { class: "text-sm text-slate-400 mb-3",
                                "Generate an AI-powered debate briefing based on this opponent's profile and positions."
                            }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text text-sm", "Topics (comma-separated)" }
                                    }
                                    input {
                                        class: "input input-bordered input-sm w-full",
                                        r#type: "text",
                                        placeholder: "healthcare, economy, education",
                                        value: "{topics_input}",
                                        oninput: move |evt| topics_input.set(evt.value()),
                                    }
                                }
                                div { class: "form-control",
                                    label { class: "label",
                                        span { class: "label-text text-sm", "Focus" }
                                    }
                                    input {
                                        class: "input input-bordered input-sm w-full",
                                        r#type: "text",
                                        placeholder: "e.g. town hall debate, TV interview",
                                        value: "{focus_input}",
                                        oninput: move |evt| focus_input.set(evt.value()),
                                    }
                                }
                            }
                            div { class: "mt-3",
                                button {
                                    class: "btn btn-primary btn-sm",
                                    disabled: *briefing_loading.read(),
                                    onclick: on_generate_briefing,
                                    if *briefing_loading.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                        "Generating Briefing..."
                                    } else {
                                        "Generate Briefing"
                                    }
                                }
                            }

                            if let Some(err) = briefing_error() {
                                div { class: "alert alert-error mt-3",
                                    span { "{err}" }
                                }
                            }

                            if let Some(ref briefing) = *briefing_result.read() {
                                BriefingDisplay { briefing: briefing.clone() }
                            }
                        }
                    }

                    // Contradictions card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Contradiction Analysis" }
                            p { class: "text-sm text-slate-400 mb-3",
                                "Use AI to scan policy positions for contradictions, flip-flops, and inconsistencies."
                            }
                            button {
                                class: "btn btn-secondary btn-sm",
                                disabled: *contradictions_loading.read(),
                                onclick: on_detect_contradictions,
                                if *contradictions_loading.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Analyzing..."
                                } else {
                                    "Find Contradictions"
                                }
                            }

                            if let Some(err) = contradictions_error() {
                                div { class: "alert alert-error mt-3",
                                    span { "{err}" }
                                }
                            }

                            if !contradictions().is_empty() {
                                ContradictionsList { contradictions: contradictions() }
                            } else if !*contradictions_loading.read() && contradictions_error().is_none() {
                                // Only show "no results" after an actual analysis has run
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render policy positions as a key-value list or raw JSON.
#[component]
fn PolicyPositionsDisplay(positions: serde_json::Value) -> Element {
    match positions {
        serde_json::Value::Object(ref map) if !map.is_empty() => rsx! {
            div { class: "overflow-x-auto",
                table { class: "table table-sm",
                    thead {
                        tr {
                            th { "Topic" }
                            th { "Position" }
                        }
                    }
                    tbody {
                        for (key, val) in map.iter() {
                            tr {
                                td { class: "font-medium capitalize", "{key}" }
                                td {
                                    {
                                        let display = match val {
                                            serde_json::Value::String(s) => s.clone(),
                                            other => other.to_string(),
                                        };
                                        rsx! { "{display}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        serde_json::Value::String(ref s) if !s.is_empty() => rsx! {
            p { class: "text-slate-400 whitespace-pre-wrap", "{s}" }
        },
        _ => rsx! {
            p { class: "text-slate-500 italic", "No policy positions recorded." }
        },
    }
}

/// Display a generated debate briefing with markdown-style sections.
#[component]
fn BriefingDisplay(briefing: DebateBriefing) -> Element {
    // Split the briefing content by markdown headers for sectioned display
    let sections: Vec<(&str, &str)> = parse_briefing_sections(&briefing.content);

    rsx! {
        div { class: "mt-4 space-y-3",
            div { class: "flex items-center gap-2 mb-2",
                h3 { class: "font-semibold text-base", "Briefing for {briefing.opponent_name}" }
                if !briefing.topics.is_empty() {
                    for topic in briefing.topics.iter() {
                        span { class: "badge badge-info badge-xs", "{topic}" }
                    }
                }
            }

            if sections.is_empty() {
                // Fallback: render as a single block
                div { class: "bg-slate-800/30 rounded-lg p-4 text-sm whitespace-pre-wrap",
                    "{briefing.content}"
                }
            } else {
                for (title, body) in sections.iter() {
                    div { class: "collapse collapse-arrow bg-slate-800/30",
                        input { r#type: "checkbox", checked: true }
                        div { class: "collapse-title font-medium", "{title}" }
                        div { class: "collapse-content",
                            p { class: "text-sm whitespace-pre-wrap", "{body}" }
                        }
                    }
                }
            }
        }
    }
}

/// Parse briefing content into (header, body) sections based on markdown ## headers.
fn parse_briefing_sections(content: &str) -> Vec<(&str, &str)> {
    let mut sections = Vec::new();
    let mut current_title: Option<&str> = None;
    let mut current_start: Option<usize> = None;

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            // Save previous section
            if let (Some(title), Some(start)) = (current_title, current_start) {
                let body_end = content
                    .lines()
                    .take(i)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();
                let body = content[start..body_end].trim();
                if !body.is_empty() {
                    sections.push((title, body));
                }
            }
            current_title = Some(trimmed.trim_start_matches("## ").trim());
            // Body starts after this line
            let header_end: usize = content
                .lines()
                .take(i + 1)
                .map(|l| l.len() + 1)
                .sum();
            current_start = Some(header_end);
        }
    }

    // Save last section
    if let (Some(title), Some(start)) = (current_title, current_start) {
        let body = content[start..].trim();
        if !body.is_empty() {
            sections.push((title, body));
        }
    }

    sections
}

/// Display a list of detected contradictions.
#[component]
fn ContradictionsList(contradictions: Vec<Contradiction>) -> Element {
    rsx! {
        div { class: "mt-4 space-y-3",
            h3 { class: "font-semibold text-base mb-2",
                "Found {contradictions.len()} contradiction(s)"
            }
            for (idx, c) in contradictions.iter().enumerate() {
                div { class: "card bg-slate-800/30 shadow-sm",
                    div { class: "card-body p-4",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "font-medium text-sm", "#{idx + 1}: {c.topic}" }
                            ConfidenceBadge { confidence: c.confidence }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                            div { class: "bg-base-100 rounded-lg p-3",
                                p { class: "text-xs font-semibold text-slate-500 mb-1 uppercase",
                                    "Statement A"
                                }
                                p { class: "text-sm", "{c.statement_a}" }
                                if let Some(ref src) = c.source_a {
                                    p { class: "text-xs text-base-content/40 mt-1 italic",
                                        "Source: {src}"
                                    }
                                }
                            }
                            div { class: "bg-base-100 rounded-lg p-3",
                                p { class: "text-xs font-semibold text-slate-500 mb-1 uppercase",
                                    "Statement B"
                                }
                                p { class: "text-sm", "{c.statement_b}" }
                                if let Some(ref src) = c.source_b {
                                    p { class: "text-xs text-base-content/40 mt-1 italic",
                                        "Source: {src}"
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

/// Badge showing contradiction confidence level.
#[component]
fn ConfidenceBadge(confidence: f64) -> Element {
    let (label, class) = if confidence >= 0.8 {
        ("High", "badge badge-error badge-sm")
    } else if confidence >= 0.5 {
        ("Medium", "badge badge-warning badge-sm")
    } else {
        ("Low", "badge badge-info badge-sm")
    };

    let pct = (confidence * 100.0) as u32;

    rsx! {
        span { class: "{class}", title: "{pct}% confidence", "{label} ({pct}%)" }
    }
}
