use crate::data_type::ValidationState;
use ev::FocusEvent;
use leptos::*;

#[component]
pub fn TextArea(
    #[prop(optional, into)] label: String,
    #[prop(into)] name: String,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] summary: Option<String>,
    #[prop(optional, into)] wrapper_class: Option<String>,
    #[prop(optional, into)] input_class: Option<String>,
    #[prop(optional, into)] label_class: Option<String>,
    #[prop(optional, into)] summary_class: Option<String>,
    #[prop(optional, into)] error_class: Option<String>,
    #[prop(optional, into)] validation_children: Option<View>,
    #[prop(optional, into)] validation: Option<Callback<String, Result<(), String>>>,
) -> impl IntoView {
    let (state, set_state) = create_signal(ValidationState::Empty);
    let validate = move |ev: FocusEvent| {
        if let Some(target) = ev.target() {
            match target.value_of().as_string() {
                Some(value) => match validation.as_ref().map(|valid| valid(value)) {
                    Some(Ok(_)) => set_state(ValidationState::Valid),
                    Some(Err(err)) => set_state(ValidationState::Invalid(err)),
                    None => set_state(ValidationState::Invalid("Failed to validate".into())),
                },
                None => set_state(ValidationState::Invalid("Failed to get value".into())),
            }
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
                on:focus=move |_| set_state(ValidationState::Dirty)
                placeholder=placeholder
            ></textarea>
        </div>
    }
}
