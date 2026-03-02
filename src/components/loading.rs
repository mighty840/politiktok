use dioxus::prelude::*;

/// Centered loading spinner using DaisyUI.
#[component]
pub fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "flex items-center justify-center p-8",
            span { class: "loading loading-spinner loading-lg text-primary" }
        }
    }
}

/// Skeleton placeholder for content that is still loading.
#[component]
pub fn Skeleton(
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "1rem".to_string())] height: String,
) -> Element {
    rsx! {
        div {
            class: "skeleton",
            style: "width: {width}; height: {height};",
        }
    }
}
