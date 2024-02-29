use leptos::*;

#[component]
pub fn Badge(children: Children) -> impl IntoView {
    view! { <span class="badge">{children()}</span> }
}
