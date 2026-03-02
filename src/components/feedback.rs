use dioxus::prelude::*;

/// Feedback state: liked, disliked, or neutral.
#[derive(Debug, Clone, Copy, PartialEq)]
enum FeedbackState {
    Neutral,
    Liked,
    Disliked,
}

/// Thumbs up / thumbs down feedback widget.
///
/// Records user feedback for a specific module and resource via a server
/// function.
#[component]
pub fn FeedbackWidget(module_id: String, resource_id: String) -> Element {
    let mut state = use_signal(|| FeedbackState::Neutral);
    let module_id = use_signal(|| module_id);
    let resource_id = use_signal(|| resource_id);

    let thumb_up_class = match *state.read() {
        FeedbackState::Liked => "btn btn-sm btn-success",
        _ => "btn btn-sm btn-ghost",
    };

    let thumb_down_class = match *state.read() {
        FeedbackState::Disliked => "btn btn-sm btn-error",
        _ => "btn btn-sm btn-ghost",
    };

    rsx! {
        div { class: "flex items-center gap-1",
            button {
                class: "{thumb_up_class}",
                title: "Helpful",
                onclick: move |_| {
                    let new_state = if *state.read() == FeedbackState::Liked {
                        FeedbackState::Neutral
                    } else {
                        FeedbackState::Liked
                    };
                    state.set(new_state);
                    let mid = module_id.read().clone();
                    let rid = resource_id.read().clone();
                    let liked = new_state == FeedbackState::Liked;
                    spawn(async move {
                        let _ = record_feedback(mid, rid, liked, true).await;
                    });
                },
                // Thumbs up unicode
                "\u{1F44D}"
            }
            button {
                class: "{thumb_down_class}",
                title: "Not helpful",
                onclick: move |_| {
                    let new_state = if *state.read() == FeedbackState::Disliked {
                        FeedbackState::Neutral
                    } else {
                        FeedbackState::Disliked
                    };
                    state.set(new_state);
                    let mid = module_id.read().clone();
                    let rid = resource_id.read().clone();
                    let disliked = new_state == FeedbackState::Disliked;
                    spawn(async move {
                        let _ = record_feedback(mid, rid, false, disliked).await;
                    });
                },
                // Thumbs down unicode
                "\u{1F44E}"
            }
        }
    }
}

/// Server function to persist feedback.
#[server(endpoint = "record-feedback")]
async fn record_feedback(
    module_id: String,
    resource_id: String,
    is_positive: bool,
    is_active: bool,
) -> Result<(), ServerFnError> {
    tracing::info!(
        module_id = %module_id,
        resource_id = %resource_id,
        is_positive = %is_positive,
        is_active = %is_active,
        "feedback recorded"
    );
    // TODO: persist to database when ready
    Ok(())
}
