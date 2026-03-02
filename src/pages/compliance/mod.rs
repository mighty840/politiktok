use dioxus::prelude::*;

use crate::modules::compliance_reporting::{
    check_compliance, generate_compliance_report, ComplianceCheckResult, ComplianceReport,
};

/// Report type options for the dropdown.
const REPORT_TYPES: &[&str] = &[
    "Financial Disclosure",
    "Advertising Compliance",
    "Donation Compliance",
    "Campaign Activity",
];

#[component]
pub fn CompliancePage() -> Element {
    // Report generation state
    let mut report_type = use_signal(|| "Financial Disclosure".to_string());
    let mut period = use_signal(String::new);
    let mut campaign_data = use_signal(String::new);
    let mut is_generating = use_signal(|| false);
    let mut report_result = use_signal(|| Option::<ComplianceReport>::None);
    let mut report_error = use_signal(|| Option::<String>::None);

    // Compliance checker state
    let mut check_action = use_signal(String::new);
    let mut is_checking = use_signal(|| false);
    let mut check_result = use_signal(|| Option::<ComplianceCheckResult>::None);
    let mut check_error = use_signal(|| Option::<String>::None);

    // Handle report generation
    let on_generate = move |_| {
        let rt = report_type().clone();
        let p = period().trim().to_string();
        let data = campaign_data().trim().to_string();

        if p.is_empty() {
            report_error.set(Some("Reporting period is required.".to_string()));
            return;
        }
        if data.is_empty() {
            report_error.set(Some("Campaign data is required.".to_string()));
            return;
        }

        is_generating.set(true);
        report_error.set(None);

        spawn(async move {
            match generate_compliance_report(rt, p, data).await {
                Ok(report) => report_result.set(Some(report)),
                Err(e) => report_error.set(Some(format!("Generation failed: {e}"))),
            }
            is_generating.set(false);
        });
    };

    // Handle compliance check
    let on_check = move |_| {
        let action = check_action().trim().to_string();
        if action.is_empty() {
            check_error.set(Some("Action description is required.".to_string()));
            return;
        }

        is_checking.set(true);
        check_error.set(None);

        spawn(async move {
            match check_compliance(action).await {
                Ok(result) => check_result.set(Some(result)),
                Err(e) => check_error.set(Some(format!("Check failed: {e}"))),
            }
            is_checking.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Compliance Reporting" }
                p { class: "text-base-content/70",
                    "Generate electoral compliance reports and check campaign actions against regulatory requirements."
                }
            }

            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Report Generation Form
                div { class: "w-full lg:w-1/2",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Generate Compliance Report" }

                            // Report Type
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Report Type" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{report_type}",
                                    onchange: move |evt: Event<FormData>| report_type.set(evt.value()),
                                    for rt in REPORT_TYPES {
                                        option { value: "{rt}", "{rt}" }
                                    }
                                }
                            }

                            // Period
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Reporting Period" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "e.g., Q1 2026, January 2026, 2025 Annual",
                                    value: "{period}",
                                    oninput: move |evt| period.set(evt.value()),
                                }
                            }

                            // Campaign Data
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Campaign Data" }
                                    span { class: "label-text-alt text-base-content/50", "Paste financial data, activities, etc." }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Paste campaign financial data, donation records, advertising spend, or activity logs here...",
                                    rows: "8",
                                    value: "{campaign_data}",
                                    oninput: move |evt| campaign_data.set(evt.value()),
                                }
                            }

                            // Error
                            if let Some(err) = report_error() {
                                div { class: "alert alert-error text-sm",
                                    span { "{err}" }
                                }
                            }

                            // Generate button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_generating.read(),
                                onclick: on_generate,
                                if *is_generating.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Generating Report..."
                                } else {
                                    "Generate Report"
                                }
                            }
                        }
                    }
                }

                // Right: Report Results
                div { class: "w-full lg:w-1/2",
                    if let Some(report) = report_result() {
                        ReportDisplay { report: report }
                    } else {
                        div { class: "card bg-base-100 shadow-sm min-h-[300px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    p { class: "text-lg font-medium text-base-content/50 mb-2",
                                        "No report generated yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Fill in the form and click Generate to create a compliance report."
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Compliance Checker Section
            div { class: "divider", "Compliance Checker" }

            div { class: "card bg-base-100 shadow-sm",
                div { class: "card-body space-y-4",
                    h2 { class: "card-title text-lg", "Check Action Compliance" }
                    p { class: "text-base-content/70 text-sm",
                        "Describe a campaign action to check whether it complies with electoral law."
                    }

                    div { class: "flex flex-col sm:flex-row gap-3",
                        textarea {
                            class: "textarea textarea-bordered flex-1",
                            placeholder: "e.g., Accept a $5,000 donation from a foreign national for a state campaign event...",
                            rows: "3",
                            value: "{check_action}",
                            oninput: move |evt| check_action.set(evt.value()),
                        }
                        button {
                            class: "btn btn-secondary self-end",
                            disabled: *is_checking.read(),
                            onclick: on_check,
                            if *is_checking.read() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Checking..."
                            } else {
                                "Check Compliance"
                            }
                        }
                    }

                    // Check error
                    if let Some(err) = check_error() {
                        div { class: "alert alert-error text-sm",
                            span { "{err}" }
                        }
                    }

                    // Check result
                    if let Some(result) = check_result() {
                        ComplianceCheckDisplay { result: result }
                    }
                }
            }
        }
    }
}

