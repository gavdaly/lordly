use leptos::*;

#[component]
pub fn Switch(name: String) -> impl IntoView {
    view! {
        <div>
            <label for=name.clone()></label>
            <input id=name name type="checkbox"/>
        </div>
    }
}
