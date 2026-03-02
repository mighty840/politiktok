use dioxus::prelude::*;

/// Status badge with color variants.
///
/// `variant` must be one of: "active", "inactive", "error", "warning".
#[component]
pub fn Badge(
    label: String,
    #[props(default = "active".to_string())] variant: String,
) -> Element {
    let badge_class = match variant.as_str() {
        "inactive" => "badge badge-ghost",
        "error" => "badge badge-error",
        "warning" => "badge badge-warning",
        _ => "badge badge-success",
    };

    rsx! {
        span { class: "{badge_class}", "{label}" }
    }
}
