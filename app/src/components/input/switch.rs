use ev::FocusEvent;
use leptos::*;

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
    #[prop(default = "".into())] wrapper_class: String,
    #[prop(default = "".into())] label_class: String,
    #[prop(default = "".into())] class: String,
    #[prop(default =(|_|{true}).into(), into)] validation: Callback<String, bool>,
) -> impl IntoView {
    let blured = move |value: FocusEvent| {
        validation(value.value_of().as_string().unwrap().clone());
    };
    view! {
        <div class={format!("input #{wrapper_class}")}>
            <label class=label_class for=name.clone()></label>
            <input class=class id=name name type="checkbox" on:blur=blured />
        </div>
    }
}
