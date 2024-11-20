use leptos::*;

/// A component that renders a list of checkboxes.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the label and the name of the checkbox.
#[component]
pub fn CheckBoxes(
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
) -> impl IntoView {
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
