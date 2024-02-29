use leptos::*;

#[component]
pub fn PickList(list: Vec<(String, String)>, label: String) -> impl IntoView {
    view! {
        <fieldset class="taglist">
            <legend>{label}</legend>

            {list
                .iter()
                .map(|(l, name)| {
                    view! {
                        <div class="tag-toggle">
                            <input type="radio" name id=name value=name/>
                            <label for=name>{l}</label>
                        </div>
                    }
                })
                .collect_view()}

        </fieldset>
    }
}

#[component]
pub fn TagList(list: Vec<(String, String)>, label: String) -> impl IntoView {
    view! {
        <fieldset class="taglist">
            <legend>{label}</legend>
            // Fix: Use tuple pattern (l, name)
            {list
                .iter()
                .map(|(l, name)| {
                    view! {
                        <div class="tag-toggle">
                            <input type="checkbox" name id=name value=name/>
                            <label for=name>{l}</label>
                        </div>
                    }
                })
                .collect_view()}
        </fieldset>
    }
}
