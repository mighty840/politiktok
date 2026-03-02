use dioxus::prelude::*;

/// Generic card wrapper with optional title and custom CSS class.
#[component]
pub fn Card(
    title: String,
    children: Element,
    #[props(default = String::new())] class: String,
) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-sm {class}",
            div { class: "card-body",
                h2 { class: "card-title", "{title}" }
                {children}
            }
        }
    }
}
