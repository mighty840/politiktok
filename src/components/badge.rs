use dioxus::prelude::*;

/// Pill-shaped status badge with color variants.
///
/// `variant` must be one of: "active", "inactive", "error", "warning".
#[component]
pub fn Badge(
    label: String,
    #[props(default = "active".to_string())] variant: String,
) -> Element {
    let badge_class = match variant.as_str() {
        "inactive" => "bg-slate-500/10 text-slate-400",
        "error" => "bg-red-500/10 text-red-400",
        "warning" => "bg-amber-500/10 text-amber-400",
        _ => "bg-emerald-500/10 text-emerald-400",
    };

    rsx! {
        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {badge_class}",
            "{label}"
        }
    }
}
