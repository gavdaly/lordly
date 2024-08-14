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
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default=(|_|{true}).into(), into)] validation: Callback<String, bool>,
    #[prop(default="".into(), into)] wrapper_class: String,
    #[prop(default="".into(), into)] input_class: String,
    #[prop(default="".into(), into)] label_class: String,
    #[prop(default="".into(), into)] summary_class: String,
    #[prop(default="".into(), into)] error_class: String,
    #[prop(default="✅".into_view(), into)] validation_children: View,
) -> impl IntoView {
    let (validation_state, set_validation_state) =
        create_signal::<ValidationState>(ValidationState::Dirty);
    let validate = move |ev: FocusEvent| {
        let target = ev.target().unwrap();
        let value = target.value_of().as_string().unwrap();
        let value = value.clone();
        match validation(value) {
            true => set_validation_state(ValidationState::Valid),
            false => set_validation_state(ValidationState::Invalid("reason".to_string())),
        };
    };
    let focused = move |_| set_validation_state(ValidationState::Dirty);

    view! {
        <div class=format!("input {wrapper_class}")>
            <div class=format!("summary {summary_class}")>{summary}</div>
            <label class=format!("input_label {label_class}") for=name.clone()>
                {label}
            </label>
            <input
                class=input_class
                type="text"
                name=name.clone()
                id=name
                placeholder=placeholder
                on:blur=validate
                on:focus=focused
            />
            {move || match validation_state() {
                ValidationState::Invalid(r) => {
                    view! {
                        <div class=format!("{error_class}") data-type="error">
                            {r}
                        </div>
                    }
                        .into_view()
                }
                ValidationState::Valid => {
                    view! {
                        <div class=format!("") data-type="success">
                            {validation_children.clone()}
                        </div>
                    }
                        .into_view()
                }
                ValidationState::Dirty => view! { <></> }.into_view(),
            }}

        </div>
    }
}
