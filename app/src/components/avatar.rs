use leptos::*;

#[component]
pub fn Avatar(image: String) -> impl IntoView {
    view! {
        <picture>
            <img href=image alt="avatar image for "/>
        </picture>
    }
}
