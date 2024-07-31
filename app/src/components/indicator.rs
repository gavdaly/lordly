use crate::data_type::{Color, Fill, Position, Shape};
use leptos::*;

/// A higher order component that wraps its children in an `span` tag, it shows a colored dot, empty or with a number.
///
///
#[component]
pub fn Indicator(
    #[prop(optional)] number: usize,
    children: Children,
    #[prop(default = "round".into(), into)] shape: Shape,
    #[prop(default="solid".into(), into)] fill: Fill,
    #[prop(default="primary".into(), into)] color: Color,
    #[prop(default="top-left".into(), into)] position: Position,
) -> impl IntoView {
    view! {
        <span class="indicator-container">
            <i class="indicator" data-position=position data-shape=shape data-fill=fill data-color=color data-number=number>{number}</i>
            <span class="indicator-children">{children()}</span>
        </span>
    }
}
