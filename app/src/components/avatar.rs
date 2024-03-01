use leptos::*;

#[component]
pub fn Avatar(#[prop(into)] href: String, #[prop(optional)] name: String) -> impl IntoView {
    let alt = format!("avatar image for {}", name);
    view! {
        <picture>
            <img href=href alt=alt />
        </picture>
    }
}
