use alloc::string::String;

use crate::data_type::ValidationState;
use leptos::ev::FocusEvent;
use leptos::prelude::*;

#[component]
pub fn TextArea(
    #[prop(optional, into)] label: String,
    #[prop(into)] name: String,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] _summary: Option<String>,
    #[prop(optional, into)] wrapper_class: Option<String>,
    #[prop(optional, into)] input_class: Option<String>,
    #[prop(optional, into)] _label_class: Option<String>,
    #[prop(optional, into)] _summary_class: Option<String>,
    #[prop(optional, into)] _error_class: Option<String>,
    #[prop(optional, into)] _validation_children: Option<AnyView>,
    #[prop(optional, into)] validation: Option<Callback<String, Result<(), String>>>,
) -> impl IntoView {
    let (_state, set_state) = signal(ValidationState::Empty);
    let validate = move |ev: FocusEvent| {
        let Some(valid) = validation else {
            return;
        };

        let Some(target) = ev.target() else {
            return;
        };
        let Some(value) = target.value_of().as_string() else {
            set_state.set(ValidationState::Invalid("Failed to validate".into()));
            return;
        };
        match valid.run(value) {
            Ok(_) => set_state.set(ValidationState::Valid),
            Err(err) => set_state.set(ValidationState::Invalid(err)),
        }
    };
    view! {
        <div data-kind="textarea" class=wrapper_class>
            <label for=name.clone()>{label}</label>
            <textarea
                class=input_class
                id=name.clone()
                name=name
                on:blur=validate
                on:focus=move |_| set_state.set(ValidationState::Dirty)
                placeholder=placeholder
            ></textarea>
        </div>
    }
}
