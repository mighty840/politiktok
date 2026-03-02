use dioxus::prelude::*;

use crate::modules::fundraising::{
    create_donor, draft_solicitation, get_fundraising_summary, list_donors, record_donation,
    FundraisingSummary,
};
use crate::models::donor::Donor;

/// Helper: compute total donated from a donor's donation_history JSONB value.
fn total_donated(donor: &Donor) -> f64 {
    donor
        .donation_history
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.get("amount").and_then(|a| a.as_f64()))
                .sum()
        })
        .unwrap_or(0.0)
}

#[component]
pub fn FundraisingPage() -> Element {
    // Refresh triggers
    let mut donors_refresh = use_signal(|| 0u32);
    let mut summary_refresh = use_signal(|| 0u32);

    // Search / filter state
    let mut search_text = use_signal(String::new);
    let mut min_score_filter = use_signal(|| 0.0_f64);

    // Modal state
    let mut show_add_modal = use_signal(|| false);
    let mut show_donation_modal = use_signal(|| Option::<String>::None); // donor_id
    let mut selected_donor = use_signal(|| Option::<Donor>::None);

    // Add donor form state
    let mut new_name = use_signal(String::new);
    let mut new_email = use_signal(String::new);
    let mut new_tags = use_signal(String::new);

    // Record donation form state
    let mut donation_amount = use_signal(String::new);
    let mut donation_date = use_signal(String::new);
    let mut donation_note = use_signal(String::new);

    // Solicitation state
    let mut solicitation_context = use_signal(String::new);
    let mut solicitation_amount = use_signal(String::new);
    let mut solicitation_result = use_signal(|| Option::<String>::None);
    let mut solicitation_loading = use_signal(|| false);

    // Loading flags
    let mut adding_donor = use_signal(|| false);
    let mut recording_donation = use_signal(|| false);

    // Fetch summary
    let summary_resource = use_resource(move || async move {
        let _ = summary_refresh();
        get_fundraising_summary().await.ok()
    });

    // Fetch donors
    let donors_resource = use_resource(move || {
        let search = search_text();
        let min_score = min_score_filter();
        async move {
            let _ = donors_refresh();
            let s = if search.is_empty() { None } else { Some(search) };
            let ms = if min_score <= 0.0 { None } else { Some(min_score) };
            list_donors(s, ms).await.unwrap_or_default()
        }
    });

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Fundraising Intelligence" }
                p { class: "text-slate-400",
                    "Optimize donation strategies with donor management, engagement tracking, and AI-powered solicitations."
                }
            }

            // Summary stat cards
            {
                match &*summary_resource.read() {
                    Some(Some(summary)) => rsx! {
                        SummaryCards { summary: summary.clone() }
                    },
                    Some(None) => rsx! {
                        div { class: "alert alert-warning", "Failed to load fundraising summary." }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-4",
                            span { class: "loading loading-spinner loading-lg text-primary" }
                        }
                    },
                }
            }

            // Filters and actions bar
            div { class: "flex flex-col sm:flex-row sm:items-center gap-3",
                input {
                    class: "input input-bordered w-full sm:w-64",
                    r#type: "text",
                    placeholder: "Search donors by name...",
                    value: "{search_text}",
                    oninput: move |e| search_text.set(e.value()),
                }
                select {
                    class: "select select-bordered w-full sm:w-48",
                    value: "0",
                    onchange: move |e| {
                        let val: f64 = e.value().parse().unwrap_or(0.0);
                        min_score_filter.set(val);
                    },
                    option { value: "0", "All Engagement Levels" }
                    option { value: "0.25", "Score >= 25%" }
                    option { value: "0.5", "Score >= 50%" }
                    option { value: "0.75", "Score >= 75%" }
                }
                div { class: "flex-1" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_add_modal.set(true),
                    "Add Donor"
                }
            }

            // Donor table
            div { class: "glass-card gradient-border",
                div { class: "card-body p-0",
                    div { class: "overflow-x-auto",
                        {
                            match &*donors_resource.read() {
                                Some(donors) if donors.is_empty() => rsx! {
                                    div { class: "p-8 text-center text-slate-400",
                                        "No donors found. Add your first donor to get started."
                                    }
                                },
                                Some(donors) => rsx! {
                                    table { class: "table table-zebra",
                                        thead {
                                            tr {
                                                th { "Name" }
                                                th { "Email" }
                                                th { "Engagement" }
                                                th { "Total Donated" }
                                                th { "Last Contact" }
                                                th { "Tags" }
                                                th { "Actions" }
                                            }
                                        }
                                        tbody {
                                            for donor in donors.iter() {
                                                {
                                                    let donor_clone = donor.clone();
                                                    let donor_id = donor.id.clone();
                                                    let donated = total_donated(donor);
                                                    let score_pct = (donor.engagement_score * 100.0) as u32;
                                                    rsx! {
                                                        tr {
                                                            class: "hover cursor-pointer",
                                                            onclick: {
                                                                let d = donor_clone.clone();
                                                                move |_| {
                                                                    selected_donor.set(Some(d.clone()));
                                                                    solicitation_result.set(None);
                                                                }
                                                            },
                                                            td { class: "font-medium",
                                                                {donor.encrypted_name.as_deref().unwrap_or("N/A")}
                                                            }
                                                            td { {donor.encrypted_email.as_deref().unwrap_or("N/A")} }
                                                            td {
                                                                div { class: "flex items-center gap-2",
                                                                    progress {
                                                                        class: "progress progress-primary w-20",
                                                                        value: "{score_pct}",
                                                                        max: "100",
                                                                    }
                                                                    span { class: "text-xs text-slate-400",
                                                                        "{score_pct}%"
                                                                    }
                                                                }
                                                            }
                                                            td { "${donated:.2}" }
                                                            td { class: "text-sm",
                                                                {donor.last_contact.as_deref().unwrap_or("Never")}
                                                            }
                                                            td {
                                                                div { class: "flex flex-wrap gap-1",
                                                                    for tag in donor.tags.iter() {
                                                                        span { class: "badge badge-outline badge-sm", "{tag}" }
                                                                    }
                                                                }
                                                            }
                                                            td {
                                                                button {
                                                                    class: "btn btn-sm btn-ghost",
                                                                    onclick: {
                                                                        let did = donor_id.clone();
                                                                        move |e: Event<MouseData>| {
                                                                            e.stop_propagation();
                                                                            show_donation_modal.set(Some(did.clone()));
                                                                            donation_amount.set(String::new());
                                                                            donation_date.set(String::new());
                                                                            donation_note.set(String::new());
                                                                        }
                                                                    },
                                                                    "Record Donation"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                None => rsx! {
                                    div { class: "flex justify-center py-8",
                                        span { class: "loading loading-spinner loading-lg text-primary" }
                                    }
                                },
                            }
                        }
                    }
                }
            }

            // Inline donor detail / solicitation panel
            if let Some(donor) = selected_donor() {
                {
                    let donated = total_donated(&donor);
                    let score_pct = (donor.engagement_score * 100.0) as u32;
                    let history = donor.donation_history.as_array().cloned().unwrap_or_default();
                    let detail_donor_id = donor.id.clone();

                    rsx! {
                        div { class: "glass-card gradient-border",
                            div { class: "p-6",
                                div { class: "flex items-center justify-between mb-4",
                                    h2 { class: "card-title",
                                        {donor.encrypted_name.as_deref().unwrap_or("Unknown Donor")}
                                    }
                                    button {
                                        class: "btn btn-sm btn-ghost",
                                        onclick: move |_| selected_donor.set(None),
                                        "Close"
                                    }
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-6",
                                    div { class: "stat bg-slate-800/30 rounded-lg",
                                        div { class: "stat-title", "Email" }
                                        div { class: "stat-value text-sm",
                                            {donor.encrypted_email.as_deref().unwrap_or("N/A")}
                                        }
                                    }
                                    div { class: "stat bg-slate-800/30 rounded-lg",
                                        div { class: "stat-title", "Total Donated" }
                                        div { class: "stat-value text-lg", "${donated:.2}" }
                                    }
                                    div { class: "stat bg-slate-800/30 rounded-lg",
                                        div { class: "stat-title", "Engagement Score" }
                                        div { class: "stat-value text-lg", "{score_pct}%" }
                                    }
                                }

                                // Donation history table
                                if !history.is_empty() {
                                    h3 { class: "text-lg font-semibold mb-2", "Donation History" }
                                    div { class: "overflow-x-auto mb-6",
                                        table { class: "table table-sm",
                                            thead {
                                                tr {
                                                    th { "Date" }
                                                    th { "Amount" }
                                                    th { "Note" }
                                                }
                                            }
                                            tbody {
                                                for entry in history.iter().rev() {
                                                    {
                                                        let date_val = entry.get("date").and_then(|v| v.as_str()).unwrap_or("-");
                                                        let amt = entry.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                        let note_val = entry.get("note").and_then(|v| v.as_str()).unwrap_or("");
                                                        rsx! {
                                                            tr {
                                                                td { "{date_val}" }
                                                                td { "${amt:.2}" }
                                                                td { "{note_val}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Draft Solicitation section
                                div { class: "divider", "Draft Solicitation Email" }
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text", "Campaign Context" }
                                        }
                                        textarea {
                                            class: "textarea textarea-bordered h-24",
                                            placeholder: "Describe the campaign goal or initiative...",
                                            value: "{solicitation_context}",
                                            oninput: move |e| solicitation_context.set(e.value()),
                                        }
                                    }
                                    div { class: "form-control",
                                        label { class: "label",
                                            span { class: "label-text", "Ask Amount ($)" }
                                        }
                                        input {
                                            class: "input input-bordered",
                                            r#type: "number",
                                            placeholder: "500.00",
                                            value: "{solicitation_amount}",
                                            oninput: move |e| solicitation_amount.set(e.value()),
                                        }
                                    }
                                }
                                button {
                                    class: "btn btn-secondary",
                                    disabled: solicitation_loading(),
                                    onclick: {
                                        let did = detail_donor_id.clone();
                                        move |_| {
                                            let did = did.clone();
                                            let ctx = solicitation_context();
                                            let amt: f64 = solicitation_amount().parse().unwrap_or(0.0);
                                            if ctx.is_empty() || amt <= 0.0 {
                                                return;
                                            }
                                            solicitation_loading.set(true);
                                            solicitation_result.set(None);
                                            spawn(async move {
                                                match draft_solicitation(did, ctx, amt).await {
                                                    Ok(email) => solicitation_result.set(Some(email)),
                                                    Err(e) => solicitation_result.set(Some(format!("Error: {e}"))),
                                                }
                                                solicitation_loading.set(false);
                                            });
                                        }
                                    },
                                    if solicitation_loading() {
                                        span { class: "loading loading-spinner loading-sm mr-2" }
                                    }
                                    "Generate Email"
                                }

                                // Solicitation result
                                if let Some(email_body) = solicitation_result() {
                                    div { class: "mt-4 p-4 bg-slate-800/30 rounded-lg",
                                        h4 { class: "font-semibold mb-2", "Generated Solicitation" }
                                        pre { class: "whitespace-pre-wrap text-sm", "{email_body}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Add Donor Modal
        if show_add_modal() {
            div { class: "modal modal-open",
                div { class: "modal-box",
                    h3 { class: "font-bold text-lg mb-4", "Add New Donor" }
                    div { class: "form-control mb-3",
                        label { class: "label",
                            span { class: "label-text", "Name" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "Full name",
                            value: "{new_name}",
                            oninput: move |e| new_name.set(e.value()),
                        }
                    }
                    div { class: "form-control mb-3",
                        label { class: "label",
                            span { class: "label-text", "Email" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "email",
                            placeholder: "donor@example.com",
                            value: "{new_email}",
                            oninput: move |e| new_email.set(e.value()),
                        }
                    }
                    div { class: "form-control mb-4",
                        label { class: "label",
                            span { class: "label-text", "Tags (comma-separated)" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "vip, recurring, event-2024",
                            value: "{new_tags}",
                            oninput: move |e| new_tags.set(e.value()),
                        }
                    }
                    div { class: "modal-action",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| {
                                show_add_modal.set(false);
                                new_name.set(String::new());
                                new_email.set(String::new());
                                new_tags.set(String::new());
                            },
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: adding_donor(),
                            onclick: move |_| {
                                let name = new_name().trim().to_string();
                                let email = new_email().trim().to_string();
                                if name.is_empty() || email.is_empty() {
                                    return;
                                }
                                let tags: Vec<String> = new_tags()
                                    .split(',')
                                    .map(|t| t.trim().to_string())
                                    .filter(|t| !t.is_empty())
                                    .collect();
                                adding_donor.set(true);
                                spawn(async move {
                                    match create_donor(name, email, tags).await {
                                        Ok(_) => {
                                            donors_refresh += 1;
                                            summary_refresh += 1;
                                            show_add_modal.set(false);
                                            new_name.set(String::new());
                                            new_email.set(String::new());
                                            new_tags.set(String::new());
                                        }
                                        Err(e) => tracing::error!("Failed to create donor: {e}"),
                                    }
                                    adding_donor.set(false);
                                });
                            },
                            if adding_donor() {
                                span { class: "loading loading-spinner loading-sm mr-2" }
                            }
                            "Save Donor"
                        }
                    }
                }
                div {
                    class: "modal-backdrop",
                    onclick: move |_| show_add_modal.set(false),
                }
            }
        }

        // Record Donation Modal
        if let Some(donor_id) = show_donation_modal() {
            div { class: "modal modal-open",
                div { class: "modal-box",
                    h3 { class: "font-bold text-lg mb-4", "Record Donation" }
                    div { class: "form-control mb-3",
                        label { class: "label",
                            span { class: "label-text", "Amount ($)" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "number",
                            step: "0.01",
                            placeholder: "100.00",
                            value: "{donation_amount}",
                            oninput: move |e| donation_amount.set(e.value()),
                        }
                    }
                    div { class: "form-control mb-3",
                        label { class: "label",
                            span { class: "label-text", "Date" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "date",
                            value: "{donation_date}",
                            oninput: move |e| donation_date.set(e.value()),
                        }
                    }
                    div { class: "form-control mb-4",
                        label { class: "label",
                            span { class: "label-text", "Note" }
                        }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "Optional note",
                            value: "{donation_note}",
                            oninput: move |e| donation_note.set(e.value()),
                        }
                    }
                    div { class: "modal-action",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| show_donation_modal.set(None),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: recording_donation(),
                            onclick: {
                                let did = donor_id.clone();
                                move |_| {
                                    let did = did.clone();
                                    let amt: f64 = donation_amount().parse().unwrap_or(0.0);
                                    let date = donation_date().clone();
                                    let note = donation_note().clone();
                                    if amt <= 0.0 || date.is_empty() {
                                        return;
                                    }
                                    recording_donation.set(true);
                                    spawn(async move {
                                        match record_donation(did, amt, date, note).await {
                                            Ok(updated_donor) => {
                                                donors_refresh += 1;
                                                summary_refresh += 1;
                                                show_donation_modal.set(None);
                                                // Update inline detail if this donor is selected
                                                if let Some(sel) = selected_donor() {
                                                    if sel.id == updated_donor.id {
                                                        selected_donor.set(Some(updated_donor));
                                                    }
                                                }
                                            }
                                            Err(e) => tracing::error!("Failed to record donation: {e}"),
                                        }
                                        recording_donation.set(false);
                                    });
                                }
                            },
                            if recording_donation() {
                                span { class: "loading loading-spinner loading-sm mr-2" }
                            }
                            "Save Donation"
                        }
                    }
                }
                div {
                    class: "modal-backdrop",
                    onclick: move |_| show_donation_modal.set(None),
                }
            }
        }
    }
}

/// Summary stat cards displayed at the top of the fundraising page.
#[component]
fn SummaryCards(summary: FundraisingSummary) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4",
            div { class: "stat bg-base-100 shadow-sm rounded-lg",
                div { class: "stat-title", "Total Donors" }
                div { class: "stat-value text-primary", "{summary.total_donors}" }
            }
            div { class: "stat bg-base-100 shadow-sm rounded-lg",
                div { class: "stat-title", "Total Raised" }
                div { class: "stat-value text-secondary", "${summary.total_raised:.2}" }
            }
            div { class: "stat bg-base-100 shadow-sm rounded-lg",
                div { class: "stat-title", "Avg Donation" }
                div { class: "stat-value text-accent", "${summary.average_donation:.2}" }
            }
        }

        // Top donors
        if !summary.top_donors.is_empty() {
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    h3 { class: "card-title text-lg", "Top Donors" }
                    div { class: "overflow-x-auto",
                        table { class: "table table-sm",
                            thead {
                                tr {
                                    th { "#" }
                                    th { "Name" }
                                    th { "Total Donated" }
                                }
                            }
                            tbody {
                                for (i, td) in summary.top_donors.iter().enumerate() {
                                    tr {
                                        td { "{i + 1}" }
                                        td { class: "font-medium", "{td.name}" }
                                        td { "${td.total_donated:.2}" }
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
