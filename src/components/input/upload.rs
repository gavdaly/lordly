use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

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
        let input_id = name.clone();
        let drop_zone_id = format!("{}-dropzone", name);

        let prevent_default = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            ev.stop_propagation();
        };

        let on_drop = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            ev.stop_propagation();

            let input_element = document()
                .get_element_by_id(&input_id)
                .and_then(|el| el.unchecked_into::<web_sys::HtmlInputElement>());

            if let Some(files) = ev.data_transfer().and_then(|dt| dt.files()) {
                if let Some(input) = input_element {
                    let dt = web_sys::DataTransfer::new().unwrap();
                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            dt.add_file(&file);
                        }
                    }
                }
            }
        };

        view! {
            <div class="input drop">
                <div
                    class="dropzone"
                    id=drop_zone_id
                    on:dragenter=prevent_default.clone()
                    on:dragover=prevent_default.clone()
                    on:dragleave=prevent_default.clone()
                    on:drop=move |ev| on_drop(ev)
                >
                    <div>
                        <label for=input_id>"Drop files here"</label>
                        <input type="file" id=input_id name=name accept=accept multiple=multiple/>
                    </div>
                </div>
            </div>
        }
    } else {
        view! {
            <div>
                <label for=name.clone()>Select files</label>
                <input type="file" id=name.clone() name=name accept=accept multiple=multiple/>
            </div>
        }
    }
}
