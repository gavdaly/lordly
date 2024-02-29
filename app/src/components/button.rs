use leptos::*;

// enum ButtonType {
//     Button = "button",
//     Submit = "submit",
//     Reset = "reset",
// }

#[component]
pub fn Button(b_type: String, children: Children) -> impl IntoView {
    view! { <button type=b_type>{children()}</button> }
}
