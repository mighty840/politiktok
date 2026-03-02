use dioxus::prelude::*;

#[component]
pub fn AdminDashboardPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Admin Dashboard" }
            p { class: "text-slate-400 mb-6",
                "System overview with health metrics, active users, and module status at a glance."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "System stats, activity charts, and quick actions will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminUsersPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "User Management" }
            p { class: "text-slate-400 mb-6",
                "Manage user accounts, roles, permissions, and team assignments."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "User table, role editor, and invitation tools will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminModulesPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Module Configuration" }
            p { class: "text-slate-400 mb-6",
                "Enable, disable, and configure platform modules for your campaign."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Module toggles, feature flags, and configuration forms will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminLlmConfigPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "LLM Configuration" }
            p { class: "text-slate-400 mb-6",
                "Configure AI model providers, API keys, rate limits, and per-module model assignments."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Provider settings, model selection, and usage monitoring will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminSourcesPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Data Sources" }
            p { class: "text-slate-400 mb-6",
                "Manage external data source connections, API integrations, and data ingestion pipelines."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Source list, connection status, and sync configuration will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminKnowledgeBasePage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Knowledge Base Admin" }
            p { class: "text-slate-400 mb-6",
                "Manage the knowledge base: upload documents, configure indexing, and monitor embedding pipelines."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Document management, indexing status, and pipeline configuration will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminHealthPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "System Health" }
            p { class: "text-slate-400 mb-6",
                "Monitor system health, service uptime, database performance, and infrastructure metrics."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Health checks, uptime charts, and alerting configuration will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminAuditPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Audit Log" }
            p { class: "text-slate-400 mb-6",
                "Review all system activity including user actions, configuration changes, and security events."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Audit log table, filters, and export tools will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminAlertsPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Alerts Configuration" }
            p { class: "text-slate-400 mb-6",
                "Configure system alerts, notification channels, escalation policies, and alert thresholds."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Alert rules, notification settings, and escalation policies will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminDataGovernancePage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Data Governance" }
            p { class: "text-slate-400 mb-6",
                "Manage data retention policies, privacy controls, GDPR compliance, and data classification."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Retention policies, privacy settings, and data classification tools will appear here." }
                }
            }
        }
    }
}

#[component]
pub fn AdminIntegrationsPage() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-3xl font-bold mb-2", "Integrations" }
            p { class: "text-slate-400 mb-6",
                "Connect third-party services like CRMs, email platforms, social media APIs, and analytics tools."
            }
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    p { class: "text-slate-400", "Integration catalog, connection status, and OAuth management will appear here." }
                }
            }
        }
    }
}
