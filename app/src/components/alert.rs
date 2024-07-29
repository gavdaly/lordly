use crate::data_type::Color;
use leptos::*;

/// An alert is an element that displays a brief, important message in a way that attracts the user’s attention without interrupting the user’s task. Alerts are typically intended by read out dynamically by a screen reader.
///
/// @attr
/// - style: The style of the alert.
/// - size: The size of the alert.
/// - additional-content: Additional content to be displayed in the alert.
/// - dismissible: Whether the alert is dismissible.
/// - has-closer: Whether the alert has a closer.
/// - heading: The heading of the alert.
/// - level: The level of the alert.
#[component]
pub fn Alert(children: Children, #[prop(into, optional)] color: Option<Color>) -> impl IntoView {
    view! { <div data-color=color>{children()}</div> }
}
