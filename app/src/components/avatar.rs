use leptos::*;

/// A component to display an avatar image.
#[component]
pub fn Avatar(
    ///href The URL of the image.
    #[prop(into)]
    href: String,
    /// The name of the person the avatar represents.
    #[prop(optional)]
    name: String,
) -> impl IntoView {
    let alt = format!("avatar image for {}", name);
    view! {
        <picture>
            <img href=href alt=alt />
        </picture>
    }
}
