use crate::data_type::Color;
use leptos::prelude::*;

#[component]
pub fn Toast(
    children: Children,
    #[prop(default={Color::Primary}, into)] color: Color,
) -> impl IntoView {
    view! { <div
        data-color=color
    >{children()}</div> }
}
