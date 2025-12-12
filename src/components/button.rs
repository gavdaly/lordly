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
    #[prop(default={ButtonType::Button}, into)] type_: ButtonType,
    #[prop(default={Shape::Rounded}, into)] shape: Shape,
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(default={Fill::Solid}, into)] fill: Fill,
    children: Children,
) -> impl IntoView {
    view! {
        <button type=type_ data-shape=shape data-color=color data-fill=fill>
            {children()}
        </button>
    }
}
