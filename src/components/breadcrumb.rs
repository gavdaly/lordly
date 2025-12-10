use alloc::string::ToString;

use alloc::string::String;

use crate::data_type::{Color, Shape};
use leptos::prelude::*;

/// A breadcrumb item representing a navigation step
/// - `href`: The URL this breadcrumb item links to
/// - `active`: Whether this is the current/active item
/// - `children`: The content/label of the breadcrumb item
#[component]
pub fn BreadcrumbItem(
    #[prop(into, optional)] _href: Option<String>,
    #[prop(optional)] active: bool,
    _children: Children,
) -> impl IntoView {
    view! {
        <li class="breadcrumb-item" data-active={active.to_string()}>
            // {move || {
            //     match (active, href.is_some()) {
            //         (true, _) => view! { <span class="breadcrumb-text">{children()}</span> }.into_any(),
            //         (_, true) => view! { <a href={href.unwrap()} class="breadcrumb-link">{children()}</a> }.into_any(),
            //         (_, _) => view! { <span class="breadcrumb-text">{children()}</span> }.into_any(),
            //     }
            // }}
        </li>
    }
}

/// A breadcrumb navigation component
/// - `color`: The color scheme of the breadcrumb
/// - `shape`: The shape style of the breadcrumb
/// - `separator`: Custom separator to use between items (default: '/')
/// - `children`: The BreadcrumbItem components
#[component]
pub fn Breadcrumb(
    #[prop(into, optional)] _color: Option<Color>,
    #[prop(into, optional)] _shape: Option<Shape>,
    #[prop(into, optional)] separator: Option<String>,
    children: Children,
) -> impl IntoView {
    // Default separator
    let separator = separator.unwrap_or_else(|| "/".to_string());

    view! {
        <nav aria-label="Breadcrumb navigation">
            <ol
                class="breadcrumb"
                //data-color=color
                //data-shape=shape
                data-separator={separator}
            >
                {children()}
            </ol>
        </nav>
    }
}
