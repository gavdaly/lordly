use crate::data_type::Color;
use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(optional, into)] _style: Option<Color>,
    children: Children,
    modal: AnyView,
) -> impl IntoView {
    view! {
        // data-style=style
        <div>{children()}</div>
        <dialog>{modal}</dialog>
    }
}

// https://github.com/GoogleChrome/dialog-polyfill
