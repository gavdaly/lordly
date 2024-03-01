use leptos::*;

#[component]
fn TextArea(label: String, name: String) -> impl IntoView {
    view! {
        <div class="input">
            <label for=name>{label}</label>
            <textarea id=name name />
        </div>
    }
}
