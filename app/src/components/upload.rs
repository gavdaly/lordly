use leptos::*;

#[component]
pub fn Upload(
    #[prop(into)] label: String,
    #[prop(into)] name: String,
    #[prop(into, optional)] accept: Option<String>,
    #[prop(optional)] multiple: Option<bool>,
) -> impl IntoView {
    view! {
        <aside>
            <div class="dropzone" />
            <div>
                <label for=name.clone()>{label}</label>
                <input type="file" id=name.clone() accept=accept multiple=multiple />
            </div>
        </aside>
    }
}
