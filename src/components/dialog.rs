use leptos::*;

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default=(|_|{}).into(), into)] init: Callback<()>,
) -> impl IntoView {
    let clicked = move |_| init(());
    view! {
        <dialog id=id on:click=clicked>
            {children()}
        </dialog>
    }
}
