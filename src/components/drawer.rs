use alloc::string::String;

use crate::data_type::Anchor;
use leptos::prelude::*;

/// A drawer component.
///
/// # Arguments
/// - `id`: The id of the drawer.
/// - `children`: The children of the drawer.
/// - `anchor`: The anchor of the drawer.
#[component]
pub fn Drawer(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default={Anchor::Left}, into)] anchor: Anchor,
) -> impl IntoView {
    view! {
        <dialog id=id data-anchor=anchor>
            {children()}
        </dialog>
    }
}
