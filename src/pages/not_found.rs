use dioxus::prelude::*;

/// 404 Not Found page.
#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        div { class: "hero min-h-screen bg-base-200",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "404" }
                    p { class: "py-6", "Page not found: /{path}" }
                    Link {
                        to: crate::app::Route::LandingPage {},
                        class: "btn btn-primary",
                        "Go Home"
                    }
                }
            }
        }
    }
}
