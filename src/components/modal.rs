use dioxus::prelude::*;

/// Modal dialog with blurred backdrop.
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
        div { class: "modal-blur-backdrop",
            // Backdrop click
            div {
                class: "absolute inset-0",
                onclick: move |_| on_close.call(()),
            }
            div { class: "glass-card animate-scale-in relative max-w-lg w-full mx-4 p-6",
                // Close button
                button {
                    class: "absolute right-3 top-3 w-8 h-8 flex items-center justify-center rounded-full text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-colors",
                    onclick: move |_| on_close.call(()),
                    "\u{2715}"
                }
                h3 { class: "text-lg font-semibold text-slate-100 mb-4", "{title}" }
                div { {children} }
            }
        }
    }
}
