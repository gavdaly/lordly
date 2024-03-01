use leptos::*;

#[component]
pub fn Switch(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <div>
            <label for=name.clone()></label>
            <input id=name name type="checkbox"/>
        </div>
    }
}
