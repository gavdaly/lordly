use leptos::*;

#[component]
pub fn Drawer(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default=(|_|{}).into(), into)] toggle: Callback<()>,
) -> impl IntoView {
    view! { <dialog id=id>{children()}</dialog> }
}
