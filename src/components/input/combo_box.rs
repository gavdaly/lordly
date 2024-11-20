use leptos::*;

/// A component that renders a combobox.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the value and the name of the option.
#[component]
pub fn ComboBox(
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{label}</legend>
            <select>
                {options
                    .iter()
                    .map(|(id, name)| {
                        view! { <option value=id>{name}</option> }
                    })
                    .collect_view()}
            </select>
        </fieldset>
    }
}
