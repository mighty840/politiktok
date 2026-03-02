use dioxus::prelude::*;

/// Alert banner with dismiss support.
///
/// `variant` must be one of: "info", "warning", "error", "success".
#[component]
pub fn AlertBanner(
    #[props(default = "info".to_string())] variant: String,
    message: String,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    let alert_class = match variant.as_str() {
        "warning" => "alert alert-warning",
        "error" => "alert alert-error",
        "success" => "alert alert-success",
        _ => "alert alert-info",
    };

    rsx! {
        div { class: "{alert_class} mb-4",
            span { "{message}" }
            if let Some(on_dismiss) = on_dismiss {
                button {
                    class: "btn btn-sm btn-ghost",
                    onclick: move |_| on_dismiss.call(()),
                    "X"
                }
            }
        }
    }
}
