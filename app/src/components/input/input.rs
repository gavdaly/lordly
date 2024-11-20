use crate::data_type::ValidationState;
use ev::FocusEvent;
use leptos::*;

/// An input component.
///
/// # Arguments
/// - `name`: The name of the input.
/// - `label`: The label of the input.
/// - `summary`: A summary of the input.
/// - `placeholder`: The placeholder of the input.
/// - `validation`: A callback that validates the input when it is blured.
/// - `wrapper_class`: The class for the wrapper.
/// - `input_class`: The class for the input.
/// - `label_class`: The class for the label.
/// - `summary_class`: The class for the summary.
/// - `error_class`: The class for the error.
/// - `validation_children`: The children for the validation.
#[component]
pub fn Input(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(default=(|_|{Ok(())}).into(), into)] validation: Callback<String, Result<(), String>>,
    #[prop(default="".into(), into)] wrapper_class: String,
    #[prop(default="".into(), into)] input_class: String,
    #[prop(default="".into(), into)] label_class: String,
    #[prop(default="".into(), into)] summary_class: String,
    #[prop(default="".into(), into)] error_class: String,
    #[prop(default="✅".into_view(), into)] validation_children: View,
) -> impl IntoView {
    let (state, set_state) = create_signal::<ValidationState>(ValidationState::Empty);

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
    let is_invalid = move |state| match state {
        ValidationState::Invalid(_) => true,
        _ => false,
    };
    let error_message_view = move || match state() {
        ValidationState::Invalid(message) => view! { <span class="error-message">{message}</span> },
        ValidationState::Valid => {
            view! {<span class="success-message">{validation_children}</span>}
        }
        _ => view! { <span></span> },
    };

    view! {
        <div class={format!("input {wrapper_class}")}>
            <label for=name.clone() class={label_class}>{label}</label>
            <input
                type="text"
                name
                id=name.clone()
                placeholder=placeholder
                on:blur=validate
                on:focus=focused
                class={format!("input-field {} {}", state().to_string(), input_class)}
                aria-invalid={is_invalid(state()).to_string()}
                aria-describedby={format!("{name}-error")}
            />
            <div id={format!("{}-error error {}", name, error_class)} data-state={move || state().to_string()}>
                |_| { error_message_view()}
            </div>
            <div class={format!("summary {}", summary_class)}>{summary}</div>
        </div>
    }
}
