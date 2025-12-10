use alloc::vec::Vec;

use alloc::string::String;

use leptos::prelude::*;

/// A list of tags with checkboxes.
/// # Arguments
/// - `list`: A Vector of tuples with the label and the name of the tag.
/// - `label`: The label of the fieldset.
#[component]
pub fn TagList(
    #[prop(into)] list: Vec<(String, String)>,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <fieldset class="taglist">
            <legend>{label}</legend>
            <For each=move || list.clone() key=|k| k.1.clone() children=move |(l, name)| {
                view!{
                    <div class="tag-toggle">
                        <input type="checkbox" name=name.clone() id=name.clone() value=name.clone()/>
                        <label for=name>{l}</label>
                    </div>
                }
            }/>
        </fieldset>
    }
}
