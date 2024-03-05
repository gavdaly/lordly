use leptos::*;

/// A higher order component that wraps its children in an `span` tag, it shows a colored dot, empty or with a number.
#[component]
pub fn Indicator(#[prop(optional)] number: usize, children: Children) -> impl IntoView {
    view! {
        <span>
            <i class="indicator" data-number=number>{number}</i>
            {children()}
        </span>
    }
}
