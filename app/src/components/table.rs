use leptos::*;

#[component]
pub fn Table(
    body: Vec<Vec<String>>,
    #[prop(optional)] headers: MaybeSignal<Vec<String>>,
    #[prop(optional)] footer: Option<Vec<String>>,
) -> impl IntoView {
    view! {
        <table>
            <Show when=move || {
                !headers().is_empty()
            }>
                "header is present"
            // {view!{ <thead>{headers().iter().map( |text|   view!{ <th>{text}</th>}).collect_view()}</thead>}}
            </Show>
            <tbody>
                {body
                    .iter()
                    .map(|row| {
                        view! { <tr>{row.iter().map(|cell| cell).collect_view()}</tr> }
                    })
                    .collect_view()}
            </tbody>
            <Show when=move || {
                footer.clone().is_some()
            }>
                "footer is present"
            // <tfoot>{footer.iter().map(|f| view! { <th>{f}</th> }).collect_view()}</tfoot>
            </Show>
        </table>
    }
}
