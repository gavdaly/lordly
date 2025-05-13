use crate::data_type::{Color, Fill, Shape};
use leptos::prelude::*;

#[component]
pub fn Tag(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] shape: Option<Shape>,
    #[prop(optional, into)] fill: Option<Fill>,
) -> impl IntoView {
    view! {
        <div class="space-item">
            <span //data-color=color data-shape=shape data-fill=fill
                class="tag">
                {children()}
            </span>
        </div>
    }
}
