use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::multilingual::{
    adapt_messaging, get_supported_languages, translate_content, AdaptedMessaging, SupportedLanguage,
    Translation,
};

/// Fallback languages used when the server call has not resolved yet.
const FALLBACK_LANGUAGES: &[(&str, &str)] = &[
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("zh", "Mandarin"),
    ("ar", "Arabic"),
    ("hi", "Hindi"),
    ("pt", "Portuguese"),
    ("de", "German"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
];

#[component]
pub fn MultilingualPage() -> Element {
    // Fetch supported languages on mount
    let languages_resource = use_server_future(get_supported_languages)?;

    let languages: Vec<SupportedLanguage> = match &*languages_resource.read() {
        Some(Ok(langs)) => langs.clone(),
        _ => FALLBACK_LANGUAGES
            .iter()
            .map(|(code, name)| SupportedLanguage {
                code: code.to_string(),
                name: name.to_string(),
            })
            .collect(),
    };

    // --- Translation form state ---
    let mut source_text = use_signal(String::new);
    let mut source_language = use_signal(|| "en".to_string());
    let mut target_language = use_signal(|| "es".to_string());
    let mut translation_context = use_signal(String::new);
    let mut is_translating = use_signal(|| false);
    let mut translation_result = use_signal(|| Option::<Translation>::None);
    let mut translation_error = use_signal(|| Option::<String>::None);

    // --- Adapt messaging state ---
    let mut adapt_text = use_signal(String::new);
    let mut adapt_culture = use_signal(String::new);
    let mut adapt_tone = use_signal(|| "Professional".to_string());
    let mut is_adapting = use_signal(|| false);
    let mut adapt_result = use_signal(|| Option::<AdaptedMessaging>::None);
    let mut adapt_error = use_signal(|| Option::<String>::None);

    // Handle translate
    let langs_for_translate = languages.clone();
    let on_translate = move |_| {
        let text = source_text().trim().to_string();
        let src_lang_code = source_language();
        let tgt_lang_code = target_language();
        let ctx = translation_context().trim().to_string();

        if text.is_empty() {
            translation_error.set(Some("Source text is required.".to_string()));
            return;
        }

        if src_lang_code == tgt_lang_code {
            translation_error.set(Some(
                "Source and target languages must be different.".to_string(),
            ));
            return;
        }

        // Resolve language names for display
        let src_name = langs_for_translate
            .iter()
            .find(|l| l.code == src_lang_code)
            .map(|l| l.name.clone())
            .unwrap_or(src_lang_code.clone());
        let tgt_name = langs_for_translate
            .iter()
            .find(|l| l.code == tgt_lang_code)
            .map(|l| l.name.clone())
            .unwrap_or(tgt_lang_code.clone());

        let context_opt = if ctx.is_empty() { None } else { Some(ctx) };

        is_translating.set(true);
        translation_error.set(None);

        spawn(async move {
            match translate_content(text, src_name, tgt_name, context_opt).await {
                Ok(result) => {
                    translation_result.set(Some(result));
                }
                Err(e) => {
                    translation_error.set(Some(format!("Translation failed: {e}")));
                }
            }
            is_translating.set(false);
        });
    };

    // Handle adapt messaging
    let on_adapt = move |_| {
        let text = adapt_text().trim().to_string();
        let culture = adapt_culture().trim().to_string();
        let tone = adapt_tone();

        if text.is_empty() {
            adapt_error.set(Some("Text is required.".to_string()));
            return;
        }
        if culture.is_empty() {
            adapt_error.set(Some("Target culture is required.".to_string()));
            return;
        }

        is_adapting.set(true);
        adapt_error.set(None);

        spawn(async move {
            match adapt_messaging(text, culture, tone).await {
                Ok(result) => {
                    adapt_result.set(Some(result));
                }
                Err(e) => {
                    adapt_error.set(Some(format!("Adaptation failed: {e}")));
                }
            }
            is_adapting.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Multilingual Outreach" }
                p { class: "text-slate-400",
                    "Translate and culturally adapt campaign materials for multilingual communities while preserving tone and intent."
                }
            }

            // ---- Translation Section ----
            div { class: "glass-card gradient-border",
                div { class: "card-body space-y-4",
                    h2 { class: "card-title text-lg", "Translate Content" }

                    // Error alert
                    if let Some(err) = translation_error() {
                        div { class: "alert alert-error shadow-sm",
                            span { "{err}" }
                            button {
                                class: "btn btn-ghost btn-xs",
                                onclick: move |_| translation_error.set(None),
                                "Dismiss"
                            }
                        }
                    }

                    // Source text
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Source Text" }
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Enter text to translate...",
                            rows: "5",
                            value: "{source_text}",
                            oninput: move |evt| source_text.set(evt.value()),
                        }
                    }

                    // Language selectors
                    div { class: "flex flex-col sm:flex-row gap-4",
                        div { class: "form-control flex-1",
                            label { class: "label",
                                span { class: "label-text font-medium", "Source Language" }
                            }
                            select {
                                class: "select select-bordered w-full",
                                value: "{source_language}",
                                onchange: move |evt: Event<FormData>| source_language.set(evt.value()),
                                for lang in &languages {
                                    option { value: "{lang.code}", "{lang.name}" }
                                }
                            }
                        }
                        div { class: "form-control flex-1",
                            label { class: "label",
                                span { class: "label-text font-medium", "Target Language" }
                            }
                            select {
                                class: "select select-bordered w-full",
                                value: "{target_language}",
                                onchange: move |evt: Event<FormData>| target_language.set(evt.value()),
                                for lang in &languages {
                                    option { value: "{lang.code}", "{lang.name}" }
                                }
                            }
                        }
                    }

                    // Context
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Context" }
                            span { class: "label-text-alt text-slate-500", "Optional" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "e.g., campaign rally speech, social media post, formal letter",
                            value: "{translation_context}",
                            oninput: move |evt| translation_context.set(evt.value()),
                        }
                    }

                    // Translate button
                    button {
                        class: "btn btn-primary",
                        disabled: *is_translating.read(),
                        onclick: on_translate,
                        if *is_translating.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Translating..."
                        } else {
                            "Translate"
                        }
                    }

                    // Translation results
                    if *is_translating.read() {
                        div { class: "flex items-center justify-center py-8",
                            div { class: "text-center space-y-4",
                                LoadingSpinner {}
                                p { class: "text-slate-400", "Translating content..." }
                            }
                        }
                    } else if let Some(result) = translation_result() {
                        div { class: "space-y-4 mt-4",
                            // Translated text card
                            div { class: "card bg-slate-800/30",
                                div { class: "p-6",
                                    div { class: "flex items-center justify-between mb-2",
                                        h3 { class: "font-semibold",
                                            "{result.source_language} → {result.target_language}"
                                        }
                                    }
                                    pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                        "{result.translated_text}"
                                    }
                                }
                            }

                            // Cultural notes
                            if !result.cultural_notes.is_empty() {
                                div { class: "space-y-2",
                                    h3 { class: "font-semibold text-sm", "Cultural Notes" }
                                    for note in &result.cultural_notes {
                                        div { class: "alert alert-info shadow-sm py-2",
                                            svg {
                                                class: "stroke-current shrink-0 w-5 h-5",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                                }
                                            }
                                            span { class: "text-sm", "{note}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ---- Adapt Messaging Section ----
            div { class: "glass-card gradient-border",
                div { class: "card-body space-y-4",
                    h2 { class: "card-title text-lg", "Adapt Messaging" }
                    p { class: "text-slate-400 text-sm",
                        "Culturally adapt messaging beyond translation — adjust framing, references, and tone for a specific cultural audience."
                    }

                    // Error alert
                    if let Some(err) = adapt_error() {
                        div { class: "alert alert-error shadow-sm",
                            span { "{err}" }
                            button {
                                class: "btn btn-ghost btn-xs",
                                onclick: move |_| adapt_error.set(None),
                                "Dismiss"
                            }
                        }
                    }

                    // Text to adapt
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "Text to Adapt" }
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Enter political messaging to adapt...",
                            rows: "4",
                            value: "{adapt_text}",
                            oninput: move |evt| adapt_text.set(evt.value()),
                        }
                    }

                    // Culture context and tone
                    div { class: "flex flex-col sm:flex-row gap-4",
                        div { class: "form-control flex-1",
                            label { class: "label",
                                span { class: "label-text font-medium", "Target Culture" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "text",
                                placeholder: "e.g., Latino community, East Asian diaspora",
                                value: "{adapt_culture}",
                                oninput: move |evt| adapt_culture.set(evt.value()),
                            }
                        }
                        div { class: "form-control flex-1",
                            label { class: "label",
                                span { class: "label-text font-medium", "Tone" }
                            }
                            select {
                                class: "select select-bordered w-full",
                                value: "{adapt_tone}",
                                onchange: move |evt: Event<FormData>| adapt_tone.set(evt.value()),
                                option { value: "Professional", "Professional" }
                                option { value: "Casual", "Casual" }
                                option { value: "Inspirational", "Inspirational" }
                                option { value: "Empathetic", "Empathetic" }
                                option { value: "Urgent", "Urgent" }
                                option { value: "Community-Oriented", "Community-Oriented" }
                            }
                        }
                    }

                    // Adapt button
                    button {
                        class: "btn btn-secondary",
                        disabled: *is_adapting.read(),
                        onclick: on_adapt,
                        if *is_adapting.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Adapting..."
                        } else {
                            "Adapt Messaging"
                        }
                    }

                    // Adaptation results
                    if *is_adapting.read() {
                        div { class: "flex items-center justify-center py-8",
                            div { class: "text-center space-y-4",
                                LoadingSpinner {}
                                p { class: "text-slate-400", "Adapting messaging..." }
                            }
                        }
                    } else if let Some(result) = adapt_result() {
                        div { class: "space-y-4 mt-4",
                            // Adapted text card
                            div { class: "card bg-slate-800/30",
                                div { class: "p-6",
                                    div { class: "flex items-center gap-2 mb-2",
                                        h3 { class: "font-semibold", "Adapted for: {result.target_culture}" }
                                        span { class: "badge badge-outline badge-sm", "{result.tone}" }
                                    }
                                    pre { class: "whitespace-pre-wrap text-sm font-sans leading-relaxed",
                                        "{result.adapted_text}"
                                    }
                                }
                            }

                            // Adaptation notes
                            if !result.adaptation_notes.is_empty() {
                                div { class: "space-y-2",
                                    h3 { class: "font-semibold text-sm", "Adaptation Notes" }
                                    for note in &result.adaptation_notes {
                                        div { class: "alert alert-info shadow-sm py-2",
                                            svg {
                                                class: "stroke-current shrink-0 w-5 h-5",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                                }
                                            }
                                            span { class: "text-sm", "{note}" }
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
