use leptos::*;

#[component]
pub fn Dialog(children: Children, id: String) -> impl IntoView {
    view! { <dialog id=id>{children()}</dialog> }
}
