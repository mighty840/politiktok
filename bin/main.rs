#![allow(non_snake_case)]

#[allow(clippy::expect_used)]
fn main() {
    dioxus_logger::init(tracing::Level::DEBUG).expect("Failed to init logger");

    #[cfg(feature = "web")]
    {
        dioxus::web::launch::launch_cfg(politiktok::App, dioxus::web::Config::new().hydrate(true));
    }

    #[cfg(feature = "server")]
    {
        politiktok::infrastructure::server_start(politiktok::App)
            .map_err(|e| tracing::error!("Unable to start server: {e}"))
            .expect("Server start failed")
    }
}
