use crate::data_type::Color;
use leptos::*;

#[component]
pub fn Modal(
    #[prop(optional, into)] style: Option<Color>,
    children: Children,
    modal: View,
) -> impl IntoView {
    view! {
        <div data-style=style>
            {children()}
        </div>
        <dialog>
            {modal}
        </dialog>
    }
}

// https://github.com/GoogleChrome/dialog-polyfill
