use leptos::*;

#[component]
pub fn Button(#[prop(into, optional)] b_type: Option<String>, children: Children) -> impl IntoView {
    let b_type = b_type.unwrap_or("button".to_string());
    view! { <button type=b_type>{children()}</button> }
}

enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl Into<String> for ButtonType {
    fn into(self) -> String {
        match self {
            ButtonType::Button => "button".to_string(),
            ButtonType::Submit => "submit".to_string(),
            ButtonType::Reset => "reset".to_string(),
        }
    }
}
