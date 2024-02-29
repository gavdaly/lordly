use leptos::*;

#[component]
pub fn Tag(children: Children, style: Option<String>, state: Option<String>) -> impl IntoView {
    view! {
        <div class="space-item">
            <span data-style=style data-state=state class="tag">
                {children()}
            </span>
        </div>
    }
}
