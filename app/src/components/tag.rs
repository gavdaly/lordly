use crate::data_type::{Shape, Style};
use leptos::*;

#[component]
pub fn Tag(
    children: Children,
    #[prop(optional, into)] style: Option<Style>,
    #[prop(optional, into)] shape: Option<Shape>,
) -> impl IntoView {
    view! {
        <div class="space-item">
            <span data-style=style data-shape=shape class="tag">
                {children()}
            </span>
        </div>
    }
}
