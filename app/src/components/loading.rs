use leptos::*;

#[component]
pub fn Loading(children: Children) -> impl IntoView {
    view! {
        <aside>
            <svg use_="#loading"></svg>
            {children()}
        </aside>
    }
}
