use dioxus::prelude::*;

/// Generic card wrapper with glassmorphism and gradient border.
#[component]
pub fn Card(
    title: String,
    children: Element,
    #[props(default = String::new())] class: String,
    #[props(default = false)] compact: bool,
) -> Element {
    let padding = if compact { "p-4" } else { "p-6" };

    rsx! {
        div { class: "glass-card gradient-border {class}",
            div { class: "{padding}",
                h2 { class: "text-lg font-semibold text-slate-100 mb-4", "{title}" }
                {children}
            }
        }
    }
}
