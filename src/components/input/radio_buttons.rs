use alloc::vec::Vec;

use alloc::string::String;

use leptos::prelude::*;

/// A component that renders a list of radio buttons.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the label and the name of the radio button.
#[component]
pub fn RadioButtons(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] options: Signal<Vec<(String, String)>>,
) -> impl IntoView {
    let options = options.clone();
    view! {
        <fieldset>
            <legend>{label.get()}</legend>
            <For each=move || options.get() key=|(name, _)| name.clone() children = move |(name, label)| {
                let name = name.clone();
                view!{
                    <div class="input">
                        <input type="radio" name=name.clone() id=name.clone() value=name/>
                        <label for="name">{label}</label>
                        </div>
                    }
                }
            />
        </fieldset>
    }
}
