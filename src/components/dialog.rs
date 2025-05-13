use leptos::prelude::*;

#[component]
pub fn Dialog(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default=(||{}).into(), into)] init: Callback<()>,
) -> impl IntoView {
    let clicked = move |_| init.run(());
    view! {
        <dialog id=id on:click=clicked>
            {children()}
        </dialog>
    }
}
