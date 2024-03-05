use leptos::*;

/// Created a pagination component with the current page's property set to `data-active='true'`
#[component]
pub fn Pagination(
    current: usize,
    total: usize,
    #[prop(into, optional)]
    url_base: String,
    #[prop(optional)] max_visible: Option<usize>,
) -> impl IntoView {
    let max_visible = max_visible.unwrap_or(5);
    let prev = || current != 1;
    let next = || current != total;
    let url_base = || url_base.clone();
    let window = move || {
        let half_visible = max_visible / 2;
        let start = if current <= half_visible {
            1
        } else {
            current - half_visible
        };
        let end = if current + half_visible > total {
            total
        } else {
            current + half_visible
        };
        start..=end
    };
    view! {
        <aside>
            {if prev() { view!{
                <><a href=format!("{}{}", url_base(), current - 1)>"prev"</a></>
            }} else { view!{
                <><i>"prev"</i></>
            }}}
            <ul>
               { window().map(|i| {
                    if i == current {
                        view! { <li data-active=true>{i}</li> }
                    } else {
                        view! { <li data-active=false><a href=format!("{}{}", url_base(), i)>{i}</a></li> }
                    }
                }).collect_view()}
            </ul>
            {if next() { view!{
                <><a href=format!("{}{}", url_base(), current + 1)>"next"</a></>
            }} else { view!{
                <><i>"next"</i></>
            }}}
        </aside>
    }
}
