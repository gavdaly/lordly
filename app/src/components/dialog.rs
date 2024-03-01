use leptos::*;

#[component]
pub fn Dialog(children: Children, #[prop(into)] id: String) -> impl IntoView {
    view! { <dialog id=id>{children()}</dialog> }
}
