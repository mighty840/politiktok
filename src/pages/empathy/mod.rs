use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::empathy_simulator::{
    get_default_personas, simulate_reactions, Persona, PersonaReaction, SimulationResult,
};

/// Badge color class for a given sentiment value.
fn sentiment_badge_class(sentiment: &str) -> &'static str {
    match sentiment {
        "positive" => "badge badge-success",
        "negative" => "badge badge-error",
        "mixed" => "badge badge-warning",
        _ => "badge badge-ghost",
    }
}

/// Descriptive label for sentiment.
fn sentiment_label(sentiment: &str) -> &str {
    match sentiment {
        "positive" => "Positive",
        "negative" => "Negative",
        "mixed" => "Mixed",
        "neutral" => "Neutral",
        _ => sentiment,
    }
}

#[component]
pub fn EmpathyPage() -> Element {
    // Load default personas on mount
    let personas_resource = use_resource(|| async { get_default_personas().await });

    // Selected persona IDs
    let mut selected_ids = use_signal(|| Vec::<String>::new());

    // Policy text input
    let mut policy_text = use_signal(String::new);

    // Simulation state
    let mut is_simulating = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut result = use_signal(|| Option::<SimulationResult>::None);

    // Toggle a persona selection
    let mut toggle_persona = move |id: String| {
        let mut ids = selected_ids.write();
        if let Some(pos) = ids.iter().position(|i| i == &id) {
            ids.remove(pos);
        } else {
            ids.push(id);
        }
    };

    // Select / deselect all personas
    let mut select_all = move |all_personas: Vec<Persona>| {
        let current = selected_ids.read();
        if current.len() == all_personas.len() {
            // Deselect all
            drop(current);
            selected_ids.set(Vec::new());
        } else {
            // Select all
            drop(current);
            selected_ids.set(all_personas.iter().map(|p| p.id.clone()).collect());
        }
    };

    // Handle simulate
    let mut on_simulate = move |all_personas: Vec<Persona>| {
        let text = policy_text().trim().to_string();
        let sel_ids = selected_ids().clone();

        if text.is_empty() {
            error_msg.set(Some("Please enter policy text to simulate.".to_string()));
            return;
        }
        if sel_ids.is_empty() {
            error_msg.set(Some("Please select at least one persona.".to_string()));
            return;
        }

        let selected_personas: Vec<Persona> = all_personas
            .into_iter()
            .filter(|p| sel_ids.contains(&p.id))
            .collect();

        is_simulating.set(true);
        error_msg.set(None);

        spawn(async move {
            match simulate_reactions(text, selected_personas).await {
                Ok(sim) => {
                    result.set(Some(sim));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Simulation failed: {e}")));
                }
            }
            is_simulating.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Audience Empathy Simulator" }
                p { class: "text-slate-400",
                    "Simulate how different audience segments would perceive and react to your policy proposals. Select personas, enter your policy text, and see reactions from diverse perspectives."
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

            // Main layout
            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Input panel
                div { class: "w-full lg:w-1/3 space-y-4",

                    // Policy text card
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Policy Text" }
                            div { class: "form-control",
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Enter the policy proposal or messaging you want to test...",
                                    rows: "8",
                                    value: "{policy_text}",
                                    oninput: move |evt| policy_text.set(evt.value()),
                                }
                            }
                        }
                    }

                    // Persona selection card
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Select Personas" }

                            match &*personas_resource.read() {
                                Some(Ok(personas)) => {
                                    let all_personas = personas.clone();
                                    let all_for_select = personas.clone();
                                    let all_for_button = personas.clone();
                                    let all_selected = selected_ids.read().len() == personas.len();

                                    rsx! {
                                        // Select all toggle
                                        div { class: "flex justify-between items-center",
                                            label { class: "flex items-center gap-2 cursor-pointer",
                                                input {
                                                    r#type: "checkbox",
                                                    class: "checkbox checkbox-primary checkbox-sm",
                                                    checked: all_selected,
                                                    onchange: move |_| select_all(all_for_select.clone()),
                                                }
                                                span { class: "label-text font-medium", "Select All" }
                                            }
                                            span { class: "text-sm text-slate-500",
                                                "{selected_ids.read().len()} / {all_personas.len()} selected"
                                            }
                                        }

                                        div { class: "divider my-1" }

                                        // Persona checkboxes
                                        div { class: "space-y-3",
                                            for persona in all_personas.iter() {
                                                {
                                                    let id = persona.id.clone();
                                                    let is_checked = selected_ids.read().contains(&id);
                                                    let id_for_toggle = id.clone();
                                                    rsx! {
                                                        label { class: "flex items-start gap-3 cursor-pointer p-2 rounded-lg hover:bg-slate-700/30 transition-colors",
                                                            input {
                                                                r#type: "checkbox",
                                                                class: "checkbox checkbox-primary checkbox-sm mt-0.5",
                                                                checked: is_checked,
                                                                onchange: move |_| toggle_persona(id_for_toggle.clone()),
                                                            }
                                                            div {
                                                                div { class: "font-medium text-sm", "{persona.name}" }
                                                                div { class: "text-xs text-slate-500", "{persona.demographic}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Simulate button
                                        button {
                                            class: "btn btn-primary w-full mt-2",
                                            disabled: *is_simulating.read(),
                                            onclick: move |_| on_simulate(all_for_button.clone()),
                                            if *is_simulating.read() {
                                                span { class: "loading loading-spinner loading-sm" }
                                                "Simulating..."
                                            } else {
                                                "Run Simulation"
                                            }
                                        }
                                    }
                                }
                                Some(Err(e)) => {
                                    rsx! {
                                        div { class: "alert alert-warning text-sm",
                                            "Failed to load personas: {e}"
                                        }
                                    }
                                }
                                None => {
                                    rsx! {
                                        div { class: "flex justify-center py-4",
                                            LoadingSpinner {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right: Results panel
                div { class: "w-full lg:w-2/3 space-y-4",

                    if *is_simulating.read() {
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center space-y-4",
                                    LoadingSpinner {}
                                    p { class: "text-slate-400", "Simulating persona reactions..." }
                                    p { class: "text-sm text-base-content/40", "This may take a moment as each persona is evaluated individually." }
                                }
                            }
                        }
                    } else if let Some(sim) = result() {
                        // Aggregate summary card
                        AggregateSummaryCard {
                            aggregate_sentiment: sim.aggregate_sentiment.clone(),
                            red_flags: sim.red_flags.clone(),
                            reaction_count: sim.reactions.len(),
                        }

                        // Individual persona reaction cards
                        for reaction in sim.reactions.iter() {
                            PersonaReactionCard {
                                reaction: reaction.clone(),
                            }
                        }
                    } else {
                        // Empty state
                        div { class: "glass-card gradient-border min-h-[400px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    div { class: "text-6xl mb-4 opacity-20", "🎭" }
                                    p { class: "text-lg font-medium text-slate-500 mb-2",
                                        "No simulation results yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Enter policy text, select personas, and click Run Simulation to see how different audiences would react."
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

/// Card showing the aggregate simulation summary including overall sentiment and red flags.
#[component]
fn AggregateSummaryCard(
    aggregate_sentiment: String,
    red_flags: Vec<String>,
    reaction_count: usize,
) -> Element {
    rsx! {
        div { class: "glass-card gradient-border border-l-4 border-primary",
            div { class: "p-6",
                div { class: "flex flex-wrap items-center justify-between gap-2 mb-3",
                    h2 { class: "card-title text-lg", "Simulation Summary" }
                    div { class: "flex items-center gap-2",
                        span { class: "text-sm text-slate-400", "Overall:" }
                        span {
                            class: sentiment_badge_class(&aggregate_sentiment),
                            "{sentiment_label(&aggregate_sentiment)}"
                        }
                    }
                }

                div { class: "text-sm text-slate-400 mb-3",
                    "{reaction_count} persona(s) evaluated"
                }

                if !red_flags.is_empty() {
                    div { class: "mt-2",
                        h3 { class: "font-semibold text-sm text-error mb-2", "Red Flags" }
                        ul { class: "list-disc list-inside space-y-1",
                            for flag in red_flags.iter() {
                                li { class: "text-sm text-base-content/80", "{flag}" }
                            }
                        }
                    }
                } else {
                    div { class: "mt-2",
                        p { class: "text-sm text-success", "No red flags detected -- the messaging appears broadly acceptable across personas." }
                    }
                }
            }
        }
    }
}

/// Card displaying a single persona's reaction to the policy.
#[component]
fn PersonaReactionCard(reaction: PersonaReaction) -> Element {
    let score_pct = (reaction.persuasion_score * 100.0).round() as u32;
    let progress_class = if reaction.persuasion_score >= 0.7 {
        "progress progress-success"
    } else if reaction.persuasion_score >= 0.4 {
        "progress progress-warning"
    } else {
        "progress progress-error"
    };

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                // Header: persona name + sentiment badge
                div { class: "flex flex-wrap items-center justify-between gap-2 mb-3",
                    div {
                        h3 { class: "font-bold text-base", "{reaction.persona.name}" }
                        p { class: "text-xs text-slate-500", "{reaction.persona.demographic}" }
                    }
                    span {
                        class: sentiment_badge_class(&reaction.sentiment),
                        "{sentiment_label(&reaction.sentiment)}"
                    }
                }

                // Reaction text
                div { class: "bg-slate-800/30 rounded-lg p-3 mb-4",
                    p { class: "text-sm italic leading-relaxed", "\"{reaction.reaction}\"" }
                }

                // Key concerns
                if !reaction.key_concerns.is_empty() {
                    div { class: "mb-4",
                        h4 { class: "text-xs font-semibold uppercase tracking-wide text-slate-400 mb-2",
                            "Key Concerns"
                        }
                        div { class: "flex flex-wrap gap-2",
                            for concern in reaction.key_concerns.iter() {
                                span { class: "badge badge-outline badge-sm", "{concern}" }
                            }
                        }
                    }
                }

                // Persuasion score as progress bar
                div {
                    div { class: "flex justify-between items-center mb-1",
                        h4 { class: "text-xs font-semibold uppercase tracking-wide text-slate-400",
                            "Persuasion Score"
                        }
                        span { class: "text-sm font-mono font-bold", "{score_pct}%" }
                    }
                    progress {
                        class: "{progress_class} w-full",
                        value: "{score_pct}",
                        max: "100",
                    }
                }
            }
        }
    }
}
