pub mod app;
pub mod check;
pub mod components;
pub mod data_type;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod kitchen_sink;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
