use leptos::*;

#[component]
pub fn Table(
    body: Vec<Vec<String>>,
    _headers: Option<Vec<String>>,
    _footer: Option<Vec<String>>,
) -> impl IntoView {
    view! {
        <table>
            // <Show when=headers.is_some()>
            // <thead>{headers.iter().map(|h| view! { <th>{h}</th> }).collect_view()}</thead>
            // </Show>
            <tbody>
                {body
                    .iter()
                    .map(|row| {
                        view! { <tr>{row.iter().map(|cell| cell).collect_view()}</tr> }
                    })
                    .collect_view()}
            </tbody>
        // <Show when=footer.is_some()>
        // <tfoot>{footer.iter().map(|f| view! { <th>{f}</th> }).collect_view()}</tfoot>
        // </Show>
        </table>
    }
}
