use dioxus::prelude::*;

use crate::components::LoadingSpinner;
use crate::modules::local_issues::{
    analyze_local_issues, generate_local_talking_points, LocalIssueReport, TalkingPoints,
};

/// Map severity level to a DaisyUI badge class.
fn severity_badge_class(severity: &str) -> &'static str {
    match severity.to_lowercase().as_str() {
        "critical" => "badge badge-error",
        "high" => "badge badge-warning",
        "moderate" => "badge badge-info",
        "low" => "badge badge-ghost",
        _ => "badge badge-ghost",
    }
}

#[component]
pub fn LocalIssuesPage() -> Element {
    // --- Analysis form state ---
    let mut area_description = use_signal(String::new);
    let mut demographics = use_signal(String::new);
    let mut recent_news = use_signal(String::new);
    let mut is_analyzing = use_signal(|| false);
    let mut analysis_error = use_signal(|| Option::<String>::None);
    let mut report = use_signal(|| Option::<LocalIssueReport>::None);

    // --- Talking points expansion state ---
    let mut expanded_issue = use_signal(|| Option::<String>::None);
    let mut is_loading_points = use_signal(|| false);
    let mut talking_points_result = use_signal(|| Option::<TalkingPoints>::None);
    let mut points_error = use_signal(|| Option::<String>::None);

    // Handle analyze
    let on_analyze = move |_| {
        let area = area_description().trim().to_string();
        let demo = demographics().trim().to_string();
        let news = recent_news().trim().to_string();

        if area.is_empty() {
            analysis_error.set(Some("Area description is required.".to_string()));
            return;
        }

        let demo_opt = if demo.is_empty() { None } else { Some(demo) };
        let news_opt = if news.is_empty() { None } else { Some(news) };

        is_analyzing.set(true);
        analysis_error.set(None);

        spawn(async move {
            match analyze_local_issues(area, demo_opt, news_opt).await {
                Ok(result) => {
                    report.set(Some(result));
                    expanded_issue.set(None);
                    talking_points_result.set(None);
                }
                Err(e) => {
                    analysis_error.set(Some(format!("Analysis failed: {e}")));
                }
            }
            is_analyzing.set(false);
        });
    };

    // Handle generate detailed talking points for a specific issue
    let mut on_expand_issue = move |issue_title: String| {
        let area = if let Some(ref r) = report() {
            r.area_description.clone()
        } else {
            return;
        };

        // Toggle off if already expanded
        if expanded_issue() == Some(issue_title.clone()) {
            expanded_issue.set(None);
            talking_points_result.set(None);
            return;
        }

        expanded_issue.set(Some(issue_title.clone()));
        is_loading_points.set(true);
        points_error.set(None);
        talking_points_result.set(None);

        spawn(async move {
            match generate_local_talking_points(issue_title, area).await {
                Ok(result) => {
                    talking_points_result.set(Some(result));
                }
                Err(e) => {
                    points_error.set(Some(format!("Failed to generate talking points: {e}")));
                }
            }
            is_loading_points.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Hyper-Local Issue Mapping" }
                p { class: "text-slate-400",
                    "Identify neighborhood-level issues and generate actionable talking points for your campaign."
                }
            }

            // Error alert
            if let Some(err) = analysis_error() {
                div { class: "alert alert-error shadow-sm",
                    svg {
                        class: "stroke-current shrink-0 h-6 w-6",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { "{err}" }
                    button {
                        class: "btn btn-ghost btn-xs",
                        onclick: move |_| analysis_error.set(None),
                        "Dismiss"
                    }
                }
            }

            // Main layout
            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Input Form
                div { class: "w-full lg:w-1/3",
                    div { class: "glass-card gradient-border",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Area Analysis" }

                            // Area description
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Area Description" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "e.g., Downtown district, Ward 5 - mixed residential and commercial area near the waterfront...",
                                    rows: "4",
                                    value: "{area_description}",
                                    oninput: move |evt| area_description.set(evt.value()),
                                }
                            }

                            // Demographics
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Demographics" }
                                    span { class: "label-text-alt text-slate-500", "Optional" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "e.g., 60% homeowners, median age 42, significant retiree population, growing immigrant community...",
                                    rows: "3",
                                    value: "{demographics}",
                                    oninput: move |evt| demographics.set(evt.value()),
                                }
                            }

                            // Recent news
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Recent Local News" }
                                    span { class: "label-text-alt text-slate-500", "Optional" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "e.g., New factory closure announced, school board redistricting vote, water main break on Oak Street...",
                                    rows: "3",
                                    value: "{recent_news}",
                                    oninput: move |evt| recent_news.set(evt.value()),
                                }
                            }

                            // Analyze button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_analyzing.read(),
                                onclick: on_analyze,
                                if *is_analyzing.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Analyzing..."
                                } else {
                                    "Analyze Issues"
                                }
                            }
                        }
                    }
                }

                // Right: Results
                div { class: "w-full lg:w-2/3 space-y-4",

                    if *is_analyzing.read() {
                        div { class: "glass-card gradient-border",
                            div { class: "card-body flex items-center justify-center py-12",
                                div { class: "text-center space-y-4",
                                    LoadingSpinner {}
                                    p { class: "text-slate-400", "Analyzing local issues..." }
                                }
                            }
                        }
                    } else if let Some(rpt) = report() {

                        // Priority summary
                        if !rpt.overall_priorities.is_empty() {
                            div { class: "glass-card gradient-border",
                                div { class: "p-6",
                                    h2 { class: "card-title text-lg mb-3", "Priority Summary" }
                                    ol { class: "list-decimal list-inside space-y-1",
                                        for priority in &rpt.overall_priorities {
                                            li { class: "text-sm", "{priority}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Issue cards
                        if rpt.issues.is_empty() {
                            div { class: "glass-card gradient-border",
                                div { class: "p-6",
                                    p { class: "text-slate-500", "No issues were identified." }
                                }
                            }
                        } else {
                            for issue in &rpt.issues {
                                {
                                    let title = issue.title.clone();
                                    let title_for_click = title.clone();
                                    let is_expanded = expanded_issue() == Some(title.clone());

                                    rsx! {
                                        div { class: "glass-card gradient-border",
                                            div { class: "card-body space-y-3",
                                                // Title and severity
                                                div { class: "flex items-start justify-between gap-2",
                                                    h3 { class: "font-semibold text-lg", "{issue.title}" }
                                                    span {
                                                        class: "{severity_badge_class(&issue.severity)}",
                                                        "{issue.severity}"
                                                    }
                                                }

                                                // Description
                                                p { class: "text-sm text-base-content/80", "{issue.description}" }

                                                // Affected demographics
                                                if !issue.affected_demographics.is_empty() {
                                                    div {
                                                        span { class: "text-xs font-medium text-slate-400 mr-2",
                                                            "Affected groups:"
                                                        }
                                                        div { class: "inline-flex flex-wrap gap-1",
                                                            for demo in &issue.affected_demographics {
                                                                span { class: "badge badge-outline badge-sm", "{demo}" }
                                                            }
                                                        }
                                                    }
                                                }

                                                // Suggested talking points
                                                if !issue.suggested_talking_points.is_empty() {
                                                    div {
                                                        h4 { class: "text-sm font-medium mb-1", "Talking Points" }
                                                        ul { class: "list-disc list-inside space-y-1",
                                                            for point in &issue.suggested_talking_points {
                                                                li { class: "text-sm text-base-content/80", "{point}" }
                                                            }
                                                        }
                                                    }
                                                }

                                                // Expand for detailed talking points
                                                button {
                                                    class: "btn btn-outline btn-sm",
                                                    onclick: move |_| on_expand_issue(title_for_click.clone()),
                                                    if is_expanded {
                                                        "Hide Detailed Points"
                                                    } else {
                                                        "Generate Detailed Talking Points"
                                                    }
                                                }

                                                // Detailed talking points (expanded)
                                                if is_expanded {
                                                    if *is_loading_points.read() {
                                                        div { class: "flex items-center gap-2 py-2",
                                                            span { class: "loading loading-spinner loading-sm" }
                                                            span { class: "text-sm text-slate-400",
                                                                "Generating detailed talking points..."
                                                            }
                                                        }
                                                    } else if let Some(ref err) = points_error() {
                                                        div { class: "alert alert-error shadow-sm py-2",
                                                            span { class: "text-sm", "{err}" }
                                                        }
                                                    } else if let Some(ref tp) = talking_points_result() {
                                                        div { class: "bg-slate-800/30 rounded-lg p-4 space-y-2",
                                                            h4 { class: "font-medium text-sm mb-2",
                                                                "Detailed Talking Points"
                                                            }
                                                            ol { class: "list-decimal list-inside space-y-2",
                                                                for point in &tp.points {
                                                                    li { class: "text-sm leading-relaxed",
                                                                        "{point}"
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
                        }
                    } else {
                        // No results yet
                        div { class: "glass-card gradient-border min-h-[300px]",
                            div { class: "card-body flex items-center justify-center",
                                div { class: "text-center",
                                    p { class: "text-lg font-medium text-slate-500 mb-2",
                                        "No analysis yet"
                                    }
                                    p { class: "text-sm text-base-content/40",
                                        "Describe your area and click Analyze Issues to identify local concerns."
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
