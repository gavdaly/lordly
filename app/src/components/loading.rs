use leptos::*;

#[component]
pub fn Loading(children: Children) -> impl IntoView {
    view! {
        <section>
            <svg use_="#loading"></svg>
            {children()}
        </section>
    }
}
