use alloc::vec::Vec;

use alloc::string::String;

use leptos::ev::*;
use leptos::prelude::*;

/// A search component.
#[component]
pub fn SearchBody(
    #[prop(optional)] hints: Option<Callback<String, Vec<Hint>>>,
    #[prop(default="find".into(), into)] button: String,
) -> impl IntoView {
    let (auto_complete, set_auto_complete) = signal(None::<Vec<Hint>>);

    let changed = move |ev: Event| {
        let Some(hints) = hints else {
            return;
        };
        let value = event_target_value(&ev);
        if value.len() == 0 {
            return;
        };
        let possible_hints = hints.run(value);
        set_auto_complete.set(Some(possible_hints));
    };

    view! {
        <>
            <label for="search">"Search"</label>
            <input type="search" id="search" name="q" on:change=changed />
            <button type="submit">{button}</button>
            <Show when=move || auto_complete.get().is_some()>
                <ul>
                    {move || {
                        if let Some(hints) = auto_complete.get() {
                            hints
                                .into_iter()
                                .map(|hint| {
                                    view! {
                                        <li>
                                            <a href=hint.url>{hint.text}</a>
                                        </li>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}

                </ul>
            </Show>
        </>
    }
}

#[derive(Debug, Clone)]
pub struct Hint {
    text: String,
    url: String,
}
