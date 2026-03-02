use dioxus::prelude::*;

/// Labeled text input field with optional error display.
#[component]
pub fn TextInput(
    label: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = String::new())] placeholder: String,
    #[props(default = None)] error: Option<String>,
) -> Element {
    let has_error = error.is_some();
    let border_class = if has_error {
        "border-red-500 focus:border-red-500"
    } else {
        "border-slate-700 focus:border-indigo-500"
    };

    rsx! {
        fieldset { class: "fieldset mb-3",
            label { class: "fieldset-legend text-sm font-medium text-slate-300", "{label}" }
            input {
                r#type: "text",
                class: "w-full px-3 py-2.5 bg-slate-800/50 {border_class} border rounded-xl text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/50 transition-colors",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt: Event<FormData>| on_change.call(evt.value()),
            }
            if let Some(err) = &error {
                p { class: "text-red-400 text-xs mt-1", "{err}" }
            }
        }
    }
}

/// Labeled select dropdown.
#[component]
pub fn SelectInput(
    label: String,
    value: String,
    options: Vec<(String, String)>,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        fieldset { class: "fieldset mb-3",
            label { class: "fieldset-legend text-sm font-medium text-slate-300", "{label}" }
            select {
                class: "w-full px-3 py-2.5 bg-slate-800/50 border border-slate-700 rounded-xl text-slate-200 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500/50 transition-colors",
                value: "{value}",
                onchange: move |evt: Event<FormData>| on_change.call(evt.value()),
                for (val, display) in &options {
                    option { value: "{val}", "{display}" }
                }
            }
        }
    }
}

/// Labeled multi-line textarea.
#[component]
pub fn TextArea(
    label: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = 4)] rows: u32,
) -> Element {
    rsx! {
        fieldset { class: "fieldset mb-3",
            label { class: "fieldset-legend text-sm font-medium text-slate-300", "{label}" }
            textarea {
                class: "w-full px-3 py-2.5 bg-slate-800/50 border border-slate-700 rounded-xl text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500/50 transition-colors",
                rows: "{rows}",
                value: "{value}",
                oninput: move |evt: Event<FormData>| on_change.call(evt.value()),
            }
        }
    }
}
