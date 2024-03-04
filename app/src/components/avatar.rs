use leptos::*;
use crate::data_type::Shape;

/// A component to display an avatar image.
#[component]
pub fn Avatar(
    ///href The URL of the image.
    #[prop(into)]
    href: String,
    /// The name of the person the avatar represents.
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional, into)]
    shape: Option<Shape>,
) -> impl IntoView {
    let alt = match name {
        Some(name) => format!("avatar image for {}", name),
        None => format!("avatar image for unknown user"),
    };
    view! {
        <picture data-shape=shape>
            <img href=href alt=alt />
        </picture>
    }
}
