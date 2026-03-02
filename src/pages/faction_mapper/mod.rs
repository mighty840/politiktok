use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::faction_mapper::{
    analyze_factions, map_consensus, ConsensusMap, Faction, FactionAnalysis,
};

#[component]
pub fn FactionMapperPage() -> Element {
    // Input state
    let mut party_context = use_signal(String::new);
    let mut known_figures = use_signal(String::new);

    // Analysis state
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis = use_signal(|| Option::<FactionAnalysis>::None);

    // Consensus mapping state
    let mut proposal_text = use_signal(String::new);
    let mut is_mapping = use_signal(|| false);
    let mut consensus = use_signal(|| Option::<ConsensusMap>::None);

    let on_analyze = move |_| {
        let context = party_context().trim().to_string();
        let figures = known_figures().trim().to_string();

        if context.is_empty() {
            error_msg.set(Some("Party context is required.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);
        consensus.set(None);

        spawn(async move {
            match analyze_factions(context, figures).await {
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

    let on_map_consensus = move |_| {
        let proposal = proposal_text().trim().to_string();
        let current_analysis = match analysis() {
            Some(a) => a,
            None => return,
        };

        if proposal.is_empty() {
            error_msg.set(Some("Policy proposal is required for consensus mapping.".to_string()));
            return;
        }

        let factions_json =
            serde_json::to_string_pretty(&current_analysis.factions).unwrap_or_default();

        is_mapping.set(true);
        error_msg.set(None);

        spawn(async move {
            match map_consensus(factions_json, proposal).await {
                Ok(result) => {
                    consensus.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Consensus mapping failed: {e}")));
                }
            }
            is_mapping.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Header
            div {
                h1 { class: "text-3xl font-bold", "Faction Mapper" }
                p { class: "text-base-content/70",
                    "Map internal party factions, identify alliances and conflicts, and analyze consensus on policy proposals."
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

            // Input section
            div { class: "card bg-base-100 shadow-sm",
                div { class: "card-body space-y-4",
                    h2 { class: "card-title text-lg", "Faction Analysis Input" }

                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Party / Political Context" }
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Describe the party, its history, current dynamics, recent events...",
                            rows: "5",
                            value: "{party_context}",
                            oninput: move |evt| party_context.set(evt.value()),
                        }
                    }

                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Known Figures" }
                            span { class: "label-text-alt text-base-content/50", "One per line" }
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "List known political figures, one per line...",
                            rows: "4",
                            value: "{known_figures}",
                            oninput: move |evt| known_figures.set(evt.value()),
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: *is_analyzing.read(),
                        onclick: on_analyze,
                        if *is_analyzing.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Analyzing..."
                        } else {
                            "Analyze Factions"
                        }
                    }
                }
            }

            // Results
            if *is_analyzing.read() {
                div { class: "flex items-center justify-center py-12",
                    div { class: "text-center space-y-4",
                        LoadingSpinner {}
                        p { class: "text-base-content/60", "Mapping internal factions..." }
                    }
                }
            } else if let Some(result) = analysis() {
                // Faction cards
                div { class: "space-y-4",
                    h3 { class: "text-xl font-semibold", "Factions ({result.factions.len()})" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4",
                        for faction in result.factions.iter() {
                            FactionCard { faction: faction.clone() }
                        }
                    }
                }

                // Alliances
                if !result.alliances.is_empty() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h3 { class: "card-title text-lg", "Alliances" }
                            div { class: "space-y-2",
                                for (a, b) in result.alliances.iter() {
                                    div { class: "flex items-center gap-2",
                                        span { class: "badge badge-success badge-outline", "{a}" }
                                        span { class: "text-base-content/50", "---" }
                                        span { class: "badge badge-success badge-outline", "{b}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Conflicts
                if !result.conflicts.is_empty() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h3 { class: "card-title text-lg", "Conflicts" }
                            div { class: "space-y-2",
                                for (a, b, issue) in result.conflicts.iter() {
                                    div { class: "flex items-center gap-2 flex-wrap",
                                        span { class: "badge badge-error badge-outline", "{a}" }
                                        span { class: "text-base-content/50", "vs" }
                                        span { class: "badge badge-error badge-outline", "{b}" }
                                        span { class: "text-sm text-base-content/70 italic", "on {issue}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Consensus mapping section
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-4",
                        h3 { class: "card-title text-lg", "Consensus Mapping" }
                        p { class: "text-base-content/70 text-sm",
                            "Test how the identified factions would respond to a specific policy proposal."
                        }

                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "Policy Proposal" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full",
                                placeholder: "Describe the policy proposal to test against factions...",
                                rows: "3",
                                value: "{proposal_text}",
                                oninput: move |evt| proposal_text.set(evt.value()),
                            }
                        }

                        button {
                            class: "btn btn-secondary",
                            disabled: *is_mapping.read(),
                            onclick: on_map_consensus,
                            if *is_mapping.read() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Mapping..."
                            } else {
                                "Map Consensus"
                            }
                        }

                        if *is_mapping.read() {
                            div { class: "flex items-center justify-center py-6",
                                LoadingSpinner {}
                            }
                        } else if let Some(cons) = consensus() {
                            div { class: "space-y-3",
                                div { class: "bg-base-200 rounded-lg p-4",
                                    p { class: "font-medium mb-2", "Strategic Analysis" }
                                    p { class: "text-sm text-base-content/80", "{cons.analysis}" }
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-3",
                                    div { class: "bg-success/10 rounded-lg p-3 border border-success/20",
                                        p { class: "font-medium text-success text-sm mb-2", "Supporters" }
                                        for s in cons.supporters.iter() {
                                            div { class: "badge badge-success badge-sm mr-1 mb-1", "{s}" }
                                        }
                                        if cons.supporters.is_empty() {
                                            p { class: "text-xs text-base-content/50", "None identified" }
                                        }
                                    }
                                    div { class: "bg-error/10 rounded-lg p-3 border border-error/20",
                                        p { class: "font-medium text-error text-sm mb-2", "Opponents" }
                                        for o in cons.opponents.iter() {
                                            div { class: "badge badge-error badge-sm mr-1 mb-1", "{o}" }
                                        }
                                        if cons.opponents.is_empty() {
                                            p { class: "text-xs text-base-content/50", "None identified" }
                                        }
                                    }
                                    div { class: "bg-warning/10 rounded-lg p-3 border border-warning/20",
                                        p { class: "font-medium text-warning text-sm mb-2", "Swing Factions" }
                                        for sw in cons.swing_factions.iter() {
                                            div { class: "badge badge-warning badge-sm mr-1 mb-1", "{sw}" }
                                        }
                                        if cons.swing_factions.is_empty() {
                                            p { class: "text-xs text-base-content/50", "None identified" }
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

#[component]
fn FactionCard(faction: Faction) -> Element {
    let influence_pct = (faction.influence_score * 100.0) as u32;
    let influence_color = if faction.influence_score >= 0.7 {
        "progress-error"
    } else if faction.influence_score >= 0.4 {
        "progress-warning"
    } else {
        "progress-info"
    };

    rsx! {
        div { class: "card bg-base-100 shadow-sm border border-base-300",
            div { class: "card-body",
                h4 { class: "card-title text-base", "{faction.name}" }
                p { class: "text-sm text-base-content/70 italic", "{faction.ideology}" }

                // Influence score
                div { class: "mt-2",
                    div { class: "flex justify-between text-xs mb-1",
                        span { "Influence" }
                        span { "{influence_pct}%" }
                    }
                    progress {
                        class: "progress {influence_color} w-full",
                        value: "{influence_pct}",
                        max: "100",
                    }
                }

                // Key figures
                if !faction.key_figures.is_empty() {
                    div { class: "mt-2",
                        p { class: "text-xs font-medium text-base-content/60 mb-1", "Key Figures" }
                        div { class: "flex flex-wrap gap-1",
                            for figure in faction.key_figures.iter() {
                                span { class: "badge badge-outline badge-sm", "{figure}" }
                            }
                        }
                    }
                }

                // Positions
                if !faction.positions.is_empty() {
                    div { class: "mt-2",
                        p { class: "text-xs font-medium text-base-content/60 mb-1", "Positions" }
                        ul { class: "list-disc list-inside text-xs text-base-content/70 space-y-0.5",
                            for pos in faction.positions.iter() {
                                li { "{pos}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
