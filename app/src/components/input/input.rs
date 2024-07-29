use ev::FocusEvent;
use leptos::*;

enum ValidationState {
    Valid,
    Dirty,
    Invalid(String),
}

#[component]
pub fn Input(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default=(|_|{true}).into(), into)] validation: Callback<String, bool>,
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
        <div class="input">
            <div class="summary">{summary}</div>
            <label for=name.clone()>{label}</label>
            <input type="text" name id=name placeholder=placeholder on:blur=validate on:focus=focused />

            // {move || match validation_state() {
            //     ValidationState::Invalid(reason) => view! {<div data-type="error">reason</div>},
            //     ValidationState::Valid => view! {<div data-type="success"></div>},
            //     ValidationState::Dirty => view! {<div></div>},
            // }}.into_view()
        </div>

    }
}
