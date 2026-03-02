use dioxus::prelude::*;

/// A single toast notification entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub variant: String,
}

/// Shared toast state accessible via context.
#[derive(Debug, Clone, Copy)]
pub struct ToastState {
    pub toasts: Signal<Vec<Toast>>,
    next_id: Signal<u64>,
}

impl ToastState {
    /// Display a new toast notification. Click to dismiss.
    pub fn show(&mut self, message: impl Into<String>, variant: impl Into<String>) {
        let id = *self.next_id.read();
        self.next_id += 1;
        self.toasts.write().push(Toast {
            id,
            message: message.into(),
            variant: variant.into(),
        });
    }

    /// Dismiss a toast by id.
    pub fn dismiss(&mut self, id: u64) {
        self.toasts.write().retain(|t| t.id != id);
    }
}

/// Wraps children and provides `ToastState` via context.
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toasts: Signal<Vec<Toast>> = use_signal(Vec::new);
    let next_id = use_signal(|| 0_u64);

    let state = ToastState { toasts, next_id };
    use_context_provider(|| state);

    rsx! {
        {children}

        div { class: "toast toast-end toast-bottom z-50",
            for toast in state.toasts.read().iter() {
                {
                    let alert_class = match toast.variant.as_str() {
                        "error" => "alert alert-error",
                        "warning" => "alert alert-warning",
                        "success" => "alert alert-success",
                        _ => "alert alert-info",
                    };
                    let id = toast.id;
                    let mut state = state;
                    rsx! {
                        div {
                            class: "{alert_class} shadow-lg cursor-pointer",
                            onclick: move |_| state.dismiss(id),
                            span { "{toast.message}" }
                        }
                    }
                }
            }
        }
    }
}
