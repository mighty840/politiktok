use dioxus::prelude::*;

/// Centered loading spinner with indigo color.
#[component]
pub fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "flex items-center justify-center p-8",
            div { class: "loading-spinner" }
        }
    }
}

/// Skeleton placeholder with shimmer animation.
#[component]
pub fn Skeleton(
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "1rem".to_string())] height: String,
) -> Element {
    rsx! {
        div {
            class: "skeleton-shimmer",
            style: "width: {width}; height: {height};",
        }
    }
}
