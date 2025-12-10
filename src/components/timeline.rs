use alloc::string::ToString;

use alloc::string::String;

use crate::data_type::Color;
use leptos::prelude::*;

/// A timeline item representing a single event
/// - `date`: Date or time information for this event
/// - `color`: The color for this specific timeline item
/// - `icon`: Optional SVG/HTML content for the item's icon
/// - `children`: The content/description of the timeline event
#[component]
pub fn TimelineItem(
    #[prop(into, optional)] date: Option<String>,
    #[prop(default={Color::Primary}, into)] color: Color,
    // #[prop(into, optional)] icon: Option<View>,
    children: Children,
) -> impl IntoView {
    let date_clone = date.clone();
    let date_memo = Memo::new(move |_| date.clone());
    let has_date = Memo::new(move |_| date_clone.is_some());
    view! {
        <li data-color=color class="timeline-item" >
            <div class="timeline-point">
            // <Show when=move || icon.is_some()>
            //     <div class="timeline-icon">{icon}</div>
            // </Show>
            </div>
            <div class="timeline-content">
                <Show when=move || has_date.get()>
                    <div class="timeline-date">{date_memo.get()}</div>
                </Show>
                <div class="timeline-body">
                    {children()}
                </div>
            </div>
        </li>
    }
}

/// A timeline component for displaying events in chronological order
/// - `color`: The default color scheme of the timeline
/// - `alternate`: Whether to display items on alternating sides
/// - `children`: The TimelineItem components
#[component]
pub fn Timeline(
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(optional)] alternate: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            class="timeline"
            data-color=color
            data-alternate={alternate.to_string()}
        >
            {children()}
        </ul>
    }
}
