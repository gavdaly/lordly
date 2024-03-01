use leptos::{leptos_dom::IntoFragment, *};

/// Created a pagination component with the current page's property set to `data-active='true'`
#[component]
pub fn Pagination(
    current: usize,
    total: usize,
    #[prop(optional)] max_visible: Option<usize>,
) -> impl IntoView {
    let max_visible = max_visible.unwrap_or(5);
    view! {
        <aside>
            <a href="#">"prev"</a>
            <ul>
                { (0..max_visible).map(|i| {
                    view! { <li data-active=i == current>{i}</li> }
                }).into_fragment()}
            </ul>
            <a href="#">"next"</a>
        </aside>
    }
}
