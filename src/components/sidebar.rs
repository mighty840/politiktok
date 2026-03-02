use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsBarChartLine, BsBriefcase, BsBullseye, BsCardChecklist, BsChatDots, BsClipboardData,
    BsCpu, BsDiagram3, BsEnvelopePaper, BsExclamationTriangle, BsFileEarmarkText, BsFlag,
    BsFunnel, BsGear, BsGlobe, BsGraphUp, BsHouseDoor, BsJournalText, BsMap, BsMegaphone,
    BsPeople, BsPersonCheck, BsQuestionCircle, BsSearch, BsShieldCheck, BsTelephone,
};
use dioxus_free_icons::Icon;

/// Navigation item descriptor.
struct NavItem {
    label: &'static str,
    route: crate::app::Route,
    icon: Element,
}

/// Navigation group descriptor.
struct NavGroup {
    label: &'static str,
    items: Vec<NavItem>,
}

/// Build the full list of navigation groups.
fn nav_groups() -> Vec<NavGroup> {
    vec![
        NavGroup {
            label: "Overview",
            items: vec![NavItem {
                label: "Dashboard",
                route: crate::app::Route::DashboardPage {},
                icon: rsx! { Icon { icon: BsHouseDoor, width: 18, height: 18 } },
            }],
        },
        NavGroup {
            label: "People & Outreach",
            items: vec![
                NavItem {
                    label: "Volunteers",
                    route: crate::app::Route::VolunteersPage {},
                    icon: rsx! { Icon { icon: BsPeople, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Canvassing",
                    route: crate::app::Route::CanvassingPage {},
                    icon: rsx! { Icon { icon: BsMap, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Call Intelligence",
                    route: crate::app::Route::CallIntelPage {},
                    icon: rsx! { Icon { icon: BsTelephone, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Fundraising",
                    route: crate::app::Route::FundraisingPage {},
                    icon: rsx! { Icon { icon: BsFunnel, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Coalition Detector",
                    route: crate::app::Route::CoalitionPage {},
                    icon: rsx! { Icon { icon: BsDiagram3, width: 18, height: 18 } },
                },
            ],
        },
        NavGroup {
            label: "AI & Content",
            items: vec![
                NavItem {
                    label: "Policy Chat",
                    route: crate::app::Route::PolicyChatPage {},
                    icon: rsx! { Icon { icon: BsChatDots, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Campaign Copy",
                    route: crate::app::Route::CampaignCopyPage {},
                    icon: rsx! { Icon { icon: BsMegaphone, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Multilingual",
                    route: crate::app::Route::MultilingualPage {},
                    icon: rsx! { Icon { icon: BsGlobe, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Empathy Simulator",
                    route: crate::app::Route::EmpathyPage {},
                    icon: rsx! { Icon { icon: BsPersonCheck, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Coaching & Debate",
                    route: crate::app::Route::CoachingPage {},
                    icon: rsx! { Icon { icon: BsBriefcase, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Question Anticipation",
                    route: crate::app::Route::QuestionAnticipationPage {},
                    icon: rsx! { Icon { icon: BsQuestionCircle, width: 18, height: 18 } },
                },
            ],
        },
        NavGroup {
            label: "Research & Analysis",
            items: vec![
                NavItem {
                    label: "Opposition Research",
                    route: crate::app::Route::OppositionPage {},
                    icon: rsx! { Icon { icon: BsSearch, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Sentiment Monitor",
                    route: crate::app::Route::SentimentDashboardPage {},
                    icon: rsx! { Icon { icon: BsGraphUp, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Narrative Contagion",
                    route: crate::app::Route::NarrativePage {},
                    icon: rsx! { Icon { icon: BsBarChartLine, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Candidate Briefings",
                    route: crate::app::Route::BriefingsPage {},
                    icon: rsx! { Icon { icon: BsFileEarmarkText, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Local Issues",
                    route: crate::app::Route::LocalIssuesPage {},
                    icon: rsx! { Icon { icon: BsFlag, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Policy Diff",
                    route: crate::app::Route::PolicyDiffPage {},
                    icon: rsx! { Icon { icon: BsClipboardData, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Faction Mapper",
                    route: crate::app::Route::FactionMapperPage {},
                    icon: rsx! { Icon { icon: BsBullseye, width: 18, height: 18 } },
                },
            ],
        },
        NavGroup {
            label: "Compliance & Monitoring",
            items: vec![
                NavItem {
                    label: "Accountability",
                    route: crate::app::Route::AccountabilityPage {},
                    icon: rsx! { Icon { icon: BsCardChecklist, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Regulatory Monitor",
                    route: crate::app::Route::RegulatoryPage {},
                    icon: rsx! { Icon { icon: BsJournalText, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Media Monitor",
                    route: crate::app::Route::MediaMonitorPage {},
                    icon: rsx! { Icon { icon: BsEnvelopePaper, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Disinfo Warning",
                    route: crate::app::Route::DisinfoPage {},
                    icon: rsx! { Icon { icon: BsExclamationTriangle, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Compliance",
                    route: crate::app::Route::CompliancePage {},
                    icon: rsx! { Icon { icon: BsShieldCheck, width: 18, height: 18 } },
                },
            ],
        },
        NavGroup {
            label: "Workspace",
            items: vec![
                NavItem {
                    label: "Meetings",
                    route: crate::app::Route::MeetingsPage {},
                    icon: rsx! { Icon { icon: BsCpu, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Knowledge Base",
                    route: crate::app::Route::KnowledgeBasePage {},
                    icon: rsx! { Icon { icon: BsJournalText, width: 18, height: 18 } },
                },
                NavItem {
                    label: "Admin",
                    route: crate::app::Route::AdminDashboardPage {},
                    icon: rsx! { Icon { icon: BsGear, width: 18, height: 18 } },
                },
            ],
        },
    ]
}

/// Application sidebar with grouped navigation links, theme toggle, and
/// mobile collapsibility.
#[component]
pub fn Sidebar(
    #[props(default = false)] open: bool,
    on_nav: EventHandler<()>,
) -> Element {
    let route = use_route::<crate::app::Route>();
    let mut is_dark = use_signal(|| true);

    let visibility_class = if open { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" };

    rsx! {
        aside {
            class: "sidebar fixed top-0 left-0 z-40 h-screen w-64 bg-base-200 border-r border-base-300 overflow-y-auto transition-transform duration-200 {visibility_class}",

            // Brand
            div { class: "p-4 border-b border-base-300",
                h2 { class: "text-xl font-bold text-primary", "PolitikTok" }
            }

            // Navigation groups
            nav { class: "p-2",
                for group in nav_groups() {
                    div { class: "mb-3",
                        p { class: "px-3 py-1 text-xs font-semibold uppercase text-base-content/50 tracking-wider",
                            "{group.label}"
                        }
                        ul { class: "menu menu-sm w-full",
                            for item in group.items {
                                {
                                    let is_active = route == item.route;
                                    let active_class = if is_active { "active" } else { "" };
                                    rsx! {
                                        li {
                                            Link {
                                                to: item.route,
                                                class: "{active_class}",
                                                onclick: move |_| on_nav.call(()),
                                                {item.icon}
                                                span { "{item.label}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Theme toggle
            div { class: "p-4 border-t border-base-300 mt-auto",
                label { class: "flex items-center gap-2 cursor-pointer",
                    span { class: "text-sm", "Theme" }
                    input {
                        r#type: "checkbox",
                        class: "toggle toggle-sm",
                        checked: *is_dark.read(),
                        onchange: move |_| {
                            let new_dark = !*is_dark.read();
                            is_dark.set(new_dark);
                            let theme = if new_dark { "politiktok-dark" } else { "politiktok-light" };
                            // Apply theme to DOM and persist
                            dioxus::prelude::document::eval(
                                &format!(
                                    "document.documentElement.setAttribute('data-theme','{theme}'); localStorage.setItem('politiktok-theme','{theme}');"
                                ),
                            );
                        },
                    }
                    span { class: "text-xs text-base-content/60",
                        if *is_dark.read() { "Dark" } else { "Light" }
                    }
                }
            }
        }
    }
}
