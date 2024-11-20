use crate::data_type::{Color, Shape, Fill, Position};
use leptos::*;

/// A badge is a small status descriptor for UI elements.
///
/// - `children`: The children of the badge.
/// - `shape`: The shape of the badge.
/// - `color`: The color of the badge.
/// - `fill`: The fill of the badge.
/// - `position`: The position of the badge.
#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, into)] shape: Option<Shape>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] fill: Option<Fill>,
    #[prop(optional, into)] position: Option<Position>,
) -> impl IntoView {
    view! {
        <span class="badge" data-color=color data-shape=shape data-fill=fill data-position=position>
            {children()}
        </span>
    }
}
