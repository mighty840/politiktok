use dioxus::prelude::*;

/// Sortable, filterable data table using DaisyUI table classes.
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
        div { class: "overflow-x-auto",
            // Filter input
            div { class: "mb-3",
                input {
                    r#type: "text",
                    class: "input input-bordered input-sm w-full max-w-xs",
                    placeholder: "Filter...",
                    value: "{filter_text}",
                    oninput: move |evt: Event<FormData>| filter_text.set(evt.value()),
                }
            }

            table { class: "table table-sm table-zebra w-full",
                thead {
                    tr {
                        for (i, header) in headers.iter().enumerate() {
                            {
                                let header = header.clone();
                                let indicator = match *sort_col.read() {
                                    Some(c) if c == i && *sort_asc.read() => " ▲",
                                    Some(c) if c == i => " ▼",
                                    _ => "",
                                };
                                rsx! {
                                    th {
                                        class: "cursor-pointer select-none hover:bg-base-200",
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
                            let row_class = if clickable { "cursor-pointer hover:bg-base-200" } else { "" };
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
                                        td { "{cell}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if filtered.is_empty() {
                p { class: "text-center text-base-content/50 py-4", "No results found." }
            }
        }
    }
}
