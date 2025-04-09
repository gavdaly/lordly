use std::collections::HashSet;
use leptos::*;

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
    #[prop(default = HashSet::new(), into)] selected: HashSet<String>,
    #[prop(optional)] on_change: Option<Callback<HashSet<String>>>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let selected_values = create_rw_signal(selected);

    let handle_change = move |id: String, checked: bool| {
        selected_values.update(|selected| {
            if checked {
                selected.insert(id);
            } else {
                selected.remove(&id);
            }
        });

        if let Some(callback) = on_change {
            callback(selected_values.get_untracked());
        }
    };

    view! {
        <fieldset class=class disabled=disabled aria-disabled=disabled>
            <legend>{label}</legend>
            {options
                .iter()
                .map(|(id, display_text)| {
                    let id_clone = id.clone();
                    let is_checked = selected_values.with(|s| s.contains(id));

                    view! {
                        <div class="checkbox-item">
                            <input
                                type="checkbox"
                                id=id
                                name=id
                                value=id
                                checked=is_checked
                                disabled=disabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    handle_change(id_clone.clone(), checked);
                                }
                            />
                            <label for=id>{display_text}</label>
                        </div>
                    }
                })
                .collect_view()}
        </fieldset>
    }
}
