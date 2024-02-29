use leptos::*;

// In leptos should work as
// <Input<Text>>

// states should be 'valid', 'dirty', 'invalid'

#[component]
pub fn Input(name: String, label: String, _summary: Option<String>) -> impl IntoView {
    //<!-- Show summary if there is one -->
    // <Show when=summary.is_some()>
    //     <div class="summary">{summary}</div>
    // </Show>

    //<!-- Show error if there is one -->
    // <div data-type="error">""</div>
    view! {
        <div class="label">
            <label for=name.clone()>{label}</label>
            <input type="text" name id=name/>
        </div>
    }
}

#[component]
fn InputCheckBoxes(label: String, options: Vec<(String, String)>) -> impl IntoView {
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
fn InputRadioButtons(label: String, options: Vec<(String, String)>) -> impl IntoView {
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
