use leptos::*;

#[component]
pub fn Drawer(children: Children, #[prop(into)] id: String) -> impl IntoView {
    view! { <dialog id=id>{children()}</dialog> }
}
