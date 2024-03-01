use crate::data_type::{Shape, Style};
use leptos::*;

#[component]
pub fn Badge(
    /// The style of the badge.
    #[prop(optional, into)]
    style: Option<Style>,
    /// The shape of the badge.
    #[prop(optional, into)]
    shape: Option<Shape>,
    children: Children,
) -> impl IntoView {
    view! { <span class="badge" data-style=style data-shape=shape>{children()}</span> }
}
