use dioxus::prelude::*;

/// Modal dialog with backdrop click to close.
#[component]
pub fn Modal(
    open: bool,
    on_close: EventHandler<()>,
    title: String,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div { class: "modal modal-open",
            // Backdrop
            div {
                class: "modal-backdrop",
                onclick: move |_| on_close.call(()),
            }
            div { class: "modal-box",
                // Close button
                button {
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    onclick: move |_| on_close.call(()),
                    "X"
                }
                h3 { class: "font-bold text-lg", "{title}" }
                div { class: "py-4", {children} }
            }
        }
    }
}
