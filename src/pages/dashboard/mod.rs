use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Dashboard" }
            p { class: "text-base-content/70 mb-8",
                "Welcome to PolitikTok. Select a module to get started."
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                ModuleCard { title: "Volunteers", description: "Manage and match campaign volunteers", to: Route::VolunteersPage {} }
                ModuleCard { title: "Policy Chat", description: "AI-powered policy research assistant", to: Route::PolicyChatPage {} }
                ModuleCard { title: "Sentiment", description: "Real-time public sentiment monitoring", to: Route::SentimentDashboardPage {} }
                ModuleCard { title: "Campaign Copy", description: "Generate targeted campaign messaging", to: Route::CampaignCopyPage {} }
                ModuleCard { title: "Opposition", description: "Track and research opponents", to: Route::OppositionPage {} }
                ModuleCard { title: "Canvassing", description: "Plan and optimize door-to-door outreach", to: Route::CanvassingPage {} }
                ModuleCard { title: "Fundraising", description: "Donor analytics and strategy optimization", to: Route::FundraisingPage {} }
                ModuleCard { title: "Accountability", description: "Track promises and legislative actions", to: Route::AccountabilityPage {} }
                ModuleCard { title: "Empathy Simulator", description: "Understand constituent perspectives", to: Route::EmpathyPage {} }
                ModuleCard { title: "Narrative", description: "Track narrative spread and contagion", to: Route::NarrativePage {} }
                ModuleCard { title: "Coalition", description: "Detect and build political coalitions", to: Route::CoalitionPage {} }
                ModuleCard { title: "Briefings", description: "Auto-generated candidate briefing docs", to: Route::BriefingsPage {} }
                ModuleCard { title: "Call Intel", description: "Phone banking intelligence and scripts", to: Route::CallIntelPage {} }
                ModuleCard { title: "Coaching", description: "Debate prep and candidate coaching", to: Route::CoachingPage {} }
                ModuleCard { title: "Multilingual", description: "Multi-language campaign content", to: Route::MultilingualPage {} }
                ModuleCard { title: "Q&A Prep", description: "Anticipate and prepare for questions", to: Route::QuestionAnticipationPage {} }
                ModuleCard { title: "Local Issues", description: "Track and respond to local concerns", to: Route::LocalIssuesPage {} }
                ModuleCard { title: "Policy Diff", description: "Compare policy positions side by side", to: Route::PolicyDiffPage {} }
                ModuleCard { title: "Faction Mapper", description: "Map internal and external factions", to: Route::FactionMapperPage {} }
                ModuleCard { title: "Regulatory", description: "Monitor regulatory changes and impact", to: Route::RegulatoryPage {} }
                ModuleCard { title: "Media Monitor", description: "Track media coverage and mentions", to: Route::MediaMonitorPage {} }
                ModuleCard { title: "Disinfo Watch", description: "Detect and counter disinformation", to: Route::DisinfoPage {} }
                ModuleCard { title: "Compliance", description: "Campaign finance and legal compliance", to: Route::CompliancePage {} }
                ModuleCard { title: "Meetings", description: "Schedule and manage campaign meetings", to: Route::MeetingsPage {} }
                ModuleCard { title: "Knowledge Base", description: "Centralized campaign knowledge repository", to: Route::KnowledgeBasePage {} }
            }
        }
    }
}

#[component]
fn ModuleCard(title: &'static str, description: &'static str, to: Route) -> Element {
    rsx! {
        Link { to,
            div { class: "card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer",
                div { class: "card-body",
                    h2 { class: "card-title text-lg", "{title}" }
                    p { class: "text-base-content/70 text-sm", "{description}" }
                }
            }
        }
    }
}