/// Display a generated compliance report.
#[component]
fn ReportDisplay(report: ComplianceReport) -> Element {
    let status_class = match report.status.as_str() {
        "compliant" => "badge badge-success",
        "non_compliant" => "badge badge-error",
        _ => "badge badge-warning",
    };

    let status_label = match report.status.as_str() {
        "compliant" => "Compliant",
        "non_compliant" => "Non-Compliant",
        _ => "Needs Review",
    };

    rsx! {
        div { class: "card bg-base-100 shadow-sm",
            div { class: "card-body space-y-4",
                div { class: "flex items-center justify-between",
                    h2 { class: "card-title text-lg", "{report.report_type}" }
                    span { class: "{status_class} badge-lg", "{status_label}" }
                }

                div { class: "flex gap-4 text-sm text-base-content/60",
                    span { "Period: {report.period}" }
                    span { "Generated: {report.created_at}" }
                }

                div { class: "divider my-1" }

                // Sections
                div { class: "space-y-4",
                    for section in report.content.iter() {
                        div { class: "border border-base-300 rounded-lg p-4",
                            div { class: "flex items-center justify-between mb-2",
                                h3 { class: "font-semibold", "{section.title}" }
                                if section.compliant {
                                    span { class: "badge badge-success badge-sm", "Compliant" }
                                } else {
                                    span { class: "badge badge-error badge-sm", "Non-Compliant" }
                                }
                            }
                            p { class: "text-sm text-base-content/80 whitespace-pre-wrap mb-2",
                                "{section.content}"
                            }
                            if !section.notes.is_empty() {
                                div { class: "bg-base-200 rounded p-2 text-sm",
                                    span { class: "font-medium", "Notes: " }
                                    "{section.notes}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Display a compliance check result.
#[component]
fn ComplianceCheckDisplay(result: ComplianceCheckResult) -> Element {
    rsx! {
        div { class: "border border-base-300 rounded-lg p-4 space-y-3",
            div { class: "flex items-center gap-3",
                if result.compliant {
                    span { class: "badge badge-success badge-lg", "Compliant" }
                } else {
                    span { class: "badge badge-error badge-lg", "Non-Compliant" }
                }
                span { class: "text-sm text-base-content/60 italic", "\"{result.action}\"" }
            }

            p { class: "text-sm whitespace-pre-wrap", "{result.explanation}" }

            if !result.relevant_laws.is_empty() {
                div {
                    h4 { class: "text-sm font-semibold mb-1", "Relevant Laws & Regulations" }
                    ul { class: "list-disc list-inside text-sm text-base-content/80 space-y-1",
                        for law in result.relevant_laws.iter() {
                            li { "{law}" }
                        }
                    }
                }
            }

            if !result.recommendations.is_empty() {
                div {
                    h4 { class: "text-sm font-semibold mb-1", "Recommendations" }
                    ul { class: "list-disc list-inside text-sm text-base-content/80 space-y-1",
                        for rec in result.recommendations.iter() {
                            li { "{rec}" }
                        }
                    }
                }
            }
        }
    }
}
