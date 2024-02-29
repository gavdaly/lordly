use leptos::*;
// use leptos_router::*;

//_search_action: Action<String, Result<String, ServerFnError>>

#[component]
pub fn Search() -> impl IntoView {
    view! {
        <div>
            <form>
                <label for="search">Search</label>
                <input type="search" id="search" name="q"/>
                <button type="submit">find</button>
            </form>
        </div>
    }
}
