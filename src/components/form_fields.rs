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
    let input_class = if has_error {
        "input input-bordered input-error w-full"
    } else {
        "input input-bordered w-full"
    };

    rsx! {
        fieldset { class: "fieldset mb-3",
            label { class: "fieldset-legend text-sm font-medium", "{label}" }
            input {
                r#type: "text",
                class: "{input_class}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt: Event<FormData>| on_change.call(evt.value()),
            }
            if let Some(err) = &error {
                p { class: "text-error text-xs mt-1", "{err}" }
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
            label { class: "fieldset-legend text-sm font-medium", "{label}" }
            select {
                class: "select select-bordered w-full",
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
            label { class: "fieldset-legend text-sm font-medium", "{label}" }
            textarea {
                class: "textarea textarea-bordered w-full",
                rows: "{rows}",
                value: "{value}",
                oninput: move |evt: Event<FormData>| on_change.call(evt.value()),
            }
        }
    }
}
