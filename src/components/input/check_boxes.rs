

use alloc::vec::Vec;

use alloc::string::String;

use leptos::prelude::*;
use alloc::collections::BTreeSet;

/// A component that renders a customizable group of checkboxes.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the option id and display text.
/// - `selected`: An optional set of initially selected option ids.
/// - `on_change`: Optional callback triggered when selection changes.
/// - `disabled`: Whether the entire checkbox group is disabled.
/// - `class`: Optional CSS class for styling.
#[component]
pub fn CheckBoxes(
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
    #[prop(default = BTreeSet::new(), into)] selected: BTreeSet<String>,
    #[prop(optional)] on_change: Option<Callback<BTreeSet<String>>>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (selected_values, set_selected_values) = signal(selected);

    let handle_change = move |id: String, checked: bool| {
        set_selected_values.update(|selected| {
            if checked {
                selected.insert(id);
            } else {
                selected.remove(&id);
            }
        });

        if let Some(callback) = on_change {
            callback.run(selected_values.get());
        }
    };

    view! {
        <fieldset class=class disabled=disabled aria-disabled=disabled>
            <legend>{label}</legend>
            <For
                each=move || options.clone()
                key=|item| item.0.clone()
                children=move |(id, display_text)| {
                    let id = Memo::new(move|_| id.clone());
                    let is_checked = Memo::new(move |_| {
                        let values = selected_values.get();
                        values.contains(&id.get())
                    });

                    view! {
                        <div class="checkbox-item">
                            <input
                                type="checkbox"
                                id=id
                                name=id.clone()
                                value=id.clone()
                                checked=is_checked.get()
                                disabled=disabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    handle_change(id.get(), checked);
                                }
                            />
                            <label for=id>{display_text}</label>
                        </div>
                    }
                }
            />
        </fieldset>
    }
}
