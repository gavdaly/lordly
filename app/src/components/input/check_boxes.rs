use leptos::*;

#[component]
pub fn CheckBoxes(label: String, options: Vec<(String, String)>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{label}</legend>
            {options
                .iter()
                .map(|(id, name)| {
                    view! {
                        <div>
                            <input type="checkbox" name id value=id/>
                            <label for=id>{name}</label>
                        </div>
                    }
                })
                .collect_view()}
        </fieldset>
    }
}
