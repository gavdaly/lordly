use crate::data_type::Style;
use leptos::*;

#[component]
pub fn Alert(children: Children, #[prop(into, optional)] style: Option<Style>) -> impl IntoView {
    view! { <div data-style=style>{children()}</div> }
}
