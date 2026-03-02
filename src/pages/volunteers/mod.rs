use dioxus::prelude::*;

use crate::app::Route;
use crate::components::LoadingSpinner;
use crate::models::task::{Assignment, Task, TaskSummary};
use crate::models::volunteer::{Volunteer, VolunteerMatch, VolunteerSummary};
use crate::modules::volunteer_matching::{
    assign_volunteer, create_task, create_volunteer, draft_message, get_at_risk_volunteers,
    get_task, get_volunteer, list_tasks, list_volunteers, match_task,
};

// ---------------------------------------------------------------------------
// F01-A: Volunteer List Page
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum VolunteerTab {
    All,
    AtRisk,
}

#[component]
pub fn VolunteersPage() -> Element {
    let mut search_text = use_signal(String::new);
    let mut status_filter = use_signal(|| String::new());
    let mut active_tab = use_signal(|| VolunteerTab::All);
    let mut show_add_modal = use_signal(|| false);
    let mut refresh_tick = use_signal(|| 0_u32);

    // Fetch all volunteers
    let volunteers_resource = use_resource(move || {
        let search = search_text();
        let status = status_filter();
        let _tick = refresh_tick();
        async move {
            let status_opt = if status.is_empty() {
                None
            } else {
                Some(status)
            };
            let search_opt = if search.is_empty() {
                None
            } else {
                Some(search)
            };
            list_volunteers(search_opt, status_opt, None)
                .await
                .unwrap_or_default()
        }
    });

    // Fetch at-risk volunteers
    let at_risk_resource = use_resource(move || {
        let _tick = refresh_tick();
        async move { get_at_risk_volunteers().await.unwrap_or_default() }
    });

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold", "Volunteer Management" }
                    p { class: "text-slate-400",
                        "Match and coordinate campaign volunteers with AI-driven task assignment."
                    }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_add_modal.set(true),
                    "+ Add Volunteer"
                }
            }

            // Tab switcher
            div { class: "tabs tabs-boxed w-fit",
                button {
                    class: if *active_tab.read() == VolunteerTab::All {
                        "tab tab-active"
                    } else {
                        "tab"
                    },
                    onclick: move |_| active_tab.set(VolunteerTab::All),
                    "All Volunteers"
                }
                button {
                    class: if *active_tab.read() == VolunteerTab::AtRisk {
                        "tab tab-active"
                    } else {
                        "tab"
                    },
                    onclick: move |_| active_tab.set(VolunteerTab::AtRisk),
                    "At Risk"
                }
            }

            // Filters (shown for All tab)
            if *active_tab.read() == VolunteerTab::All {
                div { class: "flex flex-wrap gap-3",
                    input {
                        class: "input input-bordered input-sm w-64",
                        r#type: "text",
                        placeholder: "Search by name or email...",
                        value: "{search_text}",
                        oninput: move |evt| search_text.set(evt.value()),
                    }
                    select {
                        class: "select select-bordered select-sm",
                        value: "{status_filter}",
                        onchange: move |evt: Event<FormData>| {
                            status_filter.set(evt.value());
                        },
                        option { value: "", "All Statuses" }
                        option { value: "active", "Active" }
                        option { value: "inactive", "Inactive" }
                    }
                }
            }

            // Volunteer table
            match *active_tab.read() {
                VolunteerTab::All => rsx! {
                    VolunteerTable { volunteers: volunteers_resource }
                },
                VolunteerTab::AtRisk => rsx! {
                    AtRiskTable { volunteers: at_risk_resource }
                },
            }

            // Add volunteer modal
            if *show_add_modal.read() {
                AddVolunteerModal {
                    on_close: move || show_add_modal.set(false),
                    on_success: move || {
                        show_add_modal.set(false);
                        refresh_tick += 1;
                    },
                }
            }
        }
    }
}

