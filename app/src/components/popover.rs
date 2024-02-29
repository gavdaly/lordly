use leptos::*;

#[component]
pub fn Popover(children: Children) -> impl IntoView {
    view! { <div popover>{children()}</div> }
}
