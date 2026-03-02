use dioxus::prelude::*;

/// Pagination controls with previous/next and page number buttons.
#[component]
pub fn Pagination(
    current_page: usize,
    total_pages: usize,
    on_page_change: EventHandler<usize>,
) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }

    rsx! {
        div { class: "join mt-4 flex justify-center",
            // Previous
            button {
                class: "join-item btn btn-sm",
                disabled: current_page == 1,
                onclick: {
                    let on_page_change = on_page_change.clone();
                    move |_| {
                        if current_page > 1 {
                            on_page_change.call(current_page - 1);
                        }
                    }
                },
                "Prev"
            }

            // Page numbers
            for page in 1..=total_pages {
                {
                    let active = if page == current_page { " btn-active" } else { "" };
                    let on_page_change = on_page_change.clone();
                    rsx! {
                        button {
                            class: "join-item btn btn-sm{active}",
                            onclick: move |_| on_page_change.call(page),
                            "{page}"
                        }
                    }
                }
            }

            // Next
            button {
                class: "join-item btn btn-sm",
                disabled: current_page == total_pages,
                onclick: move |_| {
                    if current_page < total_pages {
                        on_page_change.call(current_page + 1);
                    }
                },
                "Next"
            }
        }
    }
}
