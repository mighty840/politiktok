use dioxus::prelude::*;

/// Login page — redirects to Keycloak auth endpoint.
#[component]
pub fn LoginPage(redirect_url: String) -> Element {
    let redirect = if redirect_url.is_empty() {
        "/dashboard".to_string()
    } else {
        redirect_url
    };

    // Redirect to server-side auth handler
    use_effect(move || {
        let nav = navigator();
        nav.push(NavigationTarget::<crate::app::Route>::External(format!(
            "/auth?redirect_url={redirect}"
        )));
    });

    rsx! {
        div { class: "loading-page",
            span { class: "loading loading-spinner loading-lg" }
            p { "Redirecting to login..." }
        }
    }
}
