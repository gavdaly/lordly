use leptos::*;

#[component]
pub fn Indicator(children: Children) -> impl IntoView {
    view! {
        <i>
            {children()}
        </i>
    }
}
