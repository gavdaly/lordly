use leptos::*;

//_search_action: Action<String, Result<String, ServerFnError>>

#[component]
pub fn Search(#[prop(optional)] search_action: Option<String>) -> impl IntoView {
    view! {
        <form action=search_action>
            <label for="search">"Search"</label>
            <input type="search" id="search" name="q" />
            <button type="submit">"find"</button>
        </form>
    }
}