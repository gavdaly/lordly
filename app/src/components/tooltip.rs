use leptos::*;

#[component]
pub fn Tooltip(children: Children) -> impl IntoView {
    view! { <span>{children()}</span> }
}
