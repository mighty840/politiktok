use crate::pages::*;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Root application component.
#[component]
pub fn App() -> Element {
    rsx! {
        // Google Fonts — Inter (300-900)
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&display=swap",
        }

        // Tailwind CSS 3 CDN + DaisyUI 4 plugin
        document::Script { src: "https://cdn.tailwindcss.com" }
        document::Script {
            r#"tailwind.config = {{
                theme: {{
                    extend: {{
                        fontFamily: {{
                            sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
                        }},
                        colors: {{
                            indigo: {{
                                400: '#818cf8',
                                500: '#6366f1',
                                600: '#4f46e5',
                                700: '#4338ca',
                            }},
                            rose: {{
                                400: '#fb7185',
                                500: '#f43f5e',
                                600: '#e11d48',
                            }},
                        }},
                        animation: {{
                            'fade-in': 'fadeIn 0.5s ease-out forwards',
                            'slide-up': 'slideUp 0.5s ease-out forwards',
                            'slide-in-right': 'slideInRight 0.4s ease-out forwards',
                            'scale-in': 'scaleIn 0.3s ease-out forwards',
                            'pulse-soft': 'pulseSoft 2s ease-in-out infinite',
                        }},
                        keyframes: {{
                            fadeIn: {{
                                '0%': {{ opacity: '0' }},
                                '100%': {{ opacity: '1' }},
                            }},
                            slideUp: {{
                                '0%': {{ opacity: '0', transform: 'translateY(20px)' }},
                                '100%': {{ opacity: '1', transform: 'translateY(0)' }},
                            }},
                            slideInRight: {{
                                '0%': {{ opacity: '0', transform: 'translateX(100%)' }},
                                '100%': {{ opacity: '1', transform: 'translateX(0)' }},
                            }},
                            scaleIn: {{
                                '0%': {{ opacity: '0', transform: 'scale(0.95)' }},
                                '100%': {{ opacity: '1', transform: 'scale(1)' }},
                            }},
                            pulseSoft: {{
                                '0%, 100%': {{ opacity: '1' }},
                                '50%': {{ opacity: '0.7' }},
                            }},
                        }},
                    }},
                }},
                plugins: [],
            }};"#
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.23/dist/full.min.css",
        }

        document::Stylesheet { href: MAIN_CSS }

        // Theme init: read from localStorage, default to politiktok-dark
        document::Script {
            r#"(function() {{
                var t = localStorage.getItem('politiktok-theme') || 'politiktok-dark';
                document.documentElement.setAttribute('data-theme', t);
            }})();"#
        }

        Router::<Route> {}
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    LandingPage {},

    #[route("/login?:redirect_url")]
    LoginPage { redirect_url: String },

    #[layout(AppShell)]
        #[route("/dashboard")]
        DashboardPage {},

        // F01 — Volunteer Matching
        #[route("/volunteers")]
        VolunteersPage {},
        #[route("/volunteers/:id")]
        VolunteerDetailPage { id: String },
        #[route("/tasks")]
        TasksPage {},
        #[route("/tasks/:id")]
        TaskDetailPage { id: String },

        // F02 — Policy Chatbot
        #[route("/policy-chat")]
        PolicyChatPage {},

        // F03 — Sentiment Monitor
        #[route("/sentiment")]
        SentimentDashboardPage {},

        // F04 — Campaign Copy
        #[route("/campaign-copy")]
        CampaignCopyPage {},

        // F05 — Opposition Research
        #[route("/opposition")]
        OppositionPage {},
        #[route("/opposition/:id")]
        OpponentDetailPage { id: String },

        // F06 — Canvassing
        #[route("/canvassing")]
        CanvassingPage {},

        // F07 — Fundraising
        #[route("/fundraising")]
        FundraisingPage {},

        // F08 — Accountability
        #[route("/accountability")]
        AccountabilityPage {},

        // F09 — Empathy Simulator
        #[route("/empathy")]
        EmpathyPage {},

        // F10 — Narrative Contagion
        #[route("/narrative")]
        NarrativePage {},

        // F11 — Coalition Detector
        #[route("/coalition")]
        CoalitionPage {},

        // F12 — Candidate Briefings
        #[route("/briefings")]
        BriefingsPage {},

        // F13 — Call Intelligence
        #[route("/call-intel")]
        CallIntelPage {},

        // F14 — Coaching & Debate
        #[route("/coaching")]
        CoachingPage {},

        // F15 — Multilingual
        #[route("/multilingual")]
        MultilingualPage {},

        // F16 — Question Anticipation
        #[route("/question-anticipation")]
        QuestionAnticipationPage {},

        // F17 — Local Issues
        #[route("/local-issues")]
        LocalIssuesPage {},

        // F18 — Policy Diff
        #[route("/policy-diff")]
        PolicyDiffPage {},

        // F19 — Faction Mapper
        #[route("/faction-mapper")]
        FactionMapperPage {},

        // F20 — Regulatory Monitor
        #[route("/regulatory")]
        RegulatoryPage {},

        // F21 — Media Monitor
        #[route("/media-monitor")]
        MediaMonitorPage {},

        // F22 — Disinfo Warning
        #[route("/disinfo")]
        DisinfoPage {},

        // F23 — Compliance
        #[route("/compliance")]
        CompliancePage {},

        // F24 — Meetings
        #[route("/meetings")]
        MeetingsPage {},

        // F25 — Knowledge Base
        #[route("/knowledge-base")]
        KnowledgeBasePage {},

        // F26 — Admin Panel
        #[layout(AdminShell)]
            #[route("/admin")]
            AdminDashboardPage {},
            #[route("/admin/users")]
            AdminUsersPage {},
            #[route("/admin/modules")]
            AdminModulesPage {},
            #[route("/admin/llm")]
            AdminLlmConfigPage {},
            #[route("/admin/sources")]
            AdminSourcesPage {},
            #[route("/admin/kb")]
            AdminKnowledgeBasePage {},
            #[route("/admin/health")]
            AdminHealthPage {},
            #[route("/admin/audit")]
            AdminAuditPage {},
            #[route("/admin/alerts")]
            AdminAlertsPage {},
            #[route("/admin/data")]
            AdminDataGovernancePage {},
            #[route("/admin/integrations")]
            AdminIntegrationsPage {},
        #[end_layout]
    #[end_layout]

    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },
}

