use crate::data_type::Color;
use leptos::*;

// enum Dissmissable {
//     None,
//     WithCloser(Option<u32>),
//     WithOutCloser(Option<u32>)
// }

/// An alert is an element that displays a brief, important message in a way that attracts the user’s attention without interrupting the user’s task. Alerts are typically intended by read out dynamically by a screen reader.
///
/// @attr
/// - color: The color of the alert.
/// - size: The size of the alert.
/// - dismissible: Whether the alert is dismissible.
/// - has-closer: Whether the alert has a closer.
/// - heading: The heading of the alert.
/// - ttl: The time-to-live of the alert.
/// - children: The content of the alert.
#[component]
pub fn Alert(
    children: Children,
    #[prop(into, optional)] color: Option<Color>,
    #[prop(default = false, into)] has_closer: bool,
    #[prop(into, optional)] ttl: Option<u32>,
    #[prop(default = "".into(), into)] wrapper_class: String,
    #[prop(default = "".into(), into)] class: String,
    #[prop(default = "".into(), into)] alert_close: String,
) -> impl IntoView {
    // set the time-to-live of the alert.
    if let Some(ttl) = ttl {
        let _ = ttl;
    }
    view! {
        <div class=format!("alert #{wrapper_class}") data-color=color>
            <span class=format!("alert_body #{class}")>{children()}</span>
            <Show when=move || has_closer>
                <span class=format!("alert_close #{alert_close}")></span>
            </Show>
        </div>
    }
}
