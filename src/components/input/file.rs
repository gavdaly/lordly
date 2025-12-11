use alloc::string::String;

use leptos::prelude::*;

/// A component that allows users to upload files.
///
/// # Arguments
/// - `name` The name of the input field.
/// - `accept` The file types to accept.
/// - `multiple` Whether to allow multiple files.
#[component]
pub fn File(
    #[prop(into)] name: String,
    #[prop(default = false)] _drop_area: bool,
    #[prop(default = "".into(), into)] _accept: String,
    #[prop(default = false)] _multiple: bool,
) -> impl IntoView {
    let name = Signal::derive(move || name.clone());

    view! {
        <div>
            <label for=name.get()>Select files</label>
            <input type="file" id=name.get() name=name.get() accept multiple />
        </div>
    }
}
