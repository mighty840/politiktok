use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            h1 { class: "text-3xl font-bold text-slate-100", "Dashboard" }
            p { class: "text-slate-400 mb-2",
                "Welcome to PolitikTok. Select a module to get started."
            }

            // Stats row
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 stagger-enter",
                DashboardStat {
                    label: "AI Modules",
                    value: "26",
                    color: "indigo",
                }
                DashboardStat {
                    label: "Active Alerts",
                    value: "0",
                    color: "rose",
                }
                DashboardStat {
                    label: "AI Queries",
                    value: "—",
                    color: "purple",
                }
                DashboardStat {
                    label: "Team Members",
                    value: "—",
                    color: "emerald",
                }
            }

            // Categorized module grid
            ModuleCategory {
                title: "People & Outreach",
                modules: vec![
                    ("Volunteers", "Manage and match campaign volunteers", Route::VolunteersPage {}),
                    ("Canvassing", "Plan and optimize door-to-door outreach", Route::CanvassingPage {}),
                    ("Call Intel", "Phone banking intelligence and scripts", Route::CallIntelPage {}),
                    ("Fundraising", "Donor analytics and strategy optimization", Route::FundraisingPage {}),
                    ("Coalition", "Detect and build political coalitions", Route::CoalitionPage {}),
                ],
            }

            ModuleCategory {
                title: "AI & Content",
                modules: vec![
                    ("Policy Chat", "AI-powered policy research assistant", Route::PolicyChatPage {}),
                    ("Campaign Copy", "Generate targeted campaign messaging", Route::CampaignCopyPage {}),
                    ("Multilingual", "Multi-language campaign content", Route::MultilingualPage {}),
                    ("Empathy Simulator", "Understand constituent perspectives", Route::EmpathyPage {}),
                    ("Coaching", "Debate prep and candidate coaching", Route::CoachingPage {}),
                    ("Q&A Prep", "Anticipate and prepare for questions", Route::QuestionAnticipationPage {}),
                ],
            }

            ModuleCategory {
                title: "Research & Analysis",
                modules: vec![
                    ("Opposition", "Track and research opponents", Route::OppositionPage {}),
                    ("Sentiment", "Real-time public sentiment monitoring", Route::SentimentDashboardPage {}),
                    ("Narrative", "Track narrative spread and contagion", Route::NarrativePage {}),
                    ("Briefings", "Auto-generated candidate briefing docs", Route::BriefingsPage {}),
                    ("Local Issues", "Track and respond to local concerns", Route::LocalIssuesPage {}),
                    ("Policy Diff", "Compare policy positions side by side", Route::PolicyDiffPage {}),
                    ("Faction Mapper", "Map internal and external factions", Route::FactionMapperPage {}),
                ],
            }

            ModuleCategory {
                title: "Compliance & Monitoring",
                modules: vec![
                    ("Accountability", "Track promises and legislative actions", Route::AccountabilityPage {}),
                    ("Regulatory", "Monitor regulatory changes and impact", Route::RegulatoryPage {}),
                    ("Media Monitor", "Track media coverage and mentions", Route::MediaMonitorPage {}),
                    ("Disinfo Watch", "Detect and counter disinformation", Route::DisinfoPage {}),
                    ("Compliance", "Campaign finance and legal compliance", Route::CompliancePage {}),
                ],
            }

            ModuleCategory {
                title: "Workspace",
                modules: vec![
                    ("Meetings", "Schedule and manage campaign meetings", Route::MeetingsPage {}),
                    ("Knowledge Base", "Centralized campaign knowledge repository", Route::KnowledgeBasePage {}),
                ],
            }
        }
    }
}

#[component]
fn DashboardStat(label: &'static str, value: &'static str, color: &'static str) -> Element {
    let gradient = match color {
        "indigo" => "linear-gradient(135deg, rgba(99,102,241,0.15), rgba(99,102,241,0.05))",
        "rose" => "linear-gradient(135deg, rgba(244,63,94,0.15), rgba(244,63,94,0.05))",
        "purple" => "linear-gradient(135deg, rgba(168,85,247,0.15), rgba(168,85,247,0.05))",
        "emerald" => "linear-gradient(135deg, rgba(16,185,129,0.15), rgba(16,185,129,0.05))",
        _ => "linear-gradient(135deg, rgba(99,102,241,0.15), rgba(99,102,241,0.05))",
    };

    let text_color = match color {
        "indigo" => "#818cf8",
        "rose" => "#fb7185",
        "purple" => "#c084fc",
        "emerald" => "#6ee7b7",
        _ => "#818cf8",
    };

    rsx! {
        div {
            class: "glass-card p-5",
            style: "background: {gradient};",
            p { class: "text-sm text-slate-400 mb-1", "{label}" }
            p {
                class: "text-3xl font-bold",
                style: "color: {text_color};",
                "{value}"
            }
        }
    }
}

#[component]
fn ModuleCategory(title: &'static str, modules: Vec<(&'static str, &'static str, Route)>) -> Element {
    rsx! {
        div { class: "space-y-3",
            h2 { class: "text-lg font-semibold text-slate-300 flex items-center gap-2",
                "{title}"
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 stagger-enter",
                for (name, desc, route) in modules {
                    Link { to: route,
                        div { class: "glass-card gradient-border p-5 cursor-pointer group",
                            h3 { class: "text-base font-semibold text-slate-200 mb-1 group-hover:text-indigo-400 transition-colors", "{name}" }
                            p { class: "text-sm text-slate-500", "{desc}" }
                        }
                    }
                }
            }
        }
    }
}
