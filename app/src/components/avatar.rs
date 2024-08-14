use crate::data_type::Shape;
use leptos::*;

/// A component to display an avatar image.
///
/// # Arguments
/// - `href` The URL of the image.
/// - `name` The name of the person the avatar represents.
/// - `shape` The shape of the avatar.
/// - `color` The color of the avatar.
/// - `label`
/// - `icon`
///
#[component]
pub fn Avatar(
    ///href The URL of the image.
    #[prop(into)]
    image: String,
    /// The name of the person the avatar represents.
    #[prop(optional)]
    name: Option<String>,
    #[prop(optional, into)] shape: Option<Shape>,
) -> impl IntoView {
    let alt = match name {
        Some(name) => format!("avatar image for {name}",),
        None => format!("avatar image for unknown user"),
    };
    view! {
        <picture data-shape=shape>
            <img href=image alt=alt/>
        </picture>
    }
}
