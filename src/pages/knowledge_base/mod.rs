use dioxus::prelude::*;

use crate::modules::knowledge_base::{
    ask_knowledge_base, ingest_kb_document, list_kb_documents, KBAnswer, KBDocument,
};

/// Category options for filtering and tagging documents.
const CATEGORIES: &[&str] = &["All", "General", "Policy", "Operations", "Legal", "Training"];

/// Categories for document ingestion (excludes "All").
const INGEST_CATEGORIES: &[&str] = &["General", "Policy", "Operations", "Legal", "Training"];

#[component]
pub fn KnowledgeBasePage() -> Element {
    // Q&A state
    let mut question = use_signal(String::new);
    let mut qa_category = use_signal(|| "All".to_string());
    let mut is_asking = use_signal(|| false);
    let mut qa_history = use_signal(Vec::<KBAnswer>::new);
    let mut qa_error = use_signal(|| Option::<String>::None);

    // Document management state
    let mut doc_title = use_signal(String::new);
    let mut doc_content = use_signal(String::new);
    let mut doc_category = use_signal(|| "General".to_string());
    let mut is_ingesting = use_signal(|| false);
    let mut ingest_error = use_signal(|| Option::<String>::None);
    let mut ingest_success = use_signal(|| Option::<String>::None);
    let mut docs_refresh = use_signal(|| 0u32);
    let mut list_category = use_signal(|| "All".to_string());

    // Fetch documents
    let documents = use_resource(move || {
        let _trigger = docs_refresh();
        let cat = list_category().clone();
        async move { list_kb_documents(cat).await.unwrap_or_default() }
    });

    // Handle asking a question
    let mut on_ask = move |_| {
        let q = question().trim().to_string();
        if q.is_empty() {
            qa_error.set(Some("Please enter a question.".to_string()));
            return;
        }

        let cat = qa_category().clone();
        is_asking.set(true);
        qa_error.set(None);

        spawn(async move {
            match ask_knowledge_base(q, cat).await {
                Ok(answer) => {
                    qa_history.write().insert(0, answer);
                    question.set(String::new());
                }
                Err(e) => qa_error.set(Some(format!("Failed to get answer: {e}"))),
            }
            is_asking.set(false);
        });
    };

    // Handle document ingestion
    let on_ingest = move |_| {
        let t = doc_title().trim().to_string();
        let c = doc_content().trim().to_string();
        let cat = doc_category().clone();

        if t.is_empty() {
            ingest_error.set(Some("Document title is required.".to_string()));
            return;
        }
        if c.is_empty() {
            ingest_error.set(Some("Document content is required.".to_string()));
            return;
        }

        is_ingesting.set(true);
        ingest_error.set(None);
        ingest_success.set(None);

        spawn(async move {
            match ingest_kb_document(t, c, cat).await {
                Ok(doc_id) => {
                    ingest_success.set(Some(format!("Document ingested (ID: {doc_id})")));
                    doc_title.set(String::new());
                    doc_content.set(String::new());
                    docs_refresh += 1;
                }
                Err(e) => ingest_error.set(Some(format!("Ingestion failed: {e}"))),
            }
            is_ingesting.set(false);
        });
    };

    rsx! {
        div { class: "p-6 space-y-6",
            // Page header
            div {
                h1 { class: "text-3xl font-bold", "Knowledge Base" }
                p { class: "text-base-content/70",
                    "AI-powered Q&A over your campaign's internal documents. Ask questions and manage knowledge base documents."
                }
            }

            // Q&A Section
            div { class: "card bg-base-100 shadow-sm",
                div { class: "card-body space-y-4",
                    h2 { class: "card-title text-lg", "Ask a Question" }

                    // Category filter and question input
                    div { class: "flex flex-col sm:flex-row gap-3",
                        div { class: "form-control w-full sm:w-48",
                            select {
                                class: "select select-bordered w-full",
                                value: "{qa_category}",
                                onchange: move |evt: Event<FormData>| qa_category.set(evt.value()),
                                for cat in CATEGORIES {
                                    option { value: "{cat}", "{cat}" }
                                }
                            }
                        }
                        div { class: "form-control flex-1",
                            div { class: "flex gap-2",
                                input {
                                    class: "input input-bordered flex-1",
                                    r#type: "text",
                                    placeholder: "Ask a question about campaign policies, procedures, or operations...",
                                    value: "{question}",
                                    disabled: *is_asking.read(),
                                    oninput: move |evt| question.set(evt.value()),
                                    onkeypress: move |evt: KeyboardEvent| {
                                        if evt.key() == Key::Enter {
                                            on_ask(());
                                        }
                                    },
                                }
                                button {
                                    class: "btn btn-primary",
                                    disabled: *is_asking.read() || question().trim().is_empty(),
                                    onclick: move |_| on_ask(()),
                                    if *is_asking.read() {
                                        span { class: "loading loading-spinner loading-sm" }
                                    } else {
                                        "Ask"
                                    }
                                }
                            }
                        }
                    }

                    // Q&A Error
                    if let Some(err) = qa_error() {
                        div { class: "alert alert-error text-sm",
                            span { "{err}" }
                        }
                    }

                    // Q&A History
                    if !qa_history().is_empty() {
                        div { class: "space-y-4 mt-2",
                            for answer in qa_history().iter() {
                                AnswerCard { answer: answer.clone() }
                            }
                        }
                    } else if !*is_asking.read() {
                        div { class: "text-center py-8",
                            p { class: "text-base-content/50",
                                "Ask a question to search the knowledge base."
                            }
                        }
                    }
                }
            }

            // Document Management Section
            div { class: "divider", "Document Management" }

            div { class: "flex flex-col lg:flex-row gap-6",

                // Left: Ingest Form
                div { class: "w-full lg:w-1/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body space-y-4",
                            h2 { class: "card-title text-lg", "Add Document" }

                            // Title
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Title" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    placeholder: "Document title",
                                    value: "{doc_title}",
                                    oninput: move |evt| doc_title.set(evt.value()),
                                }
                            }

                            // Category
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Category" }
                                }
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{doc_category}",
                                    onchange: move |evt: Event<FormData>| doc_category.set(evt.value()),
                                    for cat in INGEST_CATEGORIES {
                                        option { value: "{cat}", "{cat}" }
                                    }
                                }
                            }

                            // Content
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Content" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Paste document content here...",
                                    rows: "8",
                                    value: "{doc_content}",
                                    oninput: move |evt| doc_content.set(evt.value()),
                                }
                            }

                            // Errors / Success
                            if let Some(err) = ingest_error() {
                                div { class: "alert alert-error text-sm",
                                    span { "{err}" }
                                }
                            }
                            if let Some(msg) = ingest_success() {
                                div { class: "alert alert-success text-sm",
                                    span { "{msg}" }
                                }
                            }

                            // Ingest button
                            button {
                                class: "btn btn-primary w-full",
                                disabled: *is_ingesting.read(),
                                onclick: on_ingest,
                                if *is_ingesting.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                    "Ingesting..."
                                } else {
                                    "Ingest Document"
                                }
                            }
                        }
                    }
                }

                // Right: Document List
                div { class: "w-full lg:w-2/3",
                    div { class: "card bg-base-100 shadow-sm",
                        div { class: "card-body",
                            div { class: "flex items-center justify-between mb-4",
                                h2 { class: "card-title text-lg", "Documents" }
                                select {
                                    class: "select select-bordered select-sm",
                                    value: "{list_category}",
                                    onchange: move |evt: Event<FormData>| {
                                        list_category.set(evt.value());
                                        docs_refresh += 1;
                                    },
                                    for cat in CATEGORIES {
                                        option { value: "{cat}", "{cat}" }
                                    }
                                }
                            }

                            {
                                let docs = documents().unwrap_or_default();
                                if docs.is_empty() {
                                    rsx! {
                                        div { class: "text-center py-8",
                                            p { class: "text-base-content/50",
                                                "No documents in the knowledge base yet. Add a document to get started."
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "overflow-x-auto",
                                            table { class: "table table-sm",
                                                thead {
                                                    tr {
                                                        th { "Title" }
                                                        th { "Category" }
                                                        th { "Created" }
                                                    }
                                                }
                                                tbody {
                                                    for doc in docs.iter() {
                                                        DocumentRow { document: doc.clone() }
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
    }
}

/// Display a single Q&A answer card.
#[component]
fn AnswerCard(answer: KBAnswer) -> Element {
    let confidence_class = if answer.confidence >= 0.8 {
        "badge badge-success badge-sm"
    } else if answer.confidence >= 0.5 {
        "badge badge-warning badge-sm"
    } else {
        "badge badge-error badge-sm"
    };

    let confidence_pct = (answer.confidence * 100.0) as u32;

    rsx! {
        div { class: "border border-base-300 rounded-lg p-4 space-y-3",
            // Question
            div { class: "flex items-start gap-2",
                span { class: "badge badge-primary badge-sm mt-0.5", "Q" }
                p { class: "font-medium text-sm", "{answer.question}" }
            }

            // Answer
            div { class: "flex items-start gap-2",
                span { class: "badge badge-secondary badge-sm mt-0.5", "A" }
                p { class: "text-sm whitespace-pre-wrap leading-relaxed", "{answer.answer}" }
            }

            // Footer: sources and confidence
            div { class: "flex items-center justify-between text-xs",
                div { class: "flex flex-wrap gap-1",
                    if !answer.sources.is_empty() {
                        span { class: "text-base-content/50 mr-1", "Sources:" }
                        for source in answer.sources.iter() {
                            span { class: "badge badge-ghost badge-xs", "{source}" }
                        }
                    }
                }
                span { class: "{confidence_class}", "Confidence: {confidence_pct}%" }
            }
        }
    }
}

/// Display a single document row in the table.
#[component]
fn DocumentRow(document: KBDocument) -> Element {
    rsx! {
        tr {
            td { class: "font-medium text-sm", "{document.title}" }
            td {
                span { class: "badge badge-ghost badge-sm", "{document.category}" }
            }
            td { class: "text-sm text-base-content/60", "{document.created_at}" }
        }
    }
}
