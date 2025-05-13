use leptos::*;

/// A component that renders an enhanced combobox.
///
/// # Arguments
/// - `label`: The label of the fieldset.
/// - `options`: A Vector of tuples with the value and the name of the option.
/// - `value`: Signal to bind the selected value.
/// - `on_change`: Optional callback when selection changes.
/// - `placeholder`: Optional placeholder text.
/// - `disabled`: Whether the combobox is disabled.
/// - `required`: Whether the combobox is required.
/// - `error`: Optional error message to display.
/// - `class`: Optional additional CSS classes.
/// - `size`: Optional size variant (sm, md, lg).
#[component]
pub fn ComboBox(
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] size: Option<String>,
) -> impl IntoView {
    let select_class = move || {
        format!(
            "combo-box {} {}",
            size.clone().unwrap_or_default(),
            if error.clone().is_some() {
                "has-error"
            } else {
                ""
            }
        )
    };

    view! {
        <fieldset class=move || class.clone().unwrap_or_default()>
            <legend>{label}</legend>
            <select
                class=select_class
                disabled=disabled
                required=required
                on:change=move |ev| {
                    let selected = event_target_value(&ev);
                    value.set(selected.clone());
                    if let Some(callback) = on_change.as_ref() {
                        callback.call(selected);
                    }
                }
                prop:value=move || value.get()
            >
                {placeholder.map(|p| view! { <option value="" disabled selected=move || value.get().is_empty()>{p}</option> })}
                {options
                    .iter()
                    .map(|(id, name)| {
                        view! { <option value=id>{name}</option> }
                    })
                    .collect_view()}
            </select>
            // {error.map(|err| view! { <div class="error-message">{err}</div> })}
        </fieldset>
    }
}
