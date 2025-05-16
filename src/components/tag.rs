use crate::data_type::{Color, Fill, Shape};
use leptos::prelude::*;

#[component]
pub fn Tag(
    children: Children,
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(default={Shape::Square}, into)] shape: Shape,
    #[prop(default={Fill::Solid}, into)] fill: Fill,
) -> impl IntoView {
    view! {
        <div class="space-item">
            <span data-color=color data-shape=shape data-fill=fill class="tag">
                {children()}
            </span>
        </div>
    }
}
