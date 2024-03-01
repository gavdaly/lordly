use leptos::*;

#[component]
pub fn RadioButtons(label: String, options: Vec<(String, String)>) -> impl IntoView {
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
