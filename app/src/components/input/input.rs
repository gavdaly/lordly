use leptos::*;

// In leptos should work as
// <Input<Text>>

// states should be 'valid', 'dirty', 'invalid'

#[component]
pub fn Input(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(into, optional)] summary: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
) -> impl IntoView {
    //<!-- Show error if there is one -->
    // <div data-type="error">""</div>
    view! {
        <div class="label">
            <label for=name.clone()>{label}</label>
            <input type="text" name id=name placeholder=placeholder />
        </div>
        <div class="summary">{summary}</div>
    }
}
