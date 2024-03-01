use crate::data_type::{ButtonType, Shape, Style};
use leptos::*;

#[component]
pub fn Button(
    #[prop(into, optional)] type_: Option<ButtonType>,
    #[prop(into, optional)] shape: Option<Shape>,
    #[prop(into, optional)] style: Option<Style>,
    children: Children,
) -> impl IntoView {
    view! { <button type=type_ data-shape=shape data-style=style>{children()}</button> }
}