#[component]
fn VolunteerTable(volunteers: Resource<Vec<VolunteerSummary>>) -> Element {
    let data = volunteers.read();

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                match data.as_ref() {
                    None => rsx! { LoadingSpinner {} },
                    Some(items) if items.is_empty() => {
                        rsx! {
                            p { class: "text-slate-400 py-4", "No volunteers found." }
                        }
                    }
                    Some(items) => {
                        rsx! {
                            div { class: "overflow-x-auto",
                                table { class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "Name" }
                                            th { "Email" }
                                            th { "Skills" }
                                            th { "Status" }
                                            th { "Churn Risk" }
                                            th { "Last Active" }
                                        }
                                    }
                                    tbody {
                                        for vol in items {
                                            VolunteerRow { volunteer: vol.clone() }
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

#[component]
fn AtRiskTable(volunteers: Resource<Vec<VolunteerSummary>>) -> Element {
    let data = volunteers.read();

    rsx! {
        div { class: "glass-card gradient-border border border-warning/30",
            div { class: "p-6",
                div { class: "flex items-center gap-2 mb-4",
                    span { class: "text-warning text-lg", "!" }
                    h3 { class: "text-lg font-semibold text-warning", "At-Risk Volunteers (Churn Score > 0.7)" }
                }

                match data.as_ref() {
                    None => rsx! { LoadingSpinner {} },
                    Some(items) if items.is_empty() => {
                        rsx! {
                            p { class: "text-slate-400 py-4", "No at-risk volunteers detected." }
                        }
                    }
                    Some(items) => {
                        rsx! {
                            div { class: "overflow-x-auto",
                                table { class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "Name" }
                                            th { "Email" }
                                            th { "Skills" }
                                            th { "Status" }
                                            th { "Churn Risk" }
                                            th { "Last Active" }
                                        }
                                    }
                                    tbody {
                                        for vol in items {
                                            VolunteerRow { volunteer: vol.clone(), at_risk: true }
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

#[component]
fn VolunteerRow(volunteer: VolunteerSummary, #[props(default = false)] at_risk: bool) -> Element {
    let status_badge = match volunteer.status.as_str() {
        "active" => "badge badge-success badge-sm",
        "inactive" => "badge badge-ghost badge-sm",
        _ => "badge badge-outline badge-sm",
    };

    let churn_text = format!("{:.0}%", volunteer.churn_score * 100.0);
    let churn_class = if volunteer.churn_score > 0.7 {
        "text-error font-semibold"
    } else if volunteer.churn_score > 0.4 {
        "text-warning"
    } else {
        "text-success"
    };

    let last_active = volunteer
        .last_active
        .clone()
        .unwrap_or_else(|| "--".to_string());

    let row_class = if at_risk {
        "hover bg-warning/5"
    } else {
        "hover"
    };

    let vol_id = volunteer.id.clone();

    rsx! {
        tr { class: "{row_class}",
            td {
                Link {
                    to: Route::VolunteerDetailPage { id: vol_id },
                    class: "link link-primary font-medium",
                    "{volunteer.name}"
                }
            }
            td { class: "text-sm", "{volunteer.email}" }
            td {
                div { class: "flex flex-wrap gap-1",
                    for skill in volunteer.skills.iter().take(3) {
                        span { class: "badge badge-outline badge-xs", "{skill}" }
                    }
                    {
                        let overflow = volunteer.skills.len().saturating_sub(3);
                        if volunteer.skills.len() > 3 {
                            rsx! {
                                span { class: "badge badge-ghost badge-xs",
                                    "+{overflow}"
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                }
            }
            td { span { class: "{status_badge}", "{volunteer.status}" } }
            td { class: "{churn_class} tabular-nums", "{churn_text}" }
            td { class: "text-xs whitespace-nowrap", "{last_active}" }
        }
    }
}

#[component]
fn AddVolunteerModal(on_close: EventHandler, on_success: EventHandler) -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut phone = use_signal(String::new);
    let mut skills_text = use_signal(String::new);
    let mut availability = use_signal(String::new);
    let mut location = use_signal(String::new);
    let mut tags_text = use_signal(String::new);
    let mut bio = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_submit = move |_| {
        let v_name = name().trim().to_string();
        let v_email = email().trim().to_string();

        if v_name.is_empty() || v_email.is_empty() {
            error_msg.set(Some("Name and email are required.".to_string()));
            return;
        }

        let v_phone = phone().trim().to_string();
        let v_skills: Vec<String> = skills_text()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let v_availability = availability().trim().to_string();
        let v_location = location().trim().to_string();
        let v_tags: Vec<String> = tags_text()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let v_bio = bio().trim().to_string();

        is_submitting.set(true);
        error_msg.set(None);

        spawn(async move {
            let v_phone_opt = if v_phone.is_empty() {
                None
            } else {
                Some(v_phone)
            };
            let v_availability_json = serde_json::Value::String(v_availability);
            let v_location_opt = if v_location.is_empty() {
                None
            } else {
                Some(serde_json::Value::String(v_location))
            };
            let v_bio_opt = if v_bio.is_empty() { None } else { Some(v_bio) };
            match create_volunteer(
                v_name,
                v_email,
                v_phone_opt,
                v_skills,
                v_availability_json,
                v_location_opt,
                v_tags,
                v_bio_opt,
            )
            .await
            {
                Ok(_) => {
                    on_success.call(());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to create volunteer: {e}")));
                }
            }
            is_submitting.set(false);
        });
    };

    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box w-11/12 max-w-2xl",
                h3 { class: "font-bold text-lg mb-4", "Add New Volunteer" }

                div { class: "space-y-3",
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Name *" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "Full name",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Email *" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "email",
                            placeholder: "email@example.com",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Phone" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "tel",
                            placeholder: "+1 555-0100",
                            value: "{phone}",
                            oninput: move |evt| phone.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Skills (comma-separated)" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "canvassing, data entry, social media",
                            value: "{skills_text}",
                            oninput: move |evt| skills_text.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Availability" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "weekends, evenings",
                            value: "{availability}",
                            oninput: move |evt| availability.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Location" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "City, State",
                            value: "{location}",
                            oninput: move |evt| location.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Tags (comma-separated)" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "spanish-speaker, driver",
                            value: "{tags_text}",
                            oninput: move |evt| tags_text.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Bio" } }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Brief description of the volunteer...",
                            rows: "3",
                            value: "{bio}",
                            oninput: move |evt| bio.set(evt.value()),
                        }
                    }
                }

                if let Some(err) = error_msg() {
                    div { class: "alert alert-error mt-4 text-sm", "{err}" }
                }

                div { class: "modal-action",
                    button {
                        class: "btn",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: *is_submitting.read(),
                        onclick: on_submit,
                        if *is_submitting.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Creating..."
                        } else {
                            "Create Volunteer"
                        }
                    }
                }
            }
            // Backdrop
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

// ---------------------------------------------------------------------------
// F01-B: Volunteer Detail Page
// ---------------------------------------------------------------------------

#[component]
pub fn VolunteerDetailPage(id: String) -> Element {
    let vol_id = id.clone();
    let volunteer_resource = use_resource(move || {
        let vid = vol_id.clone();
        async move { get_volunteer(vid).await.ok() }
    });

    let mut message_type = use_signal(|| "outreach".to_string());
    let mut generated_message = use_signal(|| Option::<String>::None);
    let mut is_generating = use_signal(|| false);
    let mut gen_error = use_signal(|| Option::<String>::None);

    let draft_id = id.clone();
    let on_generate = move |_| {
        let vid = draft_id.clone();
        let msg_type = message_type();
        is_generating.set(true);
        gen_error.set(None);
        generated_message.set(None);

        spawn(async move {
            match draft_message(vid, msg_type, None).await {
                Ok(msg) => {
                    generated_message.set(Some(msg));
                }
                Err(e) => {
                    gen_error.set(Some(format!("Failed to generate message: {e}")));
                }
            }
            is_generating.set(false);
        });
    };

    let data = volunteer_resource.read();

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Back link
            div {
                Link {
                    to: Route::VolunteersPage {},
                    class: "btn btn-ghost btn-sm gap-1",
                    "< Back to Volunteers"
                }
            }

            match data.as_ref() {
                None => rsx! { LoadingSpinner {} },
                Some(None) => rsx! {
                    div { class: "alert alert-error", "Volunteer not found." }
                },
                Some(Some(vol)) => rsx! {
                    // Header card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                                div {
                                    h1 { class: "text-2xl font-bold", "{vol.name}" }
                                    p { class: "text-slate-400", "{vol.email}" }
                                    if let Some(ref phone) = vol.phone {
                                        p { class: "text-slate-400 text-sm", "{phone}" }
                                    }
                                }
                                div { class: "flex items-center gap-2",
                                    {
                                        let status_badge = match vol.status.as_str() {
                                            "active" => "badge badge-success",
                                            "inactive" => "badge badge-ghost",
                                            _ => "badge badge-outline",
                                        };
                                        rsx! { span { class: "{status_badge}", "{vol.status}" } }
                                    }
                                    {
                                        let churn_text = format!("{:.0}% churn risk", vol.churn_score * 100.0);
                                        let churn_badge = if vol.churn_score > 0.7 {
                                            "badge badge-error"
                                        } else if vol.churn_score > 0.4 {
                                            "badge badge-warning"
                                        } else {
                                            "badge badge-success"
                                        };
                                        rsx! { span { class: "{churn_badge}", "{churn_text}" } }
                                    }
                                }
                            }
                        }
                    }

                    // Skills and tags
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Skills" }
                            div { class: "flex flex-wrap gap-2 mt-2",
                                for skill in vol.skills.iter() {
                                    span { class: "badge badge-primary badge-outline", "{skill}" }
                                }
                                if vol.skills.is_empty() {
                                    span { class: "text-slate-500 text-sm", "No skills listed" }
                                }
                            }
                            if !vol.tags.is_empty() {
                                h2 { class: "card-title text-lg mt-4", "Tags" }
                                div { class: "flex flex-wrap gap-2 mt-2",
                                    for tag in vol.tags.iter() {
                                        span { class: "badge badge-secondary badge-outline badge-sm", "{tag}" }
                                    }
                                }
                            }
                        }
                    }

                    // Bio
                    if vol.bio.is_some() {
                        div { class: "glass-card gradient-border",
                            div { class: "p-6",
                                h2 { class: "card-title text-lg", "Bio" }
                                p { class: "text-base-content/80 mt-2 whitespace-pre-wrap",
                                    "{vol.bio.as_deref().unwrap_or(\"\")}"
                                }
                            }
                        }
                    }

                    // Draft message section
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Draft Message" }
                            p { class: "text-slate-400 text-sm mb-3",
                                "Generate an AI-drafted message for this volunteer."
                            }
                            div { class: "flex flex-wrap items-end gap-3",
                                div { class: "form-control",
                                    label { class: "label", span { class: "label-text", "Message Type" } }
                                    select {
                                        class: "select select-bordered select-sm",
                                        value: "{message_type}",
                                        onchange: move |evt: Event<FormData>| {
                                            message_type.set(evt.value());
                                        },
                                        option { value: "outreach", "Outreach" }
                                        option { value: "retention", "Retention" }
                                        option { value: "thanks", "Thank You" }
                                    }
                                }
                                button {
                                    class: "btn btn-primary btn-sm",
                                    disabled: *is_generating.read(),
                                    onclick: on_generate,
                                    if *is_generating.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                        "Generating..."
                                    } else {
                                        "Generate"
                                    }
                                }
                            }

                            if let Some(err) = gen_error() {
                                div { class: "alert alert-error mt-4 text-sm", "{err}" }
                            }

                            if let Some(msg) = generated_message() {
                                div { class: "card bg-slate-800/30 mt-4",
                                    div { class: "p-6",
                                        p { class: "whitespace-pre-wrap text-sm", "{msg}" }
                                    }
                                }
                            }
                        }
                    }

                    // Metadata
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            h2 { class: "card-title text-lg", "Details" }
                            div { class: "grid grid-cols-2 gap-2 text-sm mt-2",
                                span { class: "text-slate-400", "Created" }
                                span { "{vol.created_at.as_deref().unwrap_or(\"--\")}" }
                                span { class: "text-slate-400", "Last Active" }
                                span { "{vol.last_active.as_deref().unwrap_or(\"--\")}" }
                            }
                        }
                    }
                },
            }
        }
    }
}

