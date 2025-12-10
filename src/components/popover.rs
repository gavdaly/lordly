use alloc::string::ToString;

use alloc::string::String;

use leptos::prelude::*;

use crate::data_type::Anchor;

#[component]
pub fn Popover(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default={Anchor::Top}, into)] anchor: Anchor,
) -> impl IntoView {
    view! {
        <div id=id popover data-anchor=anchor
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTarget(
    children: Children,
    #[prop(into)] popovertarget: String,
    #[prop(optional, into)] action: Option<String>,
) -> impl IntoView {
    let popoveraction = action.unwrap_or("toggle".to_string());
    view! {
        <button popovertarget=popovertarget popover=popoveraction>
            {children()}
        </button>
    }
}

// Need to add this later to suppor the polyfill.
// <script
//   src="https://cdn.jsdelivr.net/npm/@oddbird/popover-polyfill@latest"
//   crossorigin="anonymous"
//   defer
// ></script>
