use crate::data_type::{Color, Shape};
use leptos::*;

/// A badge is a small status descriptor for UI elements.
///
/// - `shape`: The shape of the badge.
/// - `children`: The children of the badge.
/// - `color`: The color of the badge.
/// - `size`: The size of the badge.
/// - `fill`: The fill of the badge.
/// - `position`: The position of the badge.
#[component]
pub fn Badge(
    /// The shape of the badge.
    #[prop(optional, into)]
    shape: Option<Shape>,
    #[prop(optional, into)] color: Option<Color>,
    children: Children,
) -> impl IntoView {
    view! { <span class="badge" data-color=color data-shape=shape>{children()}</span> }
}
