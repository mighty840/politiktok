use crate::{components::*, pages::*};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Root application component.
#[component]
pub fn App() -> Element {
    rsx! {
        // Tailwind CSS 3 CDN + DaisyUI 4 plugin
        document::Script { src: "https://cdn.tailwindcss.com" }
        document::Script {
            "tailwind.config = {{ plugins: [], theme: {{ extend: {{}} }} }};"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.23/dist/full.min.css",
        }

        document::Stylesheet { href: MAIN_CSS }

        // Set DaisyUI dark theme
        document::Script { "document.documentElement.setAttribute('data-theme', 'dark');" }

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

/// Admin shell layout — additional nav for admin pages.
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
