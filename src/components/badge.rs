use crate::data_type::{Color, Fill, Position, Shape};
use leptos::prelude::*;

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
    #[prop(default={Shape::Pill}, into)] shape: Shape,
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(default={Fill::Solid}, into)] fill: Fill,
    #[prop(default={Position::TopRight}, into)] position: Position,
) -> impl IntoView {
    view! {
        <span class="badge" data-color=color data-shape=shape data-fill=fill data-position=position>
            {children()}
        </span>
    }
}
