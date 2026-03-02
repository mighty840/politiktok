use dioxus::prelude::*;

use crate::modules::accountability::{
    add_evidence, extract_commitments, get_accountability_summary, get_commitment,
    list_commitments, AccountabilitySummary, Commitment, Evidence,
};

#[component]
pub fn AccountabilityPage() -> Element {
    let mut commitments_refresh = use_signal(|| 0u32);
    let mut summary_refresh = use_signal(|| 0u32);
    let mut topic_filter = use_signal(String::new);
    let mut status_filter = use_signal(String::new);
    let mut selected_commitment_id = use_signal(|| Option::<String>::None);

    // Fetch summary
    let summary = use_resource(move || {
        let _trigger = summary_refresh();
        async move { get_accountability_summary().await.ok() }
    });

    // Fetch commitments with filters
    let commitments = use_resource(move || {
        let _trigger = commitments_refresh();
        let topic = topic_filter();
        let status = status_filter();
        async move {
            let t = if topic.is_empty() { None } else { Some(topic) };
            let s = if status.is_empty() { None } else { Some(status) };
            list_commitments(t, s).await.unwrap_or_default()
        }
    });

    let mut on_refresh = move || {
        commitments_refresh += 1;
        summary_refresh += 1;
    };

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold mb-2", "Manifesto Accountability Engine" }
            p { class: "text-base-content/70 mb-6",
                "Track campaign promises, extract commitments from manifestos, and evaluate evidence of fulfillment."
            }

            // Summary cards
            SummaryCards { summary: summary().flatten() }

            // Extract commitments section
            ExtractCommitmentsSection {
                on_extracted: move || {
                    on_refresh();
                },
            }

            // Filters
            div { class: "flex gap-4 mb-4 mt-6",
                div { class: "form-control",
                    label { class: "label label-text text-sm", "Filter by Topic" }
                    select {
                        class: "select select-bordered select-sm",
                        value: "{topic_filter}",
                        onchange: move |evt| {
                            topic_filter.set(evt.value());
                            commitments_refresh += 1;
                        },
                        option { value: "", "All Topics" }
                        option { value: "healthcare", "Healthcare" }
                        option { value: "economy", "Economy" }
                        option { value: "education", "Education" }
                        option { value: "environment", "Environment" }
                        option { value: "security", "Security" }
                        option { value: "infrastructure", "Infrastructure" }
                        option { value: "housing", "Housing" }
                        option { value: "social", "Social" }
                    }
                }
                div { class: "form-control",
                    label { class: "label label-text text-sm", "Filter by Status" }
                    select {
                        class: "select select-bordered select-sm",
                        value: "{status_filter}",
                        onchange: move |evt| {
                            status_filter.set(evt.value());
                            commitments_refresh += 1;
                        },
                        option { value: "", "All Statuses" }
                        option { value: "active", "Active" }
                        option { value: "fulfilled", "Fulfilled" }
                        option { value: "broken", "Broken" }
                        option { value: "partial", "Partial" }
                    }
                }
            }

            // Commitments table
            div { class: "card bg-base-100 shadow-sm mb-6",
                div { class: "card-body p-0",
                    h2 { class: "text-lg font-semibold px-6 pt-4 pb-2", "Commitments" }
                    div { class: "overflow-x-auto",
                        table { class: "table table-zebra",
                            thead {
                                tr {
                                    th { "Commitment" }
                                    th { "Topic" }
                                    th { "Strength" }
                                    th { "Status" }
                                    th { "Evidence" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                if commitments().unwrap_or_default().is_empty() {
                                    tr {
                                        td { colspan: "6", class: "text-center text-base-content/50 py-8",
                                            "No commitments found. Extract commitments from a manifesto to get started."
                                        }
                                    }
                                }
                                for commitment in commitments().unwrap_or_default().iter() {
                                    {
                                        let cid = commitment.id.clone();
                                        let truncated = if commitment.text.len() > 80 {
                                            format!("{}...", &commitment.text[..80])
                                        } else {
                                            commitment.text.clone()
                                        };
                                        rsx! {
                                            tr {
                                                class: "hover cursor-pointer",
                                                onclick: move |_| {
                                                    selected_commitment_id.set(Some(cid.clone()));
                                                },
                                                td { class: "max-w-md",
                                                    span { class: "text-sm", "{truncated}" }
                                                }
                                                td {
                                                    if let Some(ref topic) = commitment.topic {
                                                        span { class: "badge badge-outline badge-sm", "{topic}" }
                                                    }
                                                }
                                                td {
                                                    StrengthBadge { strength: commitment.strength.clone() }
                                                }
                                                td {
                                                    StatusBadge { status: commitment.status.clone() }
                                                }
                                                td {
                                                    span { class: "badge badge-ghost badge-sm",
                                                        "{commitment.evidence_count}"
                                                    }
                                                }
                                                td {
                                                    button {
                                                        class: "btn btn-ghost btn-xs",
                                                        onclick: move |evt| {
                                                            evt.stop_propagation();
                                                        },
                                                        "View"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Detail panel for selected commitment
            if let Some(cid) = selected_commitment_id() {
                CommitmentDetail {
                    commitment_id: cid,
                    on_close: move || {
                        selected_commitment_id.set(None);
                    },
                    on_evidence_added: move || {
                        on_refresh();
                    },
                }
            }
        }
    }
}

/// Summary cards at the top of the dashboard.
#[component]
fn SummaryCards(summary: Option<AccountabilitySummary>) -> Element {
    let s = summary.unwrap_or(AccountabilitySummary {
        total_commitments: 0,
        fulfilled_pct: 0.0,
        broken_pct: 0.0,
        pending_pct: 0.0,
        partial_pct: 0.0,
    });

    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6",
            div { class: "stat bg-base-100 rounded-box shadow-sm",
                div { class: "stat-title", "Total Commitments" }
                div { class: "stat-value text-primary", "{s.total_commitments}" }
            }
            div { class: "stat bg-base-100 rounded-box shadow-sm",
                div { class: "stat-title", "Fulfilled" }
                div { class: "stat-value text-success", "{s.fulfilled_pct:.1}%" }
            }
            div { class: "stat bg-base-100 rounded-box shadow-sm",
                div { class: "stat-title", "Broken" }
                div { class: "stat-value text-error", "{s.broken_pct:.1}%" }
            }
            div { class: "stat bg-base-100 rounded-box shadow-sm",
                div { class: "stat-title", "Pending" }
                div { class: "stat-value text-warning", "{s.pending_pct:.1}%" }
            }
        }
    }
}

/// Section to paste manifesto text and extract commitments.
#[component]
fn ExtractCommitmentsSection(on_extracted: EventHandler) -> Element {
    let mut doc_title = use_signal(String::new);
    let mut doc_text = use_signal(String::new);
    let mut is_extracting = use_signal(|| false);
    let mut extract_error = use_signal(|| Option::<String>::None);
    let mut extract_result = use_signal(|| Option::<Vec<Commitment>>::None);
    let mut show_section = use_signal(|| false);

    let on_extract = move |_| {
        let title = doc_title().trim().to_string();
        let text = doc_text().trim().to_string();

        if title.is_empty() || text.is_empty() {
            extract_error.set(Some("Both title and text are required.".to_string()));
            return;
        }

        is_extracting.set(true);
        extract_error.set(None);
        extract_result.set(None);

        spawn(async move {
            match extract_commitments(text, title).await {
                Ok(commitments) => {
                    let count = commitments.len();
                    extract_result.set(Some(commitments));
                    doc_title.set(String::new());
                    doc_text.set(String::new());
                    on_extracted.call(());
                    tracing::info!("Extracted {count} commitments");
                }
                Err(e) => {
                    extract_error.set(Some(format!("Extraction failed: {e}")));
                }
            }
            is_extracting.set(false);
        });
    };

    rsx! {
        div { class: "card bg-base-100 shadow-sm mb-6",
            div { class: "card-body",
                div { class: "flex items-center justify-between",
                    h2 { class: "card-title text-lg", "Extract Commitments" }
                    button {
                        class: "btn btn-ghost btn-sm",
                        onclick: move |_| show_section.toggle(),
                        if *show_section.read() { "Hide" } else { "Show" }
                    }
                }

                if *show_section.read() {
                    div { class: "space-y-3 mt-3",
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Document Title" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "text",
                                placeholder: "e.g. Party Manifesto 2025",
                                value: "{doc_title}",
                                oninput: move |evt| doc_title.set(evt.value()),
                            }
                        }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Manifesto / Policy Text" }
                            }
                            textarea {
                                class: "textarea textarea-bordered w-full h-40",
                                placeholder: "Paste the full manifesto or policy document text here...",
                                value: "{doc_text}",
                                oninput: move |evt| doc_text.set(evt.value()),
                            }
                        }

                        if let Some(err) = extract_error() {
                            div { class: "alert alert-error",
                                span { "{err}" }
                            }
                        }

                        if let Some(ref results) = extract_result() {
                            div { class: "alert alert-success",
                                span { "Successfully extracted {results.len()} commitments." }
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: *is_extracting.read(),
                            onclick: on_extract,
                            if *is_extracting.read() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Extracting..."
                            } else {
                                "Extract Commitments"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Commitment detail view with evidence list and add-evidence form.
#[component]
fn CommitmentDetail(
    commitment_id: String,
    on_close: EventHandler,
    on_evidence_added: EventHandler,
) -> Element {
    let mut evidence_refresh = use_signal(|| 0u32);
    let cid = commitment_id.clone();

    let detail = use_resource(move || {
        let _trigger = evidence_refresh();
        let id = cid.clone();
        async move { get_commitment(id).await.ok() }
    });

    rsx! {
        div { class: "card bg-base-100 shadow-lg border border-base-300 mb-6",
            div { class: "card-body",
                div { class: "flex items-start justify-between mb-4",
                    h2 { class: "card-title text-lg", "Commitment Detail" }
                    button {
                        class: "btn btn-ghost btn-sm",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }

                match detail() {
                    Some(Some((commitment, evidence))) => rsx! {
                        // Commitment info
                        div { class: "mb-4",
                            p { class: "text-base mb-3", "{commitment.text}" }
                            div { class: "flex flex-wrap gap-2",
                                if let Some(ref topic) = commitment.topic {
                                    span { class: "badge badge-outline", "Topic: {topic}" }
                                }
                                StrengthBadge { strength: commitment.strength.clone() }
                                StatusBadge { status: commitment.status.clone() }
                                if let Some(ref date) = commitment.date {
                                    span { class: "badge badge-ghost", "Date: {date}" }
                                }
                            }
                        }

                        div { class: "divider" }

                        // Evidence list
                        h3 { class: "font-semibold mb-3", "Evidence ({evidence.len()})" }

                        if evidence.is_empty() {
                            p { class: "text-base-content/50 text-sm mb-4",
                                "No evidence has been submitted for this commitment yet."
                            }
                        }

                        div { class: "space-y-3 mb-4",
                            for ev in evidence.iter() {
                                EvidenceCard { evidence: ev.clone() }
                            }
                        }

                        div { class: "divider" }

                        // Add evidence form
                        AddEvidenceForm {
                            commitment_id: commitment.id.clone(),
                            on_added: move || {
                                evidence_refresh += 1;
                                on_evidence_added.call(());
                            },
                        }
                    },
                    Some(None) => rsx! {
                        div { class: "alert alert-error",
                            span { "Failed to load commitment details." }
                        }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-8",
                            span { class: "loading loading-spinner loading-lg" }
                        }
                    },
                }
            }
        }
    }
}

/// Display a single evidence item.
#[component]
fn EvidenceCard(evidence: Evidence) -> Element {
    let (badge_class, label) = match evidence.classification.as_deref() {
        Some("fulfilled") => ("badge badge-success", "Fulfilled"),
        Some("broken") => ("badge badge-error", "Broken"),
        Some("partial") => ("badge badge-warning", "Partial"),
        Some("unrelated") => ("badge badge-ghost", "Unrelated"),
        _ => ("badge badge-ghost", "Unknown"),
    };

    let confidence_pct = (evidence.confidence * 100.0) as i32;

    rsx! {
        div { class: "card card-compact bg-base-200",
            div { class: "card-body",
                div { class: "flex items-center gap-2 mb-1",
                    span { class: "{badge_class} badge-sm", "{label}" }
                    span { class: "text-xs text-base-content/60",
                        "Confidence: {confidence_pct}%"
                    }
                    if let Some(ref created) = evidence.created_at {
                        span { class: "text-xs text-base-content/40 ml-auto", "{created}" }
                    }
                }
                if let Some(ref explanation) = evidence.explanation {
                    p { class: "text-sm", "{explanation}" }
                }
            }
        }
    }
}

/// Form to add new evidence for a commitment.
#[component]
fn AddEvidenceForm(commitment_id: String, on_added: EventHandler) -> Element {
    let mut evidence_text = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut result_msg = use_signal(|| Option::<String>::None);

    let cid = commitment_id.clone();

    let on_submit = move |_| {
        let text = evidence_text().trim().to_string();
        let commitment_id = cid.clone();

        if text.is_empty() {
            error_msg.set(Some("Evidence text is required.".to_string()));
            return;
        }

        is_submitting.set(true);
        error_msg.set(None);
        result_msg.set(None);

        spawn(async move {
            match add_evidence(commitment_id, text).await {
                Ok(ev) => {
                    let classification = ev.classification.unwrap_or_else(|| "unknown".to_string());
                    let conf = (ev.confidence * 100.0) as i32;
                    result_msg.set(Some(format!(
                        "Classified as \"{classification}\" with {conf}% confidence."
                    )));
                    evidence_text.set(String::new());
                    on_added.call(());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to add evidence: {e}")));
                }
            }
            is_submitting.set(false);
        });
    };

    rsx! {
        div { class: "space-y-3",
            h3 { class: "font-semibold", "Add Evidence" }
            div { class: "form-control",
                textarea {
                    class: "textarea textarea-bordered w-full",
                    placeholder: "Paste evidence text (news article, legislative action, voting record, etc.)...",
                    rows: "3",
                    value: "{evidence_text}",
                    oninput: move |evt| evidence_text.set(evt.value()),
                }
            }

            if let Some(err) = error_msg() {
                div { class: "alert alert-error alert-sm",
                    span { "{err}" }
                }
            }

            if let Some(msg) = result_msg() {
                div { class: "alert alert-info alert-sm",
                    span { "{msg}" }
                }
            }

            button {
                class: "btn btn-primary btn-sm",
                disabled: *is_submitting.read(),
                onclick: on_submit,
                if *is_submitting.read() {
                    span { class: "loading loading-spinner loading-sm" }
                    "Classifying..."
                } else {
                    "Submit Evidence"
                }
            }
        }
    }
}

/// Badge showing commitment strength.
#[component]
fn StrengthBadge(strength: Option<String>) -> Element {
    let (class, label) = match strength.as_deref() {
        Some("strong") => ("badge badge-primary badge-sm", "Strong"),
        Some("moderate") => ("badge badge-info badge-sm", "Moderate"),
        Some("weak") => ("badge badge-ghost badge-sm", "Weak"),
        _ => ("badge badge-ghost badge-sm", "N/A"),
    };

    rsx! {
        span { class: "{class}", "{label}" }
    }
}

/// Badge showing commitment status with color coding.
#[component]
fn StatusBadge(status: String) -> Element {
    let (class, label) = match status.as_str() {
        "fulfilled" => ("badge badge-success badge-sm", "Fulfilled"),
        "broken" => ("badge badge-error badge-sm", "Broken"),
        "partial" => ("badge badge-warning badge-sm", "Partial"),
        "active" => ("badge badge-info badge-sm", "Active"),
        _ => ("badge badge-ghost badge-sm", &*status),
    };

    rsx! {
        span { class: "{class}", "{label}" }
    }
}
