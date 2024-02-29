use leptos::*;

#[component]
pub fn Pagination(current: u32, total: u32) -> impl IntoView {
    view! { <ul>{current} <li>1</li> <li>2</li> <li>3</li> <li>4</li> <li>5</li> {total}</ul> }
}
