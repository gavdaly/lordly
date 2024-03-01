use leptos::*;

#[component]
pub fn Popover(
    children: Children,
    #[prop(into)] id: String,
    #[prop(into, optional)] anchor: Option<String>,
) -> impl IntoView {
    view! { <div id=id popover anchor=anchor>{children()}</div> }
}

#[component]
pub fn PopoverTarget(
    children: Children,
    #[prop(into)] popovertarget: String,
    #[prop(optional, into)] action: Option<String>,
) -> impl IntoView {
    let popoveraction = action.unwrap_or("toggle".to_string());
    view! { <button popovertarget=popovertarget popoveraction=popoveraction>{children()}</button> }
}
