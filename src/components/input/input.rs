use super::input_spec::*;
use crate::check::Check;
use crate::data_type::ValidationState;
use core::marker::PhantomData;
use leptos::ev::FocusEvent;
use leptos::*;

/// A reusable input component that provides validation, error handling, and accessibility features.
/// Includes support for custom validation, styling classes, and error/success states.
#[component]
pub fn Input<T: InputSpec>(
    #[prop(optional)] _marker: PhantomData<T>,
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(into, optional)] type_override: Option<String>,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(into, optional)] placeholder: Option<&'static str>,
    #[prop(optional)] validation: Option<Callback<String, Check<T>>>,
    #[prop(default="".into(), into)] wrapper_class: String,
    #[prop(default="".into(), into)] input_class: String,
    #[prop(default="".into(), into)] label_class: String,
    #[prop(default="".into(), into)] summary_class: String,
    #[prop(default="".into(), into)] error_class: String,
    #[prop(default="✅".into_view(), into)] validation_children: View,
) -> impl IntoView {
    let (state, set_state) = create_signal::<ValidationState>(ValidationState::Empty);
    let type_override = type_override.unwrap_or_else(|| T::default_type());

    let validate = move |ev: FocusEvent| {
        if let Some(target) = ev.target() {
            match target.value_of().as_string() {
                Some(v) => match validation(v) {
                    Ok(_) => set_state(ValidationState::Valid),
                    Err(err) => set_state(ValidationState::Invalid(err)),
                },
                None => set_state(ValidationState::Invalid("Failed to get value".into())),
            }
        }
    };

    let focused = move |_| set_state(ValidationState::Dirty);
    let is_invalid = move |state| matches!(state, ValidationState::Invalid(_));
    let error_message_view = move || match state() {
        ValidationState::Invalid(message) => view! { <span class="error-message">{message}</span> },
        ValidationState::Valid => {
            view! { <span class="success-message">{validation_children}</span> }
        }
        _ => view! { <span></span> },
    };

    view! {
        <div data-kind="input" class=wrapper_class>
            <label for=name.clone() class=label_class>
                {label}
            </label>
            <input
                type=type_override
                autocomplete=T::autocomplete()
                name
                id=name.clone()
                placeholder=placeholder
                on:blur=validate
                on:focus=focused
                class=format!("input-field {} {}", state(), input_class)
                aria-invalid=is_invalid(state()).to_string()
                aria-describedby=format!("{name}-error")
            />
            <div
                id=format!("{name}-error error {error_class}")
                data-state=move || state().to_string()
            >
                |_|
                {error_message_view()}
            </div>
            <div class=format!("summary {summary_class}")>{summary}</div>
        </div>
    }
}
