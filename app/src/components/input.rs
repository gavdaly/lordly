use leptos::*;

// In leptos should work as
// <Input<Text>>

// states should be 'valid', 'dirty', 'invalid'

#[component]
pub fn Input(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
) -> impl IntoView {
    //<!-- Show error if there is one -->
    // <div data-type="error">""</div>
    view! {
        <div class="label">
            <label for=name.clone()>{label}</label>
            <input type="text" name id=name placeholder=placeholder />
        </div>
        <div class="summary">{summary}</div>
    }
}

#[component]
pub fn InputCheckBoxes(label: String, options: Vec<(String, String)>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{label}</legend>
            {options
                .iter()
                .map(|(name, label)| {
                    view! {
                        <div>
                            <input type="checkbox" name id=name value=name/>
                            <label for=name>{label}</label>
                        </div>
                    }
                })
                .collect_view()}
        </fieldset>
    }
}

#[component]
pub fn InputRadioButtons(label: String, options: Vec<(String, String)>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{label}</legend>
            {options
                .iter()
                .map(|(name, label)| {
                    view! {
                        <div>
                            <input type="radio" name=name id=name vaule=name/>
                            <label for="name">{label}</label>
                        </div>
                    }
                })
                .collect_view()}

        </fieldset>
    }
}
