use alloc::string::ToString;

use alloc::format;

use alloc::string::String;

use crate::data_type::Shape;
use leptos::prelude::*;

/// A component to display an avatar image.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Avatar src="https://example.com/avatar.jpg"
///             name="John Doe"
///             shape="circle"
///             width="150" />
///     }
/// }
/// ```
#[component]
pub fn Avatar(
    ///href The URL of the image.
    #[prop(into)]
    src: String,
    /// The name of the person the avatar represents.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(default={Shape::Circular}, into)] shape: Shape,
    #[prop(default={"100".into()}, into)] width: String,
) -> impl IntoView {
    let alt = match name {
        Some(name) => format!("Avatar for {name}",),
        None => "avatar image for unknown user".to_string(),
    };
    view! { <img data-shape=shape
    src=src alt=alt width=width/> }
}
