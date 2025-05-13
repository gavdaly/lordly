use crate::data_type::{ButtonType, Color, Fill, Shape};
use leptos::prelude::*;

/// A button is a clickable element that can be used to trigger actions.
/// - `type_`: The type of the button.
/// - `shape`: The shape of the button.
/// - `color`: The style of the button.
/// - `children`: The children of the button.
/// - `color`: The color of the button.
/// - `fill`: options are solid, ghost, text.
#[component]
pub fn Button(
    #[prop(into, optional)] type_: Option<ButtonType>,
    #[prop(into, optional)] shape: Option<Shape>,
    #[prop(into, optional)] color: Option<Color>,
    #[prop(into, optional)] fill: Option<Fill>,
    children: Children,
) -> impl IntoView {
    view! {
        <button //type=type_ //data-shape=shape data-color=color data-fill=fill
        >
            {children()}
        </button>
    }
}
