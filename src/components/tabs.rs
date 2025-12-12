use alloc::string::ToString;

use alloc::vec::Vec;


use crate::data_type::{Color, Shape};
use leptos::prelude::*;

#[component]
pub fn Tabs(
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(default={Shape::Square}, into)] shape: Shape,
    #[prop(optional)] default_tab: Option<&'static str>,
    #[prop(into)] tabs: Vec<TabMeta>,
    tab_content: fn(&'static str) -> AnyView,
) -> impl IntoView {
    // Determine initial active tab
    let initial_tab = default_tab
        .or_else(|| tabs.first().map(|tab| tab.id))
        .unwrap_or("");

    let (active_tab, set_active_tab) = signal(initial_tab);

    view! {
        <div class="tabs-container" data-color=color data-shape=shape>
            <div class="tabs-header" role="tablist">
                {tabs
                    .iter()
                    .map(|tab| {
                        let id = tab.id;
                        let label = tab.label;
                        let is_active = move || active_tab.get() == id;
                        view! {
                            <button
                                class="tab-button"
                                data-active=is_active().to_string()
                                role="tab"
                                on:click=move |_| set_active_tab.set(id)
                            >
                                {label}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="tabs-content">{move || tab_content(active_tab.get())}</div>
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct TabMeta {
    pub id: &'static str,
    pub label: &'static str,
}
