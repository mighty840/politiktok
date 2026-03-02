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
        div { class: "flex items-center justify-center gap-1 mt-4",
            // Previous
            button {
                class: "px-3 py-1.5 text-sm rounded-lg bg-slate-800/50 text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-colors disabled:opacity-40 disabled:cursor-not-allowed",
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
                    let (bg, text) = if page == current_page {
                        ("bg-indigo-500", "text-white")
                    } else {
                        ("bg-slate-800/50", "text-slate-400")
                    };
                    let on_page_change = on_page_change.clone();
                    rsx! {
                        button {
                            class: "w-8 h-8 text-sm rounded-lg {bg} {text} hover:bg-indigo-500/80 hover:text-white transition-colors",
                            onclick: move |_| on_page_change.call(page),
                            "{page}"
                        }
                    }
                }
            }

            // Next
            button {
                class: "px-3 py-1.5 text-sm rounded-lg bg-slate-800/50 text-slate-400 hover:text-slate-200 hover:bg-slate-700/50 transition-colors disabled:opacity-40 disabled:cursor-not-allowed",
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
