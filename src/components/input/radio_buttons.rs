use leptos::*;

/// A component that renders a list of radio buttons.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the label and the name of the radio button.
#[component]
pub fn RadioButtons(
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{label}</legend>
            {options
                .iter()
                .map(|(name, label)| {
                    view! {
                        <div class="input">
                            <input type="radio" name=name id=name vaule=name/>
                            <label for="name">{label}</label>
                        </div>
                    }
                })
                .collect_view()}

        </fieldset>
    }
}
