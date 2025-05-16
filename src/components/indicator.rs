use crate::data_type::{Color, Fill, Position, Shape};
use leptos::prelude::*;

/// A higher order component that wraps its children in an `span` tag, it shows a colored dot, empty or with a number.
/// - `number`: The number to display in the indicator.
/// - `shape`: The shape of the indicator.
/// - `fill`: The fill of the indicator.
/// - `color`: The color of the indicator.
/// - `position`: The position of the indicator.
#[component]
pub fn Indicator(
    children: Children,
    #[prop(optional)] number: Signal<Option<u8>>,
    #[prop(default="round".into(), into)] shape: Shape,
    #[prop(default="solid".into(), into)] fill: Fill,
    #[prop(default="primary".into(), into)] color: Color,
    #[prop(default="top-right".into(), into)] position: Position,
) -> impl IntoView {
    view! {
        <span class="indicator-container">
            <i
                class="indicator"
                data-position=position
                data-shape=shape
                data-fill=fill
                data-color=color
                data-number=number
            >
                {number}
            </i>
            <span class="indicator-children">{children()}</span>
        </span>
    }
}
