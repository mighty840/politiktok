use dioxus::prelude::*;

/// Renders streaming LLM output as HTML converted from markdown.
///
/// While content is actively streaming (indicated by `streaming` prop),
/// a blinking cursor animation is appended.
#[component]
pub fn StreamingText(
    content: String,
    #[props(default = false)] streaming: bool,
) -> Element {
    let html = use_memo(move || {
        let parser = pulldown_cmark::Parser::new(&content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    });

    rsx! {
        div { class: "streaming-text prose prose-sm max-w-none",
            div { dangerous_inner_html: "{html}" }
            if streaming {
                span {
                    class: "inline-block w-2 h-4 bg-primary ml-0.5 animate-pulse rounded-sm",
                    style: "vertical-align: text-bottom;",
                }
            }
        }
    }
}
