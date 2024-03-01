use leptos::*;

#[component]
pub fn CheckBoxes(label: String, options: Vec<(String, String)>) -> impl IntoView {
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
