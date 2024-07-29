use leptos::*;

#[component]
pub fn Loading(children: Children) -> impl IntoView {
    view! {
        <aside class="loading">
            <svg use_="#loading"></svg>
            {children()}
        </aside>
    }
}
