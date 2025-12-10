use alloc::vec::Vec;

use alloc::string::String;

use leptos::prelude::*;

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
            <For
                each=move || list.clone()
                key=|item| item.1.clone()
                    children=|(name, label)| { view!{
                        <div class="tag-toggle">
                            <input type="radio" name id=name.clone() value=name.clone()/>
                            <label for=name.clone()>{label}</label>
                        </div>
                    }}
            >
            </For>

        </fieldset>
    }
}
