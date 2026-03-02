use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::narrative_contagion::{
    analyze_narrative, predict_spread, suggest_counter_narrative, Narrative, NarrativeAnalysis,
};

/// Platform options for spread prediction.
const PLATFORM_OPTIONS: &[&str] = &[
    "Twitter/X",
    "Facebook",
    "TikTok",
    "Instagram",
    "YouTube",
    "Reddit",
    "Cable News",
    "Talk Radio",
];

#[component]
pub fn NarrativePage() -> Element {
    // Input state
    let mut input_text = use_signal(String::new);

    // Analysis state
    let mut is_analyzing = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut analysis = use_signal(|| Option::<NarrativeAnalysis>::None);

    // Spread prediction state
    let mut spread_platform = use_signal(|| "Twitter/X".to_string());
    let mut spread_theme = use_signal(String::new);
    let mut is_predicting = use_signal(|| false);
    let mut spread_result = use_signal(|| Option::<String>::None);

    // Counter-narrative state
    let mut counter_theme = use_signal(String::new);
    let mut is_countering = use_signal(|| false);
    let mut counter_results = use_signal(|| Option::<Vec<String>>::None);

    // Handle analyze
    let on_analyze = move |_| {
        let text = input_text().trim().to_string();
        if text.is_empty() {
            error_msg.set(Some("Please enter text to analyze.".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_msg.set(None);
        analysis.set(None);
        spread_result.set(None);
        counter_results.set(None);

        spawn(async move {
            match analyze_narrative(text).await {
                Ok(result) => {
                    // Auto-select the first narrative theme for spread/counter tools
                    if let Some(first) = result.narratives.first() {
                        spread_theme.set(first.theme.clone());
                        counter_theme.set(first.theme.clone());
                    }
                    analysis.set(Some(result));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Analysis failed: {e}")));
                }
            }
            is_analyzing.set(false);
        });
    };

    // Handle spread prediction
    let on_predict_spread = move |_| {
        let theme = spread_theme().trim().to_string();
        let platform = spread_platform().clone();
        if theme.is_empty() {
            error_msg.set(Some("Select a narrative theme first.".to_string()));
            return;
        }

        is_predicting.set(true);
        error_msg.set(None);

        spawn(async move {
            match predict_spread(theme, platform).await {
                Ok(prediction) => {
                    spread_result.set(Some(prediction));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Spread prediction failed: {e}")));
                }
            }
            is_predicting.set(false);
        });
    };

    // Handle counter-narrative
    let on_counter = move |_| {
        let theme = counter_theme().trim().to_string();
        if theme.is_empty() {
            error_msg.set(Some("Select a narrative theme first.".to_string()));
            return;
        }

        is_countering.set(true);
        error_msg.set(None);

        spawn(async move {
            match suggest_counter_narrative(theme).await {
                Ok(suggestions) => {
                    counter_results.set(Some(suggestions));
                }
                Err(e) => {
                    error_msg.set(Some(format!("Counter-narrative generation failed: {e}")));
                }
            }
            is_countering.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Narrative Contagion Model" }
                p { class: "text-base-content/70",
                    "Analyze how political narratives spread, predict viral potential, and develop counter-messaging strategies."
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
                    h2 { class: "card-title text-lg", "Input Text" }
                    p { class: "text-sm text-base-content/60",
                        "Paste a news article, social media post, speech excerpt, or any political text to analyze."
                    }
                    textarea {
                        class: "textarea textarea-bordered w-full",
                        placeholder: "Enter political text to analyze for narrative patterns...",
                        rows: "6",
                        value: "{input_text}",
                        oninput: move |evt| input_text.set(evt.value()),
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: *is_analyzing.read(),
                        onclick: on_analyze,
                        if *is_analyzing.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Analyzing..."
                        } else {
                            "Analyze Narratives"
                        }
                    }
                }
            }

            // Loading spinner
            if *is_analyzing.read() {
                div { class: "flex justify-center py-8",
                    LoadingSpinner {}
                }
            }

            // Results
            if let Some(result) = analysis() {
                // Narrative cards
                div { class: "space-y-4",
                    h2 { class: "text-2xl font-semibold", "Detected Narratives" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        for narrative in result.narratives.iter() {
                            NarrativeCard { narrative: narrative.clone() }
                        }
                    }
                }

                // Spread prediction section
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-4",
                        h2 { class: "card-title text-lg", "Spread Prediction" }
                        p { class: "text-sm text-base-content/60", "{result.spread_prediction}" }

                        div { class: "divider", "Deep Dive" }

                        div { class: "flex flex-col sm:flex-row gap-3",
                            div { class: "form-control flex-1",
                                label { class: "label",
                                    span { class: "label-text text-sm", "Narrative Theme" }
                                }
                                select {
                                    class: "select select-bordered select-sm w-full",
                                    value: "{spread_theme}",
                                    onchange: move |evt: Event<FormData>| spread_theme.set(evt.value()),
                                    for n in result.narratives.iter() {
                                        option { value: "{n.theme}", "{n.theme}" }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text text-sm", "Platform" }
                                }
                                select {
                                    class: "select select-bordered select-sm",
                                    value: "{spread_platform}",
                                    onchange: move |evt: Event<FormData>| spread_platform.set(evt.value()),
                                    for platform in PLATFORM_OPTIONS {
                                        option { value: "{platform}", "{platform}" }
                                    }
                                }
                            }
                            div { class: "form-control justify-end",
                                button {
                                    class: "btn btn-secondary btn-sm",
                                    disabled: *is_predicting.read(),
                                    onclick: on_predict_spread,
                                    if *is_predicting.read() {
                                        span { class: "loading loading-spinner loading-xs" }
                                        "Predicting..."
                                    } else {
                                        "Predict Spread"
                                    }
                                }
                            }
                        }

                        if let Some(prediction) = spread_result() {
                            div { class: "bg-base-200 rounded-lg p-4 mt-2",
                                pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                    "{prediction}"
                                }
                            }
                        }
                    }
                }

                // Counter-narrative section
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body space-y-4",
                        h2 { class: "card-title text-lg", "Counter-Narrative Suggestions" }

                        div { class: "flex flex-col sm:flex-row gap-3",
                            div { class: "form-control flex-1",
                                label { class: "label",
                                    span { class: "label-text text-sm", "Narrative Theme" }
                                }
                                select {
                                    class: "select select-bordered select-sm w-full",
                                    value: "{counter_theme}",
                                    onchange: move |evt: Event<FormData>| counter_theme.set(evt.value()),
                                    for n in result.narratives.iter() {
                                        option { value: "{n.theme}", "{n.theme}" }
                                    }
                                }
                            }
                            div { class: "form-control justify-end",
                                button {
                                    class: "btn btn-accent btn-sm",
                                    disabled: *is_countering.read(),
                                    onclick: on_counter,
                                    if *is_countering.read() {
                                        span { class: "loading loading-spinner loading-xs" }
                                        "Generating..."
                                    } else {
                                        "Suggest Counter-Narratives"
                                    }
                                }
                            }
                        }

                        if let Some(ref suggestions) = counter_results() {
                            div { class: "space-y-2 mt-2",
                                for (idx, suggestion) in suggestions.iter().enumerate() {
                                    div { class: "flex gap-3 items-start bg-base-200 rounded-lg p-3",
                                        span { class: "badge badge-primary badge-sm mt-0.5",
                                            "{idx + 1}"
                                        }
                                        p { class: "text-sm", "{suggestion}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Recommended responses from initial analysis
                if !result.recommended_responses.is_empty() {
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            h2 { class: "card-title text-lg", "Recommended Responses" }
                            ul { class: "list-disc list-inside space-y-1",
                                for response in result.recommended_responses.iter() {
                                    li { class: "text-sm", "{response}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Card displaying a single detected narrative.
#[component]
fn NarrativeCard(narrative: Narrative) -> Element {
    let virality_pct = (narrative.virality_score * 100.0) as u32;
    let virality_color = if narrative.virality_score >= 0.7 {
        "progress-error"
    } else if narrative.virality_score >= 0.4 {
        "progress-warning"
    } else {
        "progress-success"
    };

    rsx! {
        div { class: "card bg-base-100 shadow-sm border border-base-300",
            div { class: "card-body space-y-3",
                h3 { class: "card-title text-base", "{narrative.theme}" }

                div { class: "space-y-1",
                    p { class: "text-xs font-semibold text-base-content/50 uppercase tracking-wide",
                        "Framing"
                    }
                    p { class: "text-sm", "{narrative.framing}" }
                }

                div { class: "space-y-1",
                    p { class: "text-xs font-semibold text-base-content/50 uppercase tracking-wide",
                        "Target Audience"
                    }
                    p { class: "text-sm", "{narrative.target_audience}" }
                }

                div { class: "space-y-1",
                    div { class: "flex items-center justify-between",
                        p { class: "text-xs font-semibold text-base-content/50 uppercase tracking-wide",
                            "Virality Score"
                        }
                        span { class: "text-sm font-mono", "{virality_pct}%" }
                    }
                    progress {
                        class: "progress {virality_color} w-full",
                        value: "{virality_pct}",
                        max: "100",
                    }
                }

                if !narrative.emotional_triggers.is_empty() {
                    div { class: "space-y-1",
                        p { class: "text-xs font-semibold text-base-content/50 uppercase tracking-wide",
                            "Emotional Triggers"
                        }
                        div { class: "flex flex-wrap gap-1",
                            for trigger in narrative.emotional_triggers.iter() {
                                span { class: "badge badge-outline badge-sm", "{trigger}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
