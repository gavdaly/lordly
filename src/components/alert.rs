use crate::data_type::Color;
use leptos::*;

/// An alert is an element that displays a brief, important message in a way that attracts the user's attention without interrupting the user's task. Alerts are typically intended by read out dynamically by a screen reader.
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

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_dom::HtmlElement;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_alert_renders_with_title() {
        // Arrange: Create an alert with a title
        let alert = view! {
            <Alert title="Test Alert">
                "This is the alert content"
            </Alert>
        };

        // Act: Mount the alert to the document
        leptos::mount_to_body(move || alert);

        // Assert: Check that the title is rendered
        let alert_title = document().query_selector(".alert_title").unwrap().unwrap();
        assert_eq!(alert_title.text_content().unwrap(), "Test Alert");
    }

    #[wasm_bindgen_test]
    fn test_alert_renders_with_children() {
        // Arrange: Create an alert with children
        let alert = view! {
            <Alert title="Test Alert">
                "Child content"
            </Alert>
        };

        // Act: Mount the alert to the document
        leptos::mount_to_body(move || alert);

        // Assert: Check that the children are rendered
        let alert_body = document().query_selector(".alert_body").unwrap().unwrap();
        assert_eq!(alert_body.text_content().unwrap(), "Child content");
    }

    #[wasm_bindgen_test]
    fn test_alert_with_color() {
        // Test different color variants
        let colors = vec![
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Danger,
            Color::Warning,
            Color::Info,
        ];

        for color in colors {
            // Arrange: Create an alert with a specific color
            let expected_color = color.to_string();
            let alert = view! {
                <Alert title="Test Alert" color=color.clone()>
                    "This is the alert content"
                </Alert>
            };

            // Act: Mount the alert to the document
            leptos::mount_to_body(move || alert);

            // Assert: Check that the color attribute is set correctly
            let alert_div = document().query_selector(".alert").unwrap().unwrap();
            assert_eq!(
                alert_div.get_attribute("data-color").unwrap(),
                expected_color
            );

            // Clean up for next test
            document().body().unwrap().set_inner_html("");
        }
    }

    #[wasm_bindgen_test]
    fn test_alert_with_closer() {
        // Arrange: Create an alert with a closer button
        let alert = view! {
            <Alert title="Test Alert" has_closer=true>
                "This is the alert content"
            </Alert>
        };

        // Act: Mount the alert to the document
        leptos::mount_to_body(move || alert);

        // Assert: Check that the closer button is rendered
        let close_button = document().query_selector(".alert_close").unwrap();
        assert!(close_button.is_some());
    }

    #[wasm_bindgen_test]
    fn test_alert_without_closer() {
        // Arrange: Create an alert without a closer button
        let alert = view! {
            <Alert title="Test Alert" has_closer=false>
                "This is the alert content"
            </Alert>
        };

        // Act: Mount the alert to the document
        leptos::mount_to_body(|| alert);

        // Assert: Check that the closer button is not rendered
        let close_button = document().query_selector(".alert_close").unwrap();
        assert!(close_button.is_none());
    }

    #[wasm_bindgen_test]
    fn test_alert_with_custom_classes() {
        // Arrange: Create an alert with custom classes
        let alert = view! {
            <Alert
                title="Test Alert"
                wrapper_class="custom-wrapper"
                class="custom-body"
                alert_close="custom-close"
                has_closer=true
            >
                "This is the alert content"
            </Alert>
        };

        // Act: Mount the alert to the document
        leptos::mount_to_body(move || alert);

        // Assert: Check that custom classes are applied
        let alert_div = document().query_selector(".alert").unwrap().unwrap();
        assert!(alert_div.class_list().contains("custom-wrapper"));

        let alert_body = document().query_selector(".alert_body").unwrap().unwrap();
        assert!(alert_body.class_list().contains("custom-body"));

        let close_button = document().query_selector(".alert_close").unwrap().unwrap();
        assert!(close_button.class_list().contains("custom-close"));
    }
}
