use dioxus::prelude::*;

/// Page header with title, optional subtitle, and optional action buttons.
#[component]
pub fn PageHeader(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = None)] actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 mb-6 sm:flex-row sm:items-center sm:justify-between",
            div {
                h1 { class: "text-2xl font-bold", "{title}" }
                if let Some(sub) = &subtitle {
                    p { class: "text-sm text-base-content/60 mt-1", "{sub}" }
                }
            }
            if let Some(act) = actions {
                div { class: "flex gap-2 mt-2 sm:mt-0", {act} }
            }
        }
    }
}
