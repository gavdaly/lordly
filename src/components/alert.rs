use crate::data_type::Color;
use leptos::*;

/// An alert is an element that displays a brief, important message in a way that attracts the user’s attention without interrupting the user’s task. Alerts are typically intended by read out dynamically by a screen reader.
#[component]
pub fn Alert(
    children: Children,
    /// The title of the alert.
    #[prop(into)]
    title: String,
    /// The time-to-live of the alert in seconds.
    /// None means the alert will not disappear automatically.
    #[prop(into, optional)]
    ttl: Option<u32>,
    /// The color of the alert.
    #[prop(default={Color::Primary}, into)]
    color: Color,
    /// Whether the alert can be closed by the user.
    #[prop(default = false, into)]
    has_closer: bool,
    /// The class of the wrapper element.
    #[prop(default = "".into(), into)]
    wrapper_class: String,
    /// The class of the alert body.
    #[prop(default = "".into(), into)]
    class: String,
    /// The class of the alert closer.
    #[prop(default = "".into(), into)]
    alert_close: String,
) -> impl IntoView {
    // set the time-to-live of the alert.
    if let Some(ttl) = ttl {
        let _ = ttl;
    }
    view! {
        <div class=format!("alert {wrapper_class}") data-color=color>
            <span class="alert_title">{title}</span>
            <span class=format!("alert_body {class}")>{children()}</span>
            <Show when=move || has_closer>
                <button class=format!("alert_close {alert_close}")>"×"</button>
            </Show>
        </div>
    }
}
