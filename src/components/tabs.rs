use crate::data_type::{Color, Shape};
use leptos::prelude::*;
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
        let children = children();

        // Process each child tab
        for child in children.nodes.into_iter() {
            let child_view = child.into_view();
            // Extract tab metadata and register it
            if let Some(tab_el) =
                child_view.and_then(|v| v.into_any().downcast_ref::<HtmlElement<html::Div>>())
            {
                if let Some(id) = tab_el.attribute("data-tab-id") {
                    if let Some(label) = tab_el.attribute("data-label") {
                        register_tab(&id, &label);
                    }
                }
            }
            // Add the view to rendered tabs
            rendered_tabs.push(child_view);
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
                {move || tabs_with_state()}
            </div>
        </div>
    }
}
