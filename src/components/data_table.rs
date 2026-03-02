use dioxus::prelude::*;

/// Sortable, filterable data table with glassmorphism styling.
#[component]
pub fn DataTable(
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    #[props(default = None)] on_row_click: Option<EventHandler<usize>>,
) -> Element {
    let mut sort_col = use_signal(|| None::<usize>);
    let mut sort_asc = use_signal(|| true);
    let mut filter_text = use_signal(|| String::new());

    // Filter rows by search text
    let filter = filter_text.read().to_lowercase();
    let mut filtered: Vec<(usize, &Vec<String>)> = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            if filter.is_empty() {
                return true;
            }
            row.iter().any(|cell| cell.to_lowercase().contains(&filter))
        })
        .collect();

    // Sort rows by selected column
    if let Some(col) = *sort_col.read() {
        let asc = *sort_asc.read();
        filtered.sort_by(|(_, a), (_, b)| {
            let cmp = a.get(col).map(String::as_str).unwrap_or("")
                .cmp(b.get(col).map(String::as_str).unwrap_or(""));
            if asc { cmp } else { cmp.reverse() }
        });
    }

    let clickable = on_row_click.is_some();

    rsx! {
        div { class: "glass-card rounded-xl overflow-hidden",
            // Filter input
            div { class: "p-4 border-b border-slate-700/50",
                input {
                    r#type: "text",
                    class: "w-full max-w-xs px-3 py-2 text-sm bg-slate-800/50 border border-slate-700 rounded-xl text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition-colors",
                    placeholder: "Filter...",
                    value: "{filter_text}",
                    oninput: move |evt: Event<FormData>| filter_text.set(evt.value()),
                }
            }

            div { class: "overflow-x-auto",
                table { class: "table table-sm w-full",
                    thead {
                        tr { class: "bg-slate-800/50",
                            for (i, header) in headers.iter().enumerate() {
                                {
                                    let header = header.clone();
                                    let indicator = match *sort_col.read() {
                                        Some(c) if c == i && *sort_asc.read() => " \u{25B2}",
                                        Some(c) if c == i => " \u{25BC}",
                                        _ => "",
                                    };
                                    rsx! {
                                        th {
                                            class: "cursor-pointer select-none text-slate-400 text-xs uppercase tracking-wider font-semibold px-4 py-3 hover:text-slate-200 transition-colors",
                                            onclick: move |_| {
                                                if *sort_col.read() == Some(i) {
                                                    sort_asc.toggle();
                                                } else {
                                                    sort_col.set(Some(i));
                                                    sort_asc.set(true);
                                                }
                                            },
                                            "{header}{indicator}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tbody {
                        for (original_idx, row) in &filtered {
                            {
                                let idx = *original_idx;
                                let row_class = if clickable {
                                    "cursor-pointer hover:bg-indigo-500/5 border-b border-slate-800/50 transition-colors"
                                } else {
                                    "border-b border-slate-800/50"
                                };
                                let handler = on_row_click.clone();
                                rsx! {
                                    tr {
                                        class: "{row_class}",
                                        onclick: move |_| {
                                            if let Some(h) = &handler {
                                                h.call(idx);
                                            }
                                        },
                                        for cell in row.iter() {
                                            td { class: "px-4 py-3 text-sm text-slate-300", "{cell}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if filtered.is_empty() {
                p { class: "text-center text-slate-500 py-8", "No results found." }
            }
        }
    }
}
