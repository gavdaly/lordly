use crate::data_type::{Color, Fill, Shape};
use leptos::*;

/// A higher order component that wraps its children in an `span` tag, it shows a colored dot, empty or with a number.
#[component]
pub fn Indicator(
    #[prop(optional)] number: usize,
    children: Children,
    shape: Shape,
    fill: Fill,
    color: Color,
) -> impl IntoView {
    view! {
        <span>
            <i class="indicator" data-shape=shape data-fill=fill data-color=color data-number=number>{number}</i>
            {children()}
        </span>
    }
}
