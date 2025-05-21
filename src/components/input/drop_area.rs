use leptos::prelude::*;
use leptos::tachys::html::event::DragEvent;
use web_sys::{DataTransfer, File};

/// A component that allows users to upload files through drag and drop or file input.
///
/// # Arguments
/// - `name` - The name of the input field that uniquely identifies this component
/// - `accept` - The allowed file types to accept (e.g. "image/*", ".pdf", etc.)
/// - `disabled` - Whether the upload functionality is disabled
/// - `multiple` - Whether to allow multiple file uploads
/// - `on_files_change` - Callback triggered when files are added/removed
/// - `on_error` - Callback triggered when an error occurs during file upload
#[component]
pub fn DropArea(
    #[prop(into)] name: Signal<String>,
    #[prop(default = "".into(), into)] accept: Signal<String>,
    #[prop(default= false.into(), into)] disabled: Signal<bool>,
    #[prop(default = true)] multiple: bool,
    #[prop(optional)] on_files_change: Option<Callback<Vec<FileInfo>>>,
    #[prop(optional)] on_error: Option<Callback<&'static str>>,
) -> impl IntoView {
    let (error, set_error) = signal::<Option<&str>>(None);
    let (files, set_files) = signal::<Vec<FileInfo>>(vec![]);

    let drop_zone_id = Signal::derive(move || format!("{}-dropzone", name.get()));

    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let Some(dt) = ev.data_transfer() else {
            if let Some(on_error) = on_error {
                on_error.run("Failed to get data transfer");
            }
            return;
        };

        let Some(file_list) = dt.files() else {
            if let Some(on_error) = on_error {
                on_error.run("Failed to get file list");
            }
            return;
        };

        let Some(input_elem) = input_ref.get() else {
            if let Some(on_error) = on_error {
                on_error.run("Failed to get input element reference");
            }
            return;
        };

        if !multiple && file_list.length() > 1 {
            set_error.set(Some("Can only upload one file!"));
            if let Some(on_error) = on_error {
                on_error.run("Can only upload one file!");
            }
            return;
        }

        // Convert FileList to Vec<FileInfo>
        let mut new_files = vec![];
        for i in 0..file_list.length() {
            if let Some(file) = file_list.get(i) {
                new_files.push(FileInfo {
                    name: file.name(),
                    size: file.size(),
                    type_: file.type_(),
                });
            }
        }
        set_files.set(new_files.clone());
        input_elem.set_files(Some(&file_list));

        if let Some(on_files_change) = on_files_change {
            on_files_change.run(new_files);
        }
    };

    Effect::new(move |_| {
        if let Some(err) = error.get() {
            if let Some(on_error) = on_error {
                on_error.run(err);
            }
        }
    });

    view! {
        <div class="input drop">
            <div
                class="dropzone"
                id=drop_zone_id
                on:drop=on_drop
            >
                <div>
                    <label for=name.get()>"Drop files here"</label>
                    <input type="file" id=name.get() name=name.get() accept multiple=multiple node_ref=input_ref/>
                </div>
            </div>
            {move || {
                if files.get().is_empty() {
                    view! { <p class="text-gray-500">"No files"</p> }.into_any()
                } else {
                    view! {
                        <ul class="file-list">
                            {files.get()
                                .into_iter()
                                .map(|file| {
                                    let size_mb = (file.size / 1_048_576.0).round() / 100.0;
                                    view! {
                                        <li>
                                            {file.name} " (" {size_mb} " MB)"
                                            <span class="text-gray-500"> " - " {file.type_}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[derive(Clone)]
struct FileInfo {
    name: String,
    size: f64,
    type_: String,
}
