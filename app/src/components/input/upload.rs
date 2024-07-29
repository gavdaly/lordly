use leptos::*;

/// A component that allows users to upload files.
///
/// # Arguments
/// - `name` The name of the input field.
/// - `drop_area` Whether to show a drop area.
/// - `accept` The file types to accept.
/// - `multiple` Whether to allow multiple files.
#[component]
pub fn Upload(
    name: String,
    #[prop(default = false)] drop_area: bool,
    #[prop(default = "".into(), into)] accept: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    if drop_area {
        view! {
            <div class="input drop">
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
