use leptos::*;

#[component]
fn TextArea(label: String, name: String, children: Children) -> impl IntoView {
    view! {
        <div class="input">
            <label for=name>{label}</label>
            <textarea id=name name>
                {children()}
            </textarea>
        </div>
    }
}