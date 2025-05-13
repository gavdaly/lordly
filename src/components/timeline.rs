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
    #[prop(into, optional)] color: Option<Color>,
    #[prop(into, optional)] icon: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        //data-color=color
        <li class="timeline-item" >
            <div class="timeline-point">
                {icon.map(|icon| view! { <div class="timeline-icon">{icon()}</div> })}
            </div>
            <div class="timeline-content">
                {date.map(|d| view! { <div class="timeline-date">{d}</div> })}
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
    #[prop(into, optional)] color: Option<Color>,
    #[prop(optional)] alternate: bool,
    children: Children,
) -> impl IntoView {
    view! {
        //data-color=color
        <ul
            class="timeline"
            data-alternate={alternate.to_string()}
        >
            {children()}
        </ul>
    }
}