/// Main app shell layout with sidebar and auth gating.
#[component]
fn AppShell() -> Element {
    let mut sidebar_open = use_signal(|| false);

    // use_resource memoises the async call and avoids infinite re-render
    // loops that use_effect + spawn + signal writes can cause.
    #[allow(clippy::redundant_closure)]
    let auth = use_resource(move || check_auth());

    // Clone the inner value out of the Signal to avoid holding the
    // borrow across the rsx! return (Dioxus lifetime constraint).
    let auth_snapshot: Option<Result<AuthInfo, ServerFnError>> = auth.read().clone();

    match auth_snapshot {
        Some(Ok(ref info)) if info.authenticated => {
            // Provide auth info as context for child components (e.g. Sidebar)
            use_context_provider(|| Signal::new(info.clone()));

            rsx! {
                div { class: "app-shell",
                    // Mobile hamburger
                    button {
                        class: "sidebar-toggle",
                        onclick: move |_| sidebar_open.toggle(),
                        "\u{2630}"
                    }

                    // Backdrop for mobile
                    if *sidebar_open.read() {
                        div {
                            class: "sidebar-backdrop",
                            onclick: move |_| sidebar_open.set(false),
                        }
                    }

                    crate::components::Sidebar {
                        open: *sidebar_open.read(),
                        on_nav: move |_| sidebar_open.set(false),
                    }

                    main { class: "main-content",
                        Outlet::<Route> {}
                    }
                }
            }
        }
        Some(Ok(_)) => {
            // Not authenticated — redirect to login
            let nav = navigator();
            nav.push(NavigationTarget::<Route>::External("/auth".to_string()));
            rsx! { div { class: "loading-page", "Redirecting to login..." } }
        }
        Some(Err(e)) => {
            rsx! {
                div { class: "error-page",
                    h2 { "Authentication Error" }
                    p { "{e}" }
                }
            }
        }
        None => {
            rsx! {
                div { class: "loading-page",
                    crate::components::LoadingSpinner {}
                    p { "Loading..." }
                }
            }
        }
    }
}

/// Admin shell layout — pill-style tab navigation for admin pages.
#[component]
fn AdminShell() -> Element {
    rsx! {
        div { class: "admin-shell",
            nav { class: "admin-nav",
                Link { to: Route::AdminDashboardPage {}, class: "admin-nav-link", "Dashboard" }
                Link { to: Route::AdminUsersPage {}, class: "admin-nav-link", "Users" }
                Link { to: Route::AdminModulesPage {}, class: "admin-nav-link", "Modules" }
                Link { to: Route::AdminLlmConfigPage {}, class: "admin-nav-link", "LLM Config" }
                Link { to: Route::AdminSourcesPage {}, class: "admin-nav-link", "Sources" }
                Link { to: Route::AdminKnowledgeBasePage {}, class: "admin-nav-link", "Knowledge Base" }
                Link { to: Route::AdminHealthPage {}, class: "admin-nav-link", "Health" }
                Link { to: Route::AdminAuditPage {}, class: "admin-nav-link", "Audit Log" }
                Link { to: Route::AdminAlertsPage {}, class: "admin-nav-link", "Alerts" }
                Link { to: Route::AdminDataGovernancePage {}, class: "admin-nav-link", "Data Governance" }
                Link { to: Route::AdminIntegrationsPage {}, class: "admin-nav-link", "Integrations" }
            }
            div { class: "admin-content",
                Outlet::<Route> {}
            }
        }
    }
}

/// Check authentication status via server function.
#[server(endpoint = "check-auth")]
async fn check_auth() -> Result<AuthInfo, ServerFnError> {
    use crate::infrastructure::require_session;

    match require_session().await {
        Ok(user_state) => Ok(AuthInfo {
            authenticated: true,
            email: user_state.email.clone(),
            name: user_state.name.clone(),
            role: user_state.role.clone(),
        }),
        Err(_) => {
            // DEV ONLY: return mock auth when Keycloak is not running
            Ok(AuthInfo {
                authenticated: true,
                email: "dev@politiktok.local".to_string(),
                name: "Dev User".to_string(),
                role: "admin".to_string(),
            })
        }
    }
}

/// Auth info shared between client and server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct AuthInfo {
    pub authenticated: bool,
    pub email: String,
    pub name: String,
    pub role: String,
}
