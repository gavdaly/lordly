use crate::data_type::Style;
use leptos::*;

#[component]
pub fn Modal(#[prop(optional, into)] style: Option<Style>, children: Children) -> impl IntoView {
    view! {
        <div data-style=style>
            {children()}
        </div>
        <dialog>
        </dialog>
    }
}

// https://github.com/GoogleChrome/dialog-polyfill
