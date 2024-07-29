use ev::FocusEvent;
use leptos::*;

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
        <div class=wrapper_class>
            <label class=label_class for=name.clone()></label>
            <input class=class id=name name type="checkbox" on:blur=blured />
        </div>
    }
}
