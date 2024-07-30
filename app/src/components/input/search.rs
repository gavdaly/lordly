use ev::Event;
use leptos::*;

//_search_action: Action<String, Result<String, ServerFnError>>

/// A search component.
///
/// # Arguments
/// - `search_action`: The action that will be called when the form is submitted.
/// - `hints`: A callback that will be called when the input changes.
/// - `button`: The text of the submit button.
#[component]
pub fn Search(
    #[prop(optional)] search_action: Option<String>,
    #[prop(default=(|_|{vec![]}).into(), into)] hints: Callback<String, Vec<String>>,
    #[prop(default="find".into(), into)] button: String,
) -> impl IntoView {
    let (auto_complete, set_auto_complete) = create_signal(None);
    let changed = move |input: Event| {
        let value = input.value_of().as_string().unwrap();
        set_auto_complete(Some(hints(value)));
    };

    view! {
        <form action=search_action>
            <label for="search">"Search"</label>
            <input type="search" id="search" name="q" on:change=changed />
            <button type="submit">{button}</button>
            {move || match auto_complete() {
                Some(hints) => view! {
                    <ul>
                        {hints.into_iter().map(|hint| view! {<li>{hint}</li>}).collect_view()}
                    </ul>
                },
                None => view! {<ul></ul>},
            }}.into_view()
        </form>
    }
}
