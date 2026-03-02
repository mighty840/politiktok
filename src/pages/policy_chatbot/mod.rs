use dioxus::prelude::*;

use crate::components::StreamingText;
use crate::models::document::{ChatMessage, ChatSession, Document};
use crate::modules::policy_chatbot::{
    chat, create_chat_session, delete_document, get_chat_messages, ingest_document,
    list_chat_sessions, list_documents,
};

/// Active view in the policy chatbot page.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveTab {
    Chat,
    Documents,
}

#[component]
pub fn PolicyChatPage() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Chat);
    let mut active_session_id = use_signal(|| Option::<String>::None);
    let mut messages = use_signal(Vec::<ChatMessage>::new);
    let mut input_text = use_signal(String::new);
    let mut is_sending = use_signal(|| false);
    let mut sessions_refresh = use_signal(|| 0u32);
    let mut docs_refresh = use_signal(|| 0u32);

    // Fetch chat sessions
    let sessions = use_resource(move || {
        let _trigger = sessions_refresh();
        async move { list_chat_sessions().await.unwrap_or_default() }
    });

    // Fetch documents
    let documents = use_resource(move || {
        let _trigger = docs_refresh();
        async move { list_documents(String::new()).await.unwrap_or_default() }
    });

    // Load messages when session changes
    let _msgs_loader = use_resource(move || {
        let session_id = active_session_id().clone();
        async move {
            if let Some(sid) = session_id {
                if let Ok(msgs) = get_chat_messages(sid).await {
                    messages.set(msgs);
                }
            } else {
                messages.set(Vec::new());
            }
        }
    });

    // Handle creating a new chat session
    let on_new_session = move |_| {
        spawn(async move {
            match create_chat_session().await {
                Ok(session) => {
                    active_session_id.set(Some(session.id));
                    messages.set(Vec::new());
                    sessions_refresh += 1;
                }
                Err(e) => tracing::error!("Failed to create session: {e}"),
            }
        });
    };

    // Handle sending a message
    let mut on_send = move |_: ()| {
        let question = input_text().trim().to_string();
        if question.is_empty() || *is_sending.read() {
            return;
        }

        let session_id = match active_session_id() {
            Some(sid) => sid,
            None => return,
        };

        input_text.set(String::new());
        is_sending.set(true);

        // Add user message immediately
        let user_msg = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.clone(),
            role: "user".to_string(),
            content: question.clone(),
            sources: None,
            created_at: None,
        };
        messages.write().push(user_msg);

        // Add placeholder assistant message
        let placeholder_id = uuid::Uuid::new_v4().to_string();
        let placeholder = ChatMessage {
            id: placeholder_id.clone(),
            session_id: session_id.clone(),
            role: "assistant".to_string(),
            content: String::new(),
            sources: None,
            created_at: None,
        };
        messages.write().push(placeholder);

        spawn(async move {
            match chat(question, session_id, String::new()).await {
                Ok(response) => {
                    // Replace placeholder with actual response
                    let mut msgs = messages.write();
                    if let Some(msg) = msgs.iter_mut().find(|m| m.id == placeholder_id) {
                        msg.content = response.content;
                        msg.sources = response.sources;
                    }
                }
                Err(e) => {
                    let mut msgs = messages.write();
                    if let Some(msg) = msgs.iter_mut().find(|m| m.id == placeholder_id) {
                        msg.content = format!("Error: {e}");
                    }
                }
            }
            is_sending.set(false);
        });
    };

    rsx! {
        div { class: "flex h-full min-h-[calc(100vh-4rem)]",

            // Sidebar: session list
            div { class: "w-64 border-r border-base-300 bg-slate-800/30 flex flex-col",
                div { class: "p-3 border-b border-base-300",
                    button {
                        class: "btn btn-primary btn-sm w-full",
                        onclick: on_new_session,
                        "New Chat"
                    }
                }

                // Tab switcher
                div { class: "flex border-b border-base-300",
                    button {
                        class: if *active_tab.read() == ActiveTab::Chat {
                            "flex-1 py-2 text-sm font-medium border-b-2 border-primary text-primary"
                        } else {
                            "flex-1 py-2 text-sm font-medium text-slate-400 hover:text-base-content"
                        },
                        onclick: move |_| active_tab.set(ActiveTab::Chat),
                        "Chats"
                    }
                    button {
                        class: if *active_tab.read() == ActiveTab::Documents {
                            "flex-1 py-2 text-sm font-medium border-b-2 border-primary text-primary"
                        } else {
                            "flex-1 py-2 text-sm font-medium text-slate-400 hover:text-base-content"
                        },
                        onclick: move |_| active_tab.set(ActiveTab::Documents),
                        "Docs"
                    }
                }

                div { class: "flex-1 overflow-y-auto",
                    match *active_tab.read() {
                        ActiveTab::Chat => rsx! {
                            SessionList {
                                sessions: sessions().unwrap_or_default(),
                                active_id: active_session_id(),
                                on_select: move |id: String| {
                                    active_session_id.set(Some(id));
                                },
                            }
                        },
                        ActiveTab::Documents => rsx! {
                            DocumentPanel {
                                documents: documents().unwrap_or_default(),
                                on_refresh: move || docs_refresh += 1,
                            }
                        },
                    }
                }
            }

            // Main chat area
            div { class: "flex-1 flex flex-col",
                if active_session_id().is_some() {
                    // Messages
                    div { class: "flex-1 overflow-y-auto p-4 space-y-4",
                        if messages().is_empty() {
                            div { class: "flex items-center justify-center h-full",
                                div { class: "text-center text-slate-500",
                                    p { class: "text-lg font-medium mb-2", "Ask about any policy" }
                                    p { class: "text-sm",
                                        "Questions will be answered using ingested policy documents."
                                    }
                                }
                            }
                        }
                        for msg in messages().iter() {
                            MessageBubble {
                                key: "{msg.id}",
                                message: msg.clone(),
                                is_streaming: *is_sending.read() && msg.content.is_empty() && msg.role == "assistant",
                            }
                        }
                    }

                    // Input area
                    div { class: "border-t border-base-300 p-4",
                        form {
                            class: "flex gap-2",
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                on_send(());
                            },
                            input {
                                class: "input input-bordered flex-1",
                                r#type: "text",
                                placeholder: "Ask a question about policy...",
                                value: "{input_text}",
                                disabled: *is_sending.read(),
                                oninput: move |evt| input_text.set(evt.value()),
                            }
                            button {
                                class: "btn btn-primary",
                                r#type: "submit",
                                disabled: *is_sending.read() || input_text().trim().is_empty(),
                                if *is_sending.read() {
                                    span { class: "loading loading-spinner loading-sm" }
                                } else {
                                    "Send"
                                }
                            }
                        }
                    }
                } else {
                    // No session selected
                    div { class: "flex-1 flex items-center justify-center",
                        div { class: "text-center",
                            h2 { class: "text-2xl font-bold mb-2", "Policy Chatbot" }
                            p { class: "text-slate-400 mb-4",
                                "AI-powered policy research assistant. Ask questions about legislation, regulations, and policy positions."
                            }
                            button {
                                class: "btn btn-primary",
                                onclick: on_new_session,
                                "Start New Chat"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A single chat message bubble.
#[component]
fn MessageBubble(message: ChatMessage, is_streaming: bool) -> Element {
    let is_user = message.role == "user";
    let chat_class = if is_user {
        "chat chat-end"
    } else {
        "chat chat-start"
    };
    let bubble_class = if is_user {
        "chat-bubble chat-bubble-primary"
    } else {
        "chat-bubble chat-bubble-secondary"
    };

    let sources: Vec<serde_json::Value> = message
        .sources
        .as_ref()
        .and_then(|s| s.as_array().cloned())
        .unwrap_or_default();

    rsx! {
        div { class: "{chat_class}",
            div { class: "chat-header text-xs text-slate-500 mb-1",
                if is_user { "You" } else { "Policy Assistant" }
            }
            div { class: "{bubble_class}",
                if is_user {
                    p { "{message.content}" }
                } else if is_streaming {
                    StreamingText {
                        content: message.content.clone(),
                        streaming: true,
                    }
                } else {
                    StreamingText {
                        content: message.content.clone(),
                        streaming: false,
                    }
                }
            }

            // Source citations
            if !is_user && !sources.is_empty() && !is_streaming {
                div { class: "chat-footer mt-1",
                    div { class: "flex flex-wrap gap-1",
                        for source in sources.iter() {
                            {
                                let title = source["title"].as_str().unwrap_or("Unknown");
                                let score = source["score"].as_f64().unwrap_or(0.0);
                                rsx! {
                                    span {
                                        class: "badge badge-ghost badge-xs",
                                        title: "Relevance: {score:.2}",
                                        "{title}"
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

/// Sidebar list of chat sessions.
#[component]
fn SessionList(
    sessions: Vec<ChatSession>,
    active_id: Option<String>,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "p-2 space-y-1",
            if sessions.is_empty() {
                p { class: "text-sm text-slate-500 text-center py-4",
                    "No chat sessions yet"
                }
            }
            for session in sessions.iter() {
                {
                    let is_active = active_id.as_ref() == Some(&session.id);
                    let btn_class = if is_active {
                        "w-full text-left p-2 rounded-lg bg-primary/10 text-primary text-sm truncate"
                    } else {
                        "w-full text-left p-2 rounded-lg hover:bg-slate-700/30 text-sm truncate"
                    };
                    let session_id = session.id.clone();
                    let display_time = session
                        .last_active
                        .as_deref()
                        .or(session.created_at.as_deref())
                        .unwrap_or("New session");

                    rsx! {
                        button {
                            class: "{btn_class}",
                            onclick: move |_| on_select.call(session_id.clone()),
                            div { class: "font-medium truncate", "Chat {display_time}" }
                        }
                    }
                }
            }
        }
    }
}

/// Document management panel in the sidebar.
#[component]
fn DocumentPanel(documents: Vec<Document>, on_refresh: EventHandler) -> Element {
    let mut show_upload = use_signal(|| false);

    rsx! {
        div { class: "p-2 space-y-2",
            button {
                class: "btn btn-outline btn-xs w-full",
                onclick: move |_| show_upload.toggle(),
                if *show_upload.read() { "Cancel" } else { "Upload Document" }
            }

            if *show_upload.read() {
                DocumentUploadForm {
                    on_success: move || {
                        show_upload.set(false);
                        on_refresh.call(());
                    },
                }
            }

            if documents.is_empty() {
                p { class: "text-sm text-slate-500 text-center py-4",
                    "No documents ingested yet"
                }
            }

            for doc in documents.iter() {
                {
                    let doc_id = doc.id.clone();
                    let on_refresh = on_refresh;
                    rsx! {
                        div { class: "card card-compact bg-base-100 shadow-sm",
                            div { class: "card-body p-2",
                                div { class: "flex items-start justify-between gap-1",
                                    div {
                                        p { class: "text-sm font-medium truncate", "{doc.title}" }
                                        p { class: "text-xs text-slate-500",
                                            "{doc.chunk_count} chunks"
                                        }
                                    }
                                    button {
                                        class: "btn btn-ghost btn-xs text-error",
                                        title: "Delete document",
                                        onclick: move |_| {
                                            let did = doc_id.clone();
                                            let on_refresh = on_refresh;
                                            spawn(async move {
                                                if let Err(e) = delete_document(did).await {
                                                    tracing::error!("Failed to delete: {e}");
                                                } else {
                                                    on_refresh.call(());
                                                }
                                            });
                                        },
                                        "X"
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

/// Form for uploading/ingesting a new document.
#[component]
fn DocumentUploadForm(on_success: EventHandler) -> Element {
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut is_uploading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_submit = move |_| {
        let doc_title = title().trim().to_string();
        let doc_content = content().trim().to_string();

        if doc_title.is_empty() || doc_content.is_empty() {
            error_msg.set(Some("Title and content are required.".to_string()));
            return;
        }

        is_uploading.set(true);
        error_msg.set(None);

        spawn(async move {
            match ingest_document(doc_title, doc_content, String::new()).await {
                Ok(_doc_id) => {
                    title.set(String::new());
                    content.set(String::new());
                    on_success.call(());
                }
                Err(e) => {
                    error_msg.set(Some(format!("Upload failed: {e}")));
                }
            }
            is_uploading.set(false);
        });
    };

    rsx! {
        div { class: "space-y-2 p-2 bg-base-100 rounded-lg border border-base-300",
            input {
                class: "input input-bordered input-xs w-full",
                r#type: "text",
                placeholder: "Document title",
                value: "{title}",
                oninput: move |evt| title.set(evt.value()),
            }
            textarea {
                class: "textarea textarea-bordered textarea-xs w-full",
                placeholder: "Paste document content...",
                rows: "4",
                value: "{content}",
                oninput: move |evt| content.set(evt.value()),
            }
            if let Some(err) = error_msg() {
                p { class: "text-xs text-error", "{err}" }
            }
            button {
                class: "btn btn-primary btn-xs w-full",
                disabled: *is_uploading.read(),
                onclick: on_submit,
                if *is_uploading.read() {
                    span { class: "loading loading-spinner loading-xs" }
                    "Ingesting..."
                } else {
                    "Ingest Document"
                }
            }
        }
    }
}
