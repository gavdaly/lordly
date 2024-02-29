use leptos::*;

#[component]
pub fn Toast(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}
