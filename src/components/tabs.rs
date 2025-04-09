use crate::data_type::{Color, Shape};
use leptos::*;
use std::collections::HashMap;

/// A tab that should be used with the Tabs component
/// - `id`: Unique identifier for the tab
/// - `label`: Display label for the tab
/// - `active`: Whether this tab is currently active
/// - `children`: The content to show when this tab is active
#[component]
pub fn Tab(
    id: &'static str,
    label: &'static str,
    #[prop(optional)] active: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="tab-panel" data-tab-id={id} data-active={active.to_string()} role="tabpanel" hidden={!active}>
            {children()}
        </div>
    }
}

/// A component that displays tabbed content with navigation
/// - `color`: The color scheme of the tabs
/// - `shape`: The shape of the tabs
/// - `default_tab`: The ID of the tab that should be active by default
/// - `children`: The Tab components to display
#[component]
pub fn Tabs(
    #[prop(into, optional)] color: Option<Color>,
    #[prop(into, optional)] shape: Option<Shape>,
    #[prop(optional)] default_tab: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    // Create a signal to track which tab is active
    let (active_tab, set_active_tab) = create_signal(default_tab.unwrap_or(""));

    // Store to collect tab information from children
    let tab_info: StoredValue<HashMap<&'static str, &'static str>> = store_value(HashMap::new());

    // Function to register tabs and their labels
    let register_tab = move |id: &'static str, label: &'static str| {
        tab_info.update_value(|map| {
            map.insert(id, label);
        });
    };

    // This will collect tab information from all children and return the actual tab panels
    let tabs_with_state = move || {
        let mut rendered_tabs = Vec::new();

        // First pass: collect all tab information
        let mut children_fns = children().nodes.into_iter();
        while let Some(child) = children_fns.next() {
            if let Some(child_view) = child.into_view() {
                // We need to extract metadata from each Tab component
                // This is a simplified version - in reality, you would need more complex logic
                // to collect tab metadata from actual components
                if let Some(tab_el) = child_view
                    .into_any()
                    .downcast_ref::<HtmlElement<html::AnyElement>>()
                {
                    if let Some(id) = tab_el.attribute("data-tab-id") {
                        if let Some(id_str) = id.as_str() {
                            let label = id_str; // In a real implementation, get the label property
                            register_tab(id_str, label);
                        }
                    }
                }
            }
        }

        // Second pass: render tabs with correct active state
        let active_id = active_tab.get();
        let children_fns = children().clone();
        for child in children_fns.nodes.into_iter() {
            if let Some(view) = child.into_view() {
                // You would need to set the active property based on active_tab
                rendered_tabs.push(view);
            }
        }

        rendered_tabs
    };

    // Click handler for tab buttons
    let handle_tab_click = move |id: &'static str| {
        set_active_tab.set(id);
    };

    view! {
        <div class="tabs-container" data-color=color data-shape=shape>
            <div class="tabs-header" role="tablist">
                {move || {
                    tab_info.with_value(|map| {
                        map.iter().map(|(id, label)| {
                            let id_clone = *id;
                            let is_active = active_tab.get() == id_clone;
                            view! {
                                <button
                                    class="tab-button"
                                    data-active={is_active.to_string()}
                                    role="tab"
                                    on:click=move |_| handle_tab_click(id_clone)
                                >
                                    {*label}
                                </button>
                            }
                        }).collect::<Vec<_>>()
                    })
                }}
            </div>
            <div class="tabs-content">
                {tabs_with_state}
            </div>
        </div>
    }
}
