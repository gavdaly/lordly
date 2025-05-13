use leptos::prelude::*;

#[component]
pub fn Message(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}
