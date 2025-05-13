use leptos::ev::*;
use leptos::form::*;
use leptos::prelude::*;
use leptos::server_fn::*;

/// A search component.
///
#[component]
pub fn Search(
    #[prop(default=Callback::from(|_: String| vec![]), into)] hints: Callback<String, Vec<String>>,
    #[prop(default="find".into(), into)] button: String,
    // #[prop(into)] action: ServerAction<S>,
) -> impl IntoView {
    let (auto_complete, set_auto_complete) = signal(None);
    let changed = |ev: InputEvent| {
        let Some(value) = ev.value_of().as_string() else {
            return;
        };
        if value.is_empty() {
            return;
        };
        set_auto_complete.set(Some(hints.run(value)));
    };

    view! {
        <form>
        // <Action`Form action>
            <label for="search">"Search"</label>
            <input type="search" id="search" name="q" on:change=changed/>
            <button type="submit">{button}</button>
            <Show when=move || auto_complete.get().is_some()
                children=move || {
                    let hints = auto_complete.get().unwrap();
                    view! {
                        <ul>
                            {hints.into_iter().map(|hint| view! { <li>{hint}</li> }).collect_view()}
                        </ul>
                    }
                }
            />
        // </ActionForm>
        </form>
    }
}
