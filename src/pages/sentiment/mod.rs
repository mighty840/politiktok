use std::time::Duration;

use dioxus::prelude::*;

use crate::components::{BarChart, LoadingSpinner, Pagination};
use crate::models::social_post::{SentimentSpike, SentimentSummary, SocialPost};
use crate::modules::sentiment_monitor::{
    get_sentiment_feed, get_sentiment_summary, get_spike_log, get_topics,
};

/// Number of posts displayed per page in the feed.
const POSTS_PER_PAGE: i64 = 20;

/// Available time window options for the dashboard filters.
const TIME_WINDOWS: &[(&str, &str)] = &[
    ("1h", "1 Hour"),
    ("4h", "4 Hours"),
    ("24h", "24 Hours"),
    ("7d", "7 Days"),
];

#[component]
pub fn SentimentDashboardPage() -> Element {
    let mut selected_topic = use_signal(|| None::<String>);
    let mut selected_window = use_signal(|| "24h".to_string());
    let mut selected_sentiment = use_signal(|| None::<String>);
    let mut current_page = use_signal(|| 1_usize);
    let mut refresh_tick = use_signal(|| 0_u64);

    // Auto-refresh every 30 seconds using dioxus-sdk interval.
    dioxus_sdk::time::use_interval(Duration::from_secs(30), move |()| {
        refresh_tick += 1;
    });

    // Fetch topics for the dropdown.
    let topics_resource = use_resource(move || async move {
        let _ = refresh_tick();
        get_topics().await.unwrap_or_default()
    });

    // Fetch sentiment summaries.
    let summary_resource = use_resource(move || {
        let topic = selected_topic();
        let window = selected_window();
        async move {
            let _ = refresh_tick();
            get_sentiment_summary(topic, window)
                .await
                .unwrap_or_default()
        }
    });

    // Fetch the post feed.
    let feed_resource = use_resource(move || {
        let topic = selected_topic();
        let sentiment = selected_sentiment();
        let page = current_page();
        async move {
            let _ = refresh_tick();
            let offset = (page as i64 - 1) * POSTS_PER_PAGE;
            get_sentiment_feed(topic, sentiment, POSTS_PER_PAGE, offset)
                .await
                .unwrap_or_default()
        }
    });

    // Fetch spike log.
    let spikes_resource = use_resource(move || async move {
        let _ = refresh_tick();
        get_spike_log().await.unwrap_or_default()
    });

    rsx! {
        div { class: "p-6 space-y-6 animate-fade-in",
            // Page header
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold", "Sentiment Monitor" }
                    p { class: "text-slate-400",
                        "Real-time public sentiment tracking across social media and community forums."
                    }
                }
                div { class: "flex items-center gap-2",
                    span { class: "loading loading-dots loading-xs text-primary" }
                    span { class: "text-xs text-slate-500", "Auto-refreshing" }
                }
            }

            // Filters
            div { class: "flex flex-wrap gap-3",
                FilterTopicSelector {
                    topics: topics_resource,
                    selected_topic,
                    on_change: move |val: Option<String>| {
                        selected_topic.set(val);
                        current_page.set(1);
                    },
                }
                FilterWindowSelector {
                    selected_window: selected_window(),
                    on_change: move |val: String| {
                        selected_window.set(val);
                        current_page.set(1);
                    },
                }
                FilterSentimentSelector {
                    selected_sentiment: selected_sentiment(),
                    on_change: move |val: Option<String>| {
                        selected_sentiment.set(val);
                        current_page.set(1);
                    },
                }
            }

            // Summary cards
            SummaryCards { summaries: summary_resource }

            // Bar chart
            SentimentBarChart { summaries: summary_resource }

            // Post feed
            PostFeed {
                posts: feed_resource,
                current_page: current_page(),
                on_page_change: move |page: usize| current_page.set(page),
            }

            // Spike log
            SpikeLogSection { spikes: spikes_resource }
        }
    }
}

// ---------------------------------------------------------------------------
// Filter components
// ---------------------------------------------------------------------------

