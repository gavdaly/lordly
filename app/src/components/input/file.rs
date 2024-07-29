use leptos::*;

#[component]
pub fn File(
    name: String,
    #[prop(default = false)] drop_area: bool,
    #[prop(default = "".into(), into)] accept: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    if drop_area {
        view! {
            <div>
                <div class="dropzone" />
                <div>
                    <label for=name.clone()>"Drop files here"</label>
                    <input type="file" id=name.clone() name=name accept=accept multiple=multiple />
                </div>
            </div>
        }
    } else {
        view! {
            <div>
                <label for=name.clone()>Select files</label>
                <input type="file" id=name.clone() name=name accept=accept multiple=multiple />
            </div>
        }
    }
}
