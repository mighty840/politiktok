use dioxus::prelude::*;

/// Alert banner with glassmorphism and left border accent.
///
/// `variant` must be one of: "info", "warning", "error", "success".
#[component]
pub fn AlertBanner(
    #[props(default = "info".to_string())] variant: String,
    message: String,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    let border_color = match variant.as_str() {
        "warning" => "border-l-amber-500",
        "error" => "border-l-red-500",
        "success" => "border-l-emerald-500",
        _ => "border-l-indigo-500",
    };

    rsx! {
        div { class: "glass-card mb-4 px-4 py-3 border-l-4 {border_color} flex items-center justify-between",
            span { class: "text-sm text-slate-200", "{message}" }
            if let Some(on_dismiss) = on_dismiss {
                button {
                    class: "w-6 h-6 flex items-center justify-center rounded-full text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-colors text-xs",
                    onclick: move |_| on_dismiss.call(()),
                    "\u{2715}"
                }
            }
        }
    }
}
