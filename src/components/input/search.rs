use ev::Event;
use leptos::*;
use leptos_router::ActionForm;

/// A search component.
///
#[component]
pub fn Search(
    #[prop(default=(|_|{vec![]}).into(), into)] hints: Callback<String, Vec<String>>,
    #[prop(default="find".into(), into)] button: String,
) -> impl IntoView {
    let submit = Action::<Search, _>::server();
    let (auto_complete, set_auto_complete) = create_signal(None);
    let changed = move |input: Event| {
        let value = input.value_of().as_string().unwrap();
        set_auto_complete(Some(hints(value)));
    };

    view! {
        <ActionForm action=submit>
            <label for="search">"Search"</label>
            <input type="search" id="search" name="q" on:change=changed/>
            <button type="submit">{button}</button>
            {move || match auto_complete() {
                Some(hints) => {
                    view! {
                        <ul>
                            {hints.into_iter().map(|hint| view! { <li>{hint}</li> }).collect_view()}
                        </ul>
                    }
                        .into_view()
                }
                None => view! { <></> }.into_view(),
            }}

        </ActionForm>
    }
}

#[server]
async fn search(query: String) -> Result<(), ServerFnError> {
    println!("{query}");
    Ok(())
}
