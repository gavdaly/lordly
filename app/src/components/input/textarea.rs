use leptos::*;

#[component]
pub fn TextArea(
    #[prop(optional, into)] label: String,
    #[prop(into)] name: String,
) -> impl IntoView {
    view! {
        <div class="textarea">
            <label for=name.clone()>{label}</label>
            <textarea id=name.clone() name=name />
        </div>
    }
}
