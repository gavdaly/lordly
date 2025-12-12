use alloc::format;

use alloc::string::String;

use leptos::ev::FocusEvent;
use leptos::prelude::*;

/// A switch component.
///
/// # Arguments
/// - `name`: The name of the switch.
/// - `wrapper_class`: The class of the wrapper div.
/// - `label_class`: The class of the label.
/// - `class`: The class of the input.
/// - `validation`: A callback that will be called when the input is blured.
#[component]
pub fn Switch(
    #[prop(into)] name: String,
    #[prop(default = String::from(""))] wrapper_class: String,
    #[prop(default = String::from(""))] label_class: String,
    #[prop(default = String::from(""))] class: String,
    #[prop(default=Callback::new(|_|{true}), into)] validation: Callback<String, bool>,
) -> impl IntoView {
    let blured = move |value: FocusEvent| {
        let Some(value) = value.value_of().as_string() else {
            return;
        };
        validation.run(value);
    };
    view! {
        <div class=format!("input #{wrapper_class}")>
            <label class=label_class for=name.clone()></label>
            <input class=class id=name name type="checkbox" on:blur=blured />
        </div>
    }
}
