use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen bg-base-200",
            // Hero section
            div { class: "hero min-h-[70vh] bg-base-100",
                div { class: "hero-content text-center",
                    div { class: "max-w-2xl",
                        h1 { class: "text-5xl font-bold", "PolitikTok" }
                        p { class: "py-6 text-lg text-base-content/70",
                            "AI-powered campaign intelligence platform. Manage volunteers, track sentiment, generate campaign copy, and coordinate your entire political operation from one place."
                        }
                        Link {
                            to: Route::LoginPage { redirect_url: "/dashboard".to_string() },
                            class: "btn btn-primary btn-lg",
                            "Get Started"
                        }
                    }
                }
            }

            // Feature highlights
            div { class: "container mx-auto px-4 py-16",
                h2 { class: "text-3xl font-bold text-center mb-12", "Platform Features" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    FeatureCard {
                        title: "Volunteer Management",
                        description: "Match and coordinate volunteers with AI-driven task assignment and skills matching."
                    }
                    FeatureCard {
                        title: "Sentiment Analysis",
                        description: "Monitor public sentiment in real-time across social media and news sources."
                    }
                    FeatureCard {
                        title: "Campaign Copy",
                        description: "Generate targeted campaign messaging with AI-assisted content creation."
                    }
                    FeatureCard {
                        title: "Opposition Research",
                        description: "Track opponents and surface relevant policy differences automatically."
                    }
                    FeatureCard {
                        title: "Fundraising Intelligence",
                        description: "Optimize donation strategies with predictive analytics and donor insights."
                    }
                    FeatureCard {
                        title: "Coalition Building",
                        description: "Identify potential allies and build coalitions with data-driven recommendations."
                    }
                }
            }

            // CTA section
            div { class: "bg-primary text-primary-content py-16 text-center",
                h2 { class: "text-3xl font-bold mb-4", "Ready to Transform Your Campaign?" }
                p { class: "mb-8 text-lg opacity-90", "Join campaigns already using PolitikTok to win." }
                Link {
                    to: Route::LoginPage { redirect_url: "/dashboard".to_string() },
                    class: "btn btn-secondary btn-lg",
                    "Log In Now"
                }
            }
        }
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-md",
            div { class: "card-body",
                h3 { class: "card-title", "{title}" }
                p { class: "text-base-content/70", "{description}" }
            }
        }
    }
}
