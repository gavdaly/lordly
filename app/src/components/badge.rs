use leptos::*;

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, into)] badge_type: Option<String>,
) -> impl IntoView {
    view! { <span class="badge" data-type=badge_type>{children()}</span> }
}