#[component]
fn FilterTopicSelector(
    topics: Resource<Vec<String>>,
    selected_topic: Signal<Option<String>>,
    on_change: EventHandler<Option<String>>,
) -> Element {
    let topics_list = topics.read();

    rsx! {
        select {
            class: "select select-bordered select-sm",
            value: selected_topic.read().as_deref().unwrap_or(""),
            onchange: move |evt: Event<FormData>| {
                let val = evt.value();
                if val.is_empty() {
                    on_change.call(None);
                } else {
                    on_change.call(Some(val));
                }
            },
            option { value: "", "All Topics" }
            if let Some(topics) = topics_list.as_ref() {
                for topic in topics {
                    option { value: "{topic}", "{topic}" }
                }
            }
        }
    }
}

#[component]
fn FilterWindowSelector(selected_window: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "join",
            for (value, label) in TIME_WINDOWS {
                {
                    let active = if selected_window == *value { " btn-active" } else { "" };
                    let val = value.to_string();
                    rsx! {
                        button {
                            class: "join-item btn btn-sm{active}",
                            onclick: move |_| on_change.call(val.clone()),
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilterSentimentSelector(
    selected_sentiment: Option<String>,
    on_change: EventHandler<Option<String>>,
) -> Element {
    let options = [
        ("", "All Sentiments"),
        ("positive", "Positive"),
        ("negative", "Negative"),
        ("neutral", "Neutral"),
    ];

    rsx! {
        select {
            class: "select select-bordered select-sm",
            value: selected_sentiment.as_deref().unwrap_or(""),
            onchange: move |evt: Event<FormData>| {
                let val = evt.value();
                if val.is_empty() {
                    on_change.call(None);
                } else {
                    on_change.call(Some(val));
                }
            },
            for (value, label) in options {
                option { value: "{value}", "{label}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Summary cards
// ---------------------------------------------------------------------------

#[component]
fn SummaryCards(summaries: Resource<Vec<SentimentSummary>>) -> Element {
    let data = summaries.read();

    match data.as_ref() {
        None => rsx! { LoadingSpinner {} },
        Some(items) if items.is_empty() => {
            rsx! {
                div { class: "alert",
                    "No sentiment data available for the selected filters."
                }
            }
        }
        Some(items) => {
            let total_positive: i64 = items.iter().map(|s| s.positive_count).sum();
            let total_negative: i64 = items.iter().map(|s| s.negative_count).sum();
            let total_neutral: i64 = items.iter().map(|s| s.neutral_count).sum();
            let total_posts: i64 = items.iter().map(|s| s.total_count).sum();

            rsx! {
                div { class: "stats stats-vertical lg:stats-horizontal shadow w-full",
                    div { class: "stat",
                        div { class: "stat-title", "Total Posts" }
                        div { class: "stat-value", "{total_posts}" }
                        div { class: "stat-desc", "{items.len()} topics" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Positive" }
                        div { class: "stat-value text-success", "{total_positive}" }
                        div { class: "stat-desc", "{format_pct(total_positive, total_posts)}%" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Negative" }
                        div { class: "stat-value text-error", "{total_negative}" }
                        div { class: "stat-desc", "{format_pct(total_negative, total_posts)}%" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Neutral" }
                        div { class: "stat-value text-info", "{total_neutral}" }
                        div { class: "stat-desc", "{format_pct(total_neutral, total_posts)}%" }
                    }
                }
            }
        }
    }
}

/// Format a count as a percentage of a total.
fn format_pct(count: i64, total: i64) -> String {
    if total == 0 {
        return "0.0".to_string();
    }
    format!("{:.1}", count as f64 / total as f64 * 100.0)
}

// ---------------------------------------------------------------------------
// Bar chart
// ---------------------------------------------------------------------------

#[component]
fn SentimentBarChart(summaries: Resource<Vec<SentimentSummary>>) -> Element {
    let data = summaries.read();

    let Some(items) = data.as_ref() else {
        return rsx! {};
    };

    if items.is_empty() {
        return rsx! {};
    }

    // Build chart data: one bar per topic showing positive vs negative ratio.
    let chart_data: Vec<(String, f64)> = items
        .iter()
        .take(15)
        .map(|s| {
            let label = format!("{} (+{}/-{})", s.topic, s.positive_count, s.negative_count);
            (label, s.total_count as f64)
        })
        .collect();

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                h2 { class: "card-title text-lg", "Sentiment Distribution by Topic" }
                BarChart { data: chart_data, height: "300px".to_string() }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Post feed
// ---------------------------------------------------------------------------

#[component]
fn PostFeed(
    posts: Resource<Vec<SocialPost>>,
    current_page: usize,
    on_page_change: EventHandler<usize>,
) -> Element {
    let data = posts.read();

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                h2 { class: "card-title text-lg", "Recent Posts" }

                match data.as_ref() {
                    None => rsx! { LoadingSpinner {} },
                    Some(items) if items.is_empty() => {
                        rsx! {
                            p { class: "text-slate-400 py-4", "No posts match the current filters." }
                        }
                    }
                    Some(items) => {
                        rsx! {
                            div { class: "overflow-x-auto",
                                table { class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "Source" }
                                            th { "Text" }
                                            th { "Sentiment" }
                                            th { "Score" }
                                            th { "Topics" }
                                            th { "Posted" }
                                        }
                                    }
                                    tbody {
                                        for post in items {
                                            PostRow { post: post.clone() }
                                        }
                                    }
                                }
                            }

                            // Show pagination if we got a full page (more results likely exist).
                            if items.len() as i64 == POSTS_PER_PAGE {
                                Pagination {
                                    current_page,
                                    total_pages: current_page + 1,
                                    on_page_change,
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
fn PostRow(post: SocialPost) -> Element {
    let sentiment_badge = sentiment_badge_class(&post.sentiment);
    let sentiment_label = post.sentiment.as_deref().unwrap_or("unclassified");
    let score_text = match post.sentiment_score {
        Some(s) => format!("{s:.2}"),
        None => "--".to_string(),
    };
    let topics_text = if post.topics.is_empty() {
        "--".to_string()
    } else {
        post.topics.join(", ")
    };
    let posted_text = post.posted_at.as_deref().unwrap_or("--");
    let truncated_text = truncate_text(&post.text, 120);

    rsx! {
        tr {
            td { span { class: "badge badge-outline badge-sm", "{post.source_platform}" } }
            td { class: "max-w-md",
                span { class: "text-sm", "{truncated_text}" }
            }
            td { span { class: "{sentiment_badge}", "{sentiment_label}" } }
            td { class: "tabular-nums text-sm", "{score_text}" }
            td { class: "text-xs", "{topics_text}" }
            td { class: "text-xs whitespace-nowrap", "{posted_text}" }
        }
    }
}

/// Return a DaisyUI badge class string for a sentiment value.
fn sentiment_badge_class(sentiment: &Option<String>) -> &'static str {
    match sentiment.as_deref() {
        Some("positive") => "badge badge-success badge-sm",
        Some("negative") => "badge badge-error badge-sm",
        Some("neutral") => "badge badge-info badge-sm",
        _ => "badge badge-ghost badge-sm",
    }
}

/// Truncate text to a maximum length, appending an ellipsis if needed.
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

// ---------------------------------------------------------------------------
// Spike log
// ---------------------------------------------------------------------------

#[component]
fn SpikeLogSection(spikes: Resource<Vec<SentimentSpike>>) -> Element {
    let data = spikes.read();

    rsx! {
        div { class: "glass-card gradient-border",
            div { class: "p-6",
                h2 { class: "card-title text-lg", "Spike Log" }

                match data.as_ref() {
                    None => rsx! { LoadingSpinner {} },
                    Some(items) if items.is_empty() => {
                        rsx! {
                            p { class: "text-slate-400 py-4", "No sentiment spikes detected." }
                        }
                    }
                    Some(items) => {
                        rsx! {
                            div { class: "overflow-x-auto",
                                table { class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "Topic" }
                                            th { "Sentiment" }
                                            th { "Magnitude" }
                                            th { "Sample Posts" }
                                            th { "Detected At" }
                                        }
                                    }
                                    tbody {
                                        for spike in items {
                                            SpikeRow { spike: spike.clone() }
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
fn SpikeRow(spike: SentimentSpike) -> Element {
    let magnitude_text = format!("{:.2}x", spike.spike_magnitude);
    let sample_count = spike.sample_posts.len();
    let sentiment_badge = match spike.sentiment.as_str() {
        "positive" => "badge badge-success badge-sm",
        "negative" => "badge badge-error badge-sm",
        _ => "badge badge-info badge-sm",
    };

    rsx! {
        tr {
            td { class: "font-medium", "{spike.topic}" }
            td { span { class: "{sentiment_badge}", "{spike.sentiment}" } }
            td { class: "tabular-nums font-mono", "{magnitude_text}" }
            td { "{sample_count} posts" }
            td { class: "text-xs whitespace-nowrap", "{spike.detected_at}" }
        }
    }
}
