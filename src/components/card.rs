use crate::data_type::{Color, Shape};
use leptos::prelude::*;

/// A Card is a container component that groups related content and actions.
/// - `color`: The color scheme of the card.
/// - `shape`: The shape of the card (rounded, square, etc).
/// - `header`: Optional header content for the card.
/// - `footer`: Optional footer content for the card.
/// - `children`: The main content of the card.
#[component]
pub fn Card(
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(default={Shape::Square}, into)] shape: Shape,
    #[prop(into, optional)] header: Option<Children>,
    #[prop(into, optional)] footer: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card"
            data-color=color
            data-shape=shape
        >
            {header.map(|header| view! {
                <div class="card-header">
                    {header()}
                </div>
            })}
            <div class="card-body">
                {children()}
            </div>
            {footer.map(|footer| view! {
                <div class="card-footer">
                    {footer()}
                </div>
            })}
        </div>
    }
}
