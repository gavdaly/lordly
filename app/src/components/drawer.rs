use leptos::*;

#[component]
pub fn Drawer(children: Children, id: String) -> impl IntoView {
    view! { <dialog id=id>{children()}</dialog> }
}
