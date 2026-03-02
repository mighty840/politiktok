use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsBarChartLine, BsBriefcase, BsBullseye, BsCardChecklist, BsChatDots, BsChevronDown,
    BsClipboardData, BsCpu, BsDiagram3, BsEnvelopePaper, BsExclamationTriangle, BsFileEarmarkText,
    BsFlag, BsFunnel, BsGear, BsGlobe, BsGraphUp, BsHouseDoor, BsJournalText, BsMap, BsMegaphone,
    BsPeople, BsPersonCheck, BsQuestionCircle, BsSearch, BsShieldCheck, BsTelephone,
};
use dioxus_free_icons::Icon;

use crate::app::AuthInfo;

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

/// Collapsible nav group with chevron toggle.
#[component]
fn CollapsibleNavGroup(
    label: &'static str,
    children: Element,
    #[props(default = true)] default_open: bool,
) -> Element {
    let mut is_open = use_signal(move || default_open);

    rsx! {
        div { class: "mb-1",
            div {
                class: "nav-group-header",
                onclick: move |_| is_open.toggle(),
                span { class: "sidebar-section-label", style: "padding: 0;", "{label}" }
                span {
                    class: if *is_open.read() { "nav-group-chevron open" } else { "nav-group-chevron" },
                    Icon { icon: BsChevronDown, width: 12, height: 12 }
                }
            }
            div {
                class: if *is_open.read() { "nav-group-items expanded" } else { "nav-group-items collapsed" },
                {children}
            }
        }
    }
}

/// Application sidebar with grouped navigation links, theme toggle, and
/// mobile collapsibility.
#[component]
pub fn Sidebar(#[props(default = false)] open: bool, on_nav: EventHandler<()>) -> Element {
    let route = use_route::<crate::app::Route>();
    let mut is_dark = use_signal(|| true);

    // Try to read auth info from context (provided by AppShell)
    let auth_info = try_use_context::<Signal<AuthInfo>>();

    let visibility_class = if open {
        "translate-x-0"
    } else {
        "-translate-x-full lg:translate-x-0"
    };

    // Derive user initials for avatar
    let (user_name, user_email, user_initials) = if let Some(info) = auth_info {
        let info = info.read();
        let initials: String = info
            .name
            .split_whitespace()
            .take(2)
            .filter_map(|w| w.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .collect();
        let initials = if initials.is_empty() {
            "U".to_string()
        } else {
            initials
        };
        (info.name.clone(), info.email.clone(), initials)
    } else {
        ("User".to_string(), String::new(), "U".to_string())
    };

    rsx! {
        aside {
            class: "sidebar sidebar-glass fixed top-0 left-0 z-40 h-screen w-64 overflow-y-auto transition-transform duration-200 {visibility_class}",

            // Brand header with gradient logo
            div { class: "sidebar-header",
                // Logo icon — gradient rounded square with "PT"
                div {
                    class: "flex items-center justify-center w-9 h-9 rounded-lg text-white text-sm font-bold flex-shrink-0",
                    style: "background: linear-gradient(135deg, #6366f1, #8b5cf6);",
                    "PT"
                }
                h1 { class: "text-lg font-bold",
                    style: "background: linear-gradient(135deg, #818cf8, #c084fc); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;",
                    "PolitikTok"
                }
            }

            // Navigation groups
            nav { class: "sidebar-nav",
                for group in nav_groups() {
                    CollapsibleNavGroup { label: group.label,
                        for item in group.items {
                            {
                                let is_active = route == item.route;
                                let active_class = if is_active { "sidebar-link active" } else { "sidebar-link" };
                                rsx! {
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

            // User section at bottom
            div { class: "mt-auto border-t border-white/5 p-4",
                div { class: "flex items-center gap-3 mb-3",
                    // Avatar circle with gradient
                    div {
                        class: "flex items-center justify-center w-8 h-8 rounded-full text-white text-xs font-semibold flex-shrink-0",
                        style: "background: linear-gradient(135deg, #6366f1, #ec4899);",
                        "{user_initials}"
                    }
                    div { class: "min-w-0 flex-1",
                        p { class: "text-sm font-medium text-slate-200 truncate", "{user_name}" }
                        if !user_email.is_empty() {
                            p { class: "text-xs text-slate-500 truncate", "{user_email}" }
                        }
                    }
                }

                // Theme toggle
                label { class: "flex items-center gap-2 cursor-pointer",
                    span { class: "text-slate-500 text-xs",
                        if *is_dark.read() { "\u{263E}" } else { "\u{2600}" }
                    }
                    input {
                        r#type: "checkbox",
                        class: "toggle toggle-sm",
                        checked: *is_dark.read(),
                        onchange: move |_| {
                            let new_dark = !*is_dark.read();
                            is_dark.set(new_dark);
                            let theme = if new_dark {
                                "politiktok-dark"
                            } else {
                                "politiktok-light"
                            };
                            dioxus::prelude::document::eval(&format!(
                                "document.documentElement.setAttribute('data-theme','{theme}'); localStorage.setItem('politiktok-theme','{theme}');"
                            ));
                        },
                    }
                    span { class: "text-xs text-slate-500",
                        if *is_dark.read() { "Dark" } else { "Light" }
                    }
                }
            }
        }
    }
}
