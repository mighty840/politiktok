use dioxus::prelude::*;

/// CSS-only horizontal bar chart with gradient fill bars.
///
/// Each entry in `data` is a `(label, value)` tuple. Bars are scaled relative
/// to the maximum value in the dataset.
#[component]
pub fn BarChart(
    data: Vec<(String, f64)>,
    #[props(default = "200px".to_string())] height: String,
) -> Element {
    let max_value = data
        .iter()
        .map(|(_, v)| *v)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    rsx! {
        div {
            class: "flex flex-col gap-2 w-full",
            style: "min-height: {height};",

            for (label, value) in &data {
                {
                    let pct = (*value / max_value) * 100.0;
                    let pct_str = format!("{pct:.1}");
                    let val_str = format!("{value:.1}");
                    rsx! {
                        div { class: "flex items-center gap-2",
                            span { class: "text-xs w-24 text-right truncate text-slate-400", "{label}" }
                            div { class: "flex-1 bg-slate-800/50 rounded-full h-5 overflow-hidden",
                                div {
                                    class: "h-full rounded-full transition-all duration-300",
                                    style: "width: {pct_str}%; background: linear-gradient(90deg, #6366f1, #8b5cf6);",
                                }
                            }
                            span { class: "text-xs w-12 tabular-nums text-slate-400", "{val_str}" }
                        }
                    }
                }
            }
        }
    }
}
