use alloc::format;

use alloc::string::String;

use leptos::prelude::*;

/// A rating component.
/// # Arguments
/// - `max`: The maximum rating.
/// - `name`: The name of the input.
#[component]
pub fn Rate(
    #[prop(default = 5)] max: u8,
    #[prop(into)] name: String,
    #[prop(into, optional)] label: Option<String>,
    #[prop(default={"⭐️".into_any()})] _unchecked: AnyView,
    #[prop(default={"🌟".into_any()})] _checked: AnyView,
) -> impl IntoView {
    let name = Signal::derive(move || name.clone());
    let label = Signal::derive(move || label.clone());
    let (selected, set_selected) = signal(0u8);

    view! {
        <fieldset class="rating">
            <Show when=move || label.get().is_some()>
                <legend>{move || label.get()}</legend>
            </Show>
            <aside class="rating-container">
                <For
                    each=move || 1..=max 
                    key=|index| *index
                    children=move |index| {
                        let input_id = format!("{}-{}", name.get(), index);
                        view! {
                            <div class="rating-item">
                                <input
                                    type="radio"
                                    name=name.get()
                                    id=input_id.clone()
                                    value=index
                                    on:input=move |_| set_selected.set(index)
                                    class="rating-input"
                                />
                                <label
                                    for=input_id
                                    class="rating-label"
                                    class:selected=move || index <= selected.get()
                                >
                                    <Show when=move || {index <= selected.get()} fallback=move || {view!{ "⭐️" }}>
                                        {"🌟"}
                                    </Show>
                                </label>
                            </div>
                        }
                    }
                />
            </aside>
            <style>
                ".rating-input {
                    position: absolute;
                    opacity: 0;
                    width: 0;
                    height: 0;
                }
                .rating-container {
                    display: flex;
                    flex-direction: row;
                    gap: 0.5rem;
                }
                .rating-label {
                    cursor: pointer;
                    color: #ccc;
                    transition: color 0.2s ease-in-out;
                }"
            </style>
        </fieldset>
    }
}