// ---------------------------------------------------------------------------
// F01-C: Tasks List Page
// ---------------------------------------------------------------------------

#[component]
pub fn TasksPage() -> Element {
    let mut search_text = use_signal(String::new);
    let mut status_filter = use_signal(String::new);
    let mut show_create_modal = use_signal(|| false);
    let mut refresh_tick = use_signal(|| 0_u32);

    let tasks_resource = use_resource(move || {
        let status = status_filter();
        let search = search_text();
        let _tick = refresh_tick();
        async move {
            let status_opt = if status.is_empty() {
                None
            } else {
                Some(status)
            };
            let search_opt = if search.is_empty() {
                None
            } else {
                Some(search)
            };
            list_tasks(status_opt, search_opt).await.unwrap_or_default()
        }
    });

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold", "Tasks" }
                    p { class: "text-slate-400",
                        "View and manage campaign tasks and volunteer assignments."
                    }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_create_modal.set(true),
                    "+ Create Task"
                }
            }

            // Filters
            div { class: "flex flex-wrap gap-3",
                input {
                    class: "input input-bordered input-sm w-64",
                    r#type: "text",
                    placeholder: "Search tasks...",
                    value: "{search_text}",
                    oninput: move |evt| search_text.set(evt.value()),
                }
                select {
                    class: "select select-bordered select-sm",
                    value: "{status_filter}",
                    onchange: move |evt: Event<FormData>| {
                        status_filter.set(evt.value());
                    },
                    option { value: "", "All Statuses" }
                    option { value: "open", "Open" }
                    option { value: "in_progress", "In Progress" }
                    option { value: "completed", "Completed" }
                    option { value: "cancelled", "Cancelled" }
                }
            }

            // Tasks table
            div { class: "glass-card gradient-border",
                div { class: "p-6",
                    {
                        let data = tasks_resource.read();
                        match data.as_ref() {
                            None => rsx! { LoadingSpinner {} },
                            Some(items) if items.is_empty() => {
                                rsx! {
                                    p { class: "text-slate-400 py-4", "No tasks found." }
                                }
                            }
                            Some(items) => {
                                rsx! {
                                    div { class: "overflow-x-auto",
                                        table { class: "table table-sm",
                                            thead {
                                                tr {
                                                    th { "Title" }
                                                    th { "Required Skills" }
                                                    th { "Status" }
                                                    th { "Start Date" }
                                                    th { "Assigned / Max" }
                                                }
                                            }
                                            tbody {
                                                for task in items {
                                                    TaskRow { task: task.clone() }
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

            // Create task modal
            if *show_create_modal.read() {
                CreateTaskModal {
                    on_close: move || show_create_modal.set(false),
                    on_success: move || {
                        show_create_modal.set(false);
                        refresh_tick += 1;
                    },
                }
            }
        }
    }
}

#[component]
fn TaskRow(task: TaskSummary) -> Element {
    let status_badge = match task.status.as_str() {
        "open" => "badge badge-info badge-sm",
        "in_progress" => "badge badge-warning badge-sm",
        "completed" => "badge badge-success badge-sm",
        "cancelled" => "badge badge-ghost badge-sm",
        _ => "badge badge-outline badge-sm",
    };

    let date_text = task.date_start.clone().unwrap_or_else(|| "--".to_string());
    let assignment_text = format!("{}/{}", task.assigned_count, task.max_volunteers);
    let task_id = task.id.clone();

    rsx! {
        tr { class: "hover",
            td {
                Link {
                    to: Route::TaskDetailPage { id: task_id },
                    class: "link link-primary font-medium",
                    "{task.title}"
                }
            }
            td {
                div { class: "flex flex-wrap gap-1",
                    for skill in task.required_skills.iter().take(3) {
                        span { class: "badge badge-outline badge-xs", "{skill}" }
                    }
                    {
                        let overflow = task.required_skills.len().saturating_sub(3);
                        if task.required_skills.len() > 3 {
                            rsx! {
                                span { class: "badge badge-ghost badge-xs",
                                    "+{overflow}"
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                }
            }
            td { span { class: "{status_badge}", "{task.status}" } }
            td { class: "text-xs whitespace-nowrap", "{date_text}" }
            td { class: "tabular-nums", "{assignment_text}" }
        }
    }
}

#[component]
fn CreateTaskModal(on_close: EventHandler, on_success: EventHandler) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut skills_text = use_signal(String::new);
    let mut location = use_signal(String::new);
    let mut date_start = use_signal(String::new);
    let mut date_end = use_signal(String::new);
    let mut max_volunteers = use_signal(|| "5".to_string());
    let mut is_submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_submit = move |_| {
        let t_title = title().trim().to_string();
        let t_desc = description().trim().to_string();

        if t_title.is_empty() {
            error_msg.set(Some("Title is required.".to_string()));
            return;
        }

        let t_skills: Vec<String> = skills_text()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let t_location = location().trim().to_string();
        let t_start = date_start().trim().to_string();
        let t_end = date_end().trim().to_string();
        let t_max: i32 = max_volunteers().trim().parse().unwrap_or(5);

        is_submitting.set(true);
        error_msg.set(None);

        spawn(async move {
            let t_location_opt = if t_location.is_empty() {
                None
            } else {
                Some(serde_json::Value::String(t_location))
            };
            let t_start_opt = if t_start.is_empty() {
                None
            } else {
                Some(t_start)
            };
            let t_end_opt = if t_end.is_empty() { None } else { Some(t_end) };
            match create_task(
                t_title,
                t_desc,
                t_skills,
                t_location_opt,
                t_start_opt,
                t_end_opt,
                t_max,
            )
            .await
            {
                Ok(_) => {
                    on_success.call(());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to create task: {e}")));
                }
            }
            is_submitting.set(false);
        });
    };

    rsx! {
        div { class: "modal modal-open",
            div { class: "modal-box w-11/12 max-w-2xl",
                h3 { class: "font-bold text-lg mb-4", "Create New Task" }

                div { class: "space-y-3",
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Title *" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "Task title",
                            value: "{title}",
                            oninput: move |evt| title.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Description" } }
                        textarea {
                            class: "textarea textarea-bordered w-full",
                            placeholder: "Describe the task requirements...",
                            rows: "3",
                            value: "{description}",
                            oninput: move |evt| description.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Required Skills (comma-separated)" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "canvassing, phone banking",
                            value: "{skills_text}",
                            oninput: move |evt| skills_text.set(evt.value()),
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Location" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "text",
                            placeholder: "City, State",
                            value: "{location}",
                            oninput: move |evt| location.set(evt.value()),
                        }
                    }
                    div { class: "grid grid-cols-2 gap-3",
                        div { class: "form-control",
                            label { class: "label", span { class: "label-text", "Start Date" } }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "date",
                                value: "{date_start}",
                                oninput: move |evt| date_start.set(evt.value()),
                            }
                        }
                        div { class: "form-control",
                            label { class: "label", span { class: "label-text", "End Date" } }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "date",
                                value: "{date_end}",
                                oninput: move |evt| date_end.set(evt.value()),
                            }
                        }
                    }
                    div { class: "form-control",
                        label { class: "label", span { class: "label-text", "Max Volunteers" } }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "number",
                            min: "1",
                            value: "{max_volunteers}",
                            oninput: move |evt| max_volunteers.set(evt.value()),
                        }
                    }
                }

                if let Some(err) = error_msg() {
                    div { class: "alert alert-error mt-4 text-sm", "{err}" }
                }

                div { class: "modal-action",
                    button {
                        class: "btn",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: *is_submitting.read(),
                        onclick: on_submit,
                        if *is_submitting.read() {
                            span { class: "loading loading-spinner loading-sm" }
                            "Creating..."
                        } else {
                            "Create Task"
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

// ---------------------------------------------------------------------------
// F01-D: Task Detail Page
// ---------------------------------------------------------------------------

#[component]
pub fn TaskDetailPage(id: String) -> Element {
    let task_id = id.clone();
    let mut refresh_tick = use_signal(|| 0_u32);

    let task_resource = use_resource(move || {
        let tid = task_id.clone();
        let _tick = refresh_tick();
        async move { get_task(tid).await.ok() }
    });

    let mut matches = use_signal(|| Vec::<VolunteerMatch>::new());
    let mut is_matching = use_signal(|| false);
    let mut match_error = use_signal(|| Option::<String>::None);
    let mut assign_error = use_signal(|| Option::<String>::None);

    let match_id = id.clone();
    let on_find_matches = move |_| {
        let tid = match_id.clone();
        is_matching.set(true);
        match_error.set(None);
        matches.set(Vec::new());

        spawn(async move {
            match match_task(tid, Some(10)).await {
                Ok(results) => {
                    matches.set(results);
                }
                Err(e) => {
                    match_error.set(Some(format!("Matching failed: {e}")));
                }
            }
            is_matching.set(false);
        });
    };

    let data = task_resource.read();

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Back link
            div {
                Link {
                    to: Route::TasksPage {},
                    class: "btn btn-ghost btn-sm gap-1",
                    "< Back to Tasks"
                }
            }

            match data.as_ref() {
                None => rsx! { LoadingSpinner {} },
                Some(None) => rsx! {
                    div { class: "alert alert-error", "Task not found." }
                },
                Some(Some(task)) => rsx! {
                    // Task header card
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                                div {
                                    h1 { class: "text-2xl font-bold", "{task.title}" }
                                    if !task.description.is_empty() {
                                        p { class: "text-slate-400 mt-1", "{task.description}" }
                                    }
                                }
                                {
                                    let status_badge = match task.status.as_str() {
                                        "open" => "badge badge-info badge-lg",
                                        "in_progress" => "badge badge-warning badge-lg",
                                        "completed" => "badge badge-success badge-lg",
                                        "cancelled" => "badge badge-ghost badge-lg",
                                        _ => "badge badge-outline badge-lg",
                                    };
                                    rsx! { span { class: "{status_badge}", "{task.status}" } }
                                }
                            }

                            // Task details grid
                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 mt-4",
                                div {
                                    span { class: "text-xs text-slate-500 block", "Start Date" }
                                    span { class: "text-sm font-medium",
                                        "{task.date_start.as_deref().unwrap_or(\"--\")}"
                                    }
                                }
                                div {
                                    span { class: "text-xs text-slate-500 block", "End Date" }
                                    span { class: "text-sm font-medium",
                                        "{task.date_end.as_deref().unwrap_or(\"--\")}"
                                    }
                                }
                                div {
                                    span { class: "text-xs text-slate-500 block", "Max Volunteers" }
                                    span { class: "text-sm font-medium", "{task.max_volunteers}" }
                                }
                                div {
                                    span { class: "text-xs text-slate-500 block", "Created" }
                                    span { class: "text-sm font-medium",
                                        "{task.created_at.as_deref().unwrap_or(\"--\")}"
                                    }
                                }
                            }

                            // Required skills
                            if !task.required_skills.is_empty() {
                                div { class: "mt-4",
                                    span { class: "text-xs text-slate-500 block mb-1", "Required Skills" }
                                    div { class: "flex flex-wrap gap-2",
                                        for skill in task.required_skills.iter() {
                                            span { class: "badge badge-primary badge-outline", "{skill}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Find matches section
                    div { class: "glass-card gradient-border",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between",
                                h2 { class: "card-title text-lg", "Volunteer Matching" }
                                button {
                                    class: "btn btn-primary btn-sm",
                                    disabled: *is_matching.read(),
                                    onclick: on_find_matches,
                                    if *is_matching.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                        "Finding Matches..."
                                    } else {
                                        "Find Matches"
                                    }
                                }
                            }

                            if let Some(err) = match_error() {
                                div { class: "alert alert-error mt-4 text-sm", "{err}" }
                            }

                            if let Some(err) = assign_error() {
                                div { class: "alert alert-error mt-4 text-sm", "{err}" }
                            }

                            if !matches().is_empty() {
                                div { class: "overflow-x-auto mt-4",
                                    table { class: "table table-sm",
                                        thead {
                                            tr {
                                                th { "Volunteer" }
                                                th { "Overall Score" }
                                                th { "Semantic" }
                                                th { "Availability" }
                                                th { "Proximity" }
                                                th { "Actions" }
                                            }
                                        }
                                        tbody {
                                            for m in matches().iter() {
                                                {
                                                    let vol_name = m.volunteer.name.clone();
                                                    let vol_id = m.volunteer.id.clone();
                                                    let score_text = format!("{:.1}%", m.score * 100.0);
                                                    let semantic_text = format!("{:.2}", m.score_breakdown.semantic);
                                                    let avail_text = format!("{:.2}", m.score_breakdown.availability);
                                                    let prox_text = format!("{:.2}", m.score_breakdown.proximity);
                                                    let assign_task_id = id.clone();
                                                    let assign_vol_id = vol_id.clone();

                                                    rsx! {
                                                        tr { class: "hover",
                                                            td {
                                                                Link {
                                                                    to: Route::VolunteerDetailPage { id: vol_id },
                                                                    class: "link link-primary",
                                                                    "{vol_name}"
                                                                }
                                                            }
                                                            td { class: "font-semibold tabular-nums", "{score_text}" }
                                                            td { class: "tabular-nums text-sm", "{semantic_text}" }
                                                            td { class: "tabular-nums text-sm", "{avail_text}" }
                                                            td { class: "tabular-nums text-sm", "{prox_text}" }
                                                            td {
                                                                button {
                                                                    class: "btn btn-success btn-xs",
                                                                    onclick: move |_| {
                                                                        let tid = assign_task_id.clone();
                                                                        let vid = assign_vol_id.clone();
                                                                        spawn(async move {
                                                                            match assign_volunteer(tid, vid).await {
                                                                                Ok(_) => {
                                                                                    refresh_tick += 1;
                                                                                }
                                                                                Err(e) => {
                                                                                    assign_error.set(Some(format!("Assignment failed: {e}")));
                                                                                }
                                                                            }
                                                                        });
                                                                    },
                                                                    "Assign"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if !*is_matching.read() && match_error().is_none() {
                                p { class: "text-slate-500 text-sm mt-4",
                                    "Click \"Find Matches\" to discover the best-fit volunteers for this task."
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
