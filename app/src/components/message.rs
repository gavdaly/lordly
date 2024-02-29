use leptos::*;

#[component]
pub fn Message(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}
