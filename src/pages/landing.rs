use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen",

            // Hero section
            section { class: "animated-gradient-bg relative overflow-hidden min-h-[90vh] flex items-center justify-center",
                // Decorative blurred circles
                div {
                    class: "absolute top-20 left-10 w-72 h-72 rounded-full opacity-20 blur-3xl pointer-events-none",
                    style: "background: #6366f1;",
                }
                div {
                    class: "absolute bottom-20 right-10 w-96 h-96 rounded-full opacity-15 blur-3xl pointer-events-none",
                    style: "background: #ec4899;",
                }
                div {
                    class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] rounded-full opacity-10 blur-3xl pointer-events-none",
                    style: "background: #8b5cf6;",
                }

                div { class: "relative z-10 text-center px-6 max-w-4xl mx-auto animate-fade-in",
                    // Pill badge
                    span {
                        class: "inline-flex items-center px-4 py-1.5 rounded-full text-sm font-medium mb-8 border",
                        style: "background: rgba(99,102,241,0.1); border-color: rgba(99,102,241,0.3); color: #818cf8;",
                        "AI-Powered Campaign Intelligence"
                    }

                    // Title with gradient text
                    h1 {
                        class: "text-5xl sm:text-6xl lg:text-7xl font-extrabold mb-6 tracking-tight",
                        style: "background: linear-gradient(135deg, #e2e8f0, #818cf8, #c084fc); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;",
                        "PolitikTok"
                    }

                    p { class: "text-lg sm:text-xl text-slate-400 mb-10 max-w-2xl mx-auto leading-relaxed",
                        "Manage volunteers, track sentiment, generate campaign copy, and coordinate your entire political operation — all powered by local AI."
                    }

                    // Dual CTA
                    div { class: "flex flex-col sm:flex-row gap-4 justify-center",
                        Link {
                            to: Route::LoginPage { redirect_url: "/dashboard".to_string() },
                            class: "inline-flex items-center justify-center px-8 py-3.5 rounded-xl text-white font-semibold text-lg btn-scale",
                            style: "background: linear-gradient(135deg, #6366f1, #8b5cf6);",
                            "Get Started"
                        }
                        Link {
                            to: Route::LoginPage { redirect_url: "/dashboard".to_string() },
                            class: "inline-flex items-center justify-center px-8 py-3.5 rounded-xl font-semibold text-lg border btn-scale",
                            style: "border-color: rgba(148,163,184,0.2); color: #94a3b8;",
                            "Learn More"
                        }
                    }
                }
            }

            // Stats bar
            section { class: "py-12 px-6",
                style: "background: var(--color-bg-secondary);",
                div { class: "max-w-5xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-8 text-center",
                    StatItem { value: "26", label: "AI Modules" }
                    StatItem { value: "100%", label: "Open Source" }
                    StatItem { value: "Real-time", label: "Monitoring" }
                    StatItem { value: "Multi", label: "Language" }
                }
            }

            // Features grid
            section { class: "py-20 px-6",
                div { class: "max-w-6xl mx-auto",
                    h2 { class: "text-3xl sm:text-4xl font-bold text-center mb-4 text-slate-100",
                        "Everything you need to win"
                    }
                    p { class: "text-slate-400 text-center mb-16 max-w-2xl mx-auto",
                        "26 integrated AI modules covering every aspect of modern campaign operations."
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 stagger-enter",
                        LandingFeatureCard {
                            icon: "👥",
                            title: "Volunteer Management",
                            description: "AI-driven matching, churn prediction, and automated outreach for your volunteer network."
                        }
                        LandingFeatureCard {
                            icon: "📊",
                            title: "Sentiment Analysis",
                            description: "Monitor public sentiment in real-time across social media and news sources."
                        }
                        LandingFeatureCard {
                            icon: "✍️",
                            title: "Campaign Copy",
                            description: "Generate targeted messaging with AI-assisted content creation and A/B testing."
                        }
                        LandingFeatureCard {
                            icon: "🔍",
                            title: "Opposition Research",
                            description: "Track opponents and surface relevant policy differences automatically."
                        }
                        LandingFeatureCard {
                            icon: "💰",
                            title: "Fundraising Intelligence",
                            description: "Optimize donation strategies with predictive analytics and donor insights."
                        }
                        LandingFeatureCard {
                            icon: "🤝",
                            title: "Coalition Building",
                            description: "Identify potential allies and build coalitions with data-driven recommendations."
                        }
                    }
                }
            }

            // CTA section
            section {
                class: "relative py-20 px-6 text-center overflow-hidden",
                style: "background: linear-gradient(135deg, rgba(99,102,241,0.15), rgba(139,92,246,0.1));",
                div { class: "relative z-10 max-w-2xl mx-auto",
                    h2 { class: "text-3xl sm:text-4xl font-bold mb-4 text-slate-100",
                        "Ready to Transform Your Campaign?"
                    }
                    p { class: "text-lg text-slate-400 mb-8",
                        "Join campaigns already using PolitikTok to win."
                    }
                    Link {
                        to: Route::LoginPage { redirect_url: "/dashboard".to_string() },
                        class: "inline-flex items-center justify-center px-8 py-3.5 rounded-xl text-white font-semibold text-lg btn-scale",
                        style: "background: linear-gradient(135deg, #f43f5e, #e11d48);",
                        "Start Now"
                    }
                }
            }

            // Footer
            footer { class: "py-8 px-6 text-center border-t",
                style: "border-color: var(--glass-border);",
                p { class: "text-sm text-slate-500",
                    "PolitikTok — Open-source AI-powered campaign intelligence"
                }
            }
        }
    }
}

#[component]
fn StatItem(value: &'static str, label: &'static str) -> Element {
    rsx! {
        div {
            p {
                class: "text-2xl sm:text-3xl font-bold mb-1",
                style: "background: linear-gradient(135deg, #818cf8, #c084fc); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;",
                "{value}"
            }
            p { class: "text-sm text-slate-400", "{label}" }
        }
    }
}

#[component]
fn LandingFeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> Element {
    rsx! {
        div { class: "glass-card gradient-border p-6",
            div {
                class: "flex items-center justify-center w-12 h-12 rounded-xl mb-4 text-2xl",
                style: "background: rgba(99,102,241,0.1);",
                "{icon}"
            }
            h3 { class: "text-lg font-semibold text-slate-100 mb-2", "{title}" }
            p { class: "text-slate-400 text-sm leading-relaxed", "{description}" }
        }
    }
}
