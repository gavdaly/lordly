use leptos::*;

/// A list of tags with radio buttons.
///
/// # Arguments
/// - `list`: A Vector of tuples with the label and the name of the tag.
/// - `label`: The label of the fieldset.
#[component]
pub fn PickList(
    #[prop(into)] list: Vec<(String, String)>,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <fieldset class="taglist">
            <legend>{label}</legend>

            {list
                .iter()
                .map(|(l, name)| {
                    view! {
                        <div class="tag-toggle">
                            <input type="radio" name id=name value=name/>
                            <label for=name>{l}</label>
                        </div>
                    }
                })
                .collect_view()}

        </fieldset>
    }
}
