use alloc::format;
use alloc::string::{String, ToString};
use crate::data_type::ValidationState;
use leptos::ev::FocusEvent;
use leptos::prelude::*;

/// A reusable input component that provides validation, error handling, and accessibility features.
/// Includes support for custom validation, styling classes, and error/success states.
#[component]
pub fn Input(
    #[prop(into)] name: String,
    #[prop(into)] label: &'static str,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(default=Callback::new(|_|{ValidationState::Valid}), into)] validation: Callback<
        String,
        ValidationState,
    >,
    #[prop(default="".into(), into)] wrapper_class: String,
    #[prop(default="".into(), into)] input_class: String,
    #[prop(default="".into(), into)] label_class: String,
    #[prop(default="".into(), into)] summary_class: String,
    #[prop(default="".into(), into)] error_class: String,
    #[prop(default="✅".into_any(), into)] validation_children: AnyView,
) -> impl IntoView {
    let (state, set_state) = signal(ValidationState::Empty);

    let validate = move |ev: FocusEvent| {
        if let Some(target) = ev.target() {
            match target.value_of().as_string() {
                Some(v) => set_state.set(validation.run(v)),
                None => set_state.set(ValidationState::Invalid(String::from("Failed to get value"))),
            }
        }
    };

    let focused = move |_| set_state.set(ValidationState::Dirty);
    let is_invalid = move |state| matches!(state, ValidationState::Invalid(_));
    let error_message_view = move || match state.get() {
        ValidationState::Invalid(message) => {
            view! { <span class="error-message">{message}</span> }.into_any()
        }
        ValidationState::Valid => {
            view! { <span class="success-message">{validation_children}</span> }.into_any()
        }
        _ => view! { <span></span> }.into_any(),
    };

    view! {
        <div data-kind="input" class=wrapper_class>
            <label for=name.clone() class=label_class>
                {label}
            </label>
            <input
                type="text"
                name
                id=name.clone()
                placeholder=placeholder
                on:blur=validate
                on:focus=focused
                class=format!("input-field {} {}", state.get(), input_class)
                aria-invalid=is_invalid(state.get()).to_string()
                aria-describedby=format!("{name}-error")
            />
            <div
                id=format!("{name}-error error {error_class}")
                data-state=move || state.get().to_string()
            >
                |_|
                {error_message_view()}
            </div>
            <div class=format!("summary {summary_class}")>{summary}</div>
        </div>
    }
}
