use crate::data_type::{Color, Shape};
use leptos::*;

/// An Accordion Item component that can be expanded or collapsed
/// - `title`: The header text shown for this accordion item
/// - `open`: Whether this item is expanded by default
/// - `children`: The collapsible content
#[component]
pub fn AccordionItem(
    #[prop(into)] title: String,
    #[prop(optional)] open: bool,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(open);
    
    let toggle = move |_| set_is_open.update(|open| *open = !*open);
    
    view! {
        <div class="accordion-item" data-open={move || is_open.get().to_string()}>
            <button 
                class="accordion-header" 
                on:click=toggle
                aria-expanded={move || is_open.get().to_string()}
            >
                <span class="accordion-title">{title}</span>
                <span class="accordion-icon"></span>
            </button>
            <div 
                class="accordion-content" 
                style:display={move || if is_open.get() { "block" } else { "none" }}
            >
                {children()}
            </div>
        </div>
    }
}

/// An Accordion component for displaying collapsible content sections
/// - `color`: The color scheme of the accordion
/// - `shape`: The shape style of the accordion
/// - `allow_multiple`: Whether multiple items can be expanded at once
/// - `children`: The AccordionItem components
#[component]
pub fn Accordion(
    #[prop(into, optional)] color: Option<Color>,
    #[prop(into, optional)] shape: Option<Shape>,
    #[prop(optional)] allow_multiple: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div 
            class="accordion" 
            data-color=color 
            data-shape=shape
            data-allow-multiple={allow_multiple.to_string()}
        >
            {children()}
        </div>
    }
}