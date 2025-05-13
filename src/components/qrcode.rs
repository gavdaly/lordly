use leptos::prelude::*;

#[component]
pub fn qrcode(value: String) -> impl IntoView {
    // let qrcode = create_action::<GenerateQr, ServerFnError>();
    // qrcode.dispatch(value);
    view! { <div class="qrcode">{value}</div> }
}

// pub async fn generate_qr(value: String) -> Result<String, ServerFnError> {
//     set_timeout(||{}, 1).await;
//     Ok(format(""))
// }
