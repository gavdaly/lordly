use leptos::*;

#[component]
pub fn Table(
    body: Vec<Vec<String>>,
    #[prop(optional)] headers: Option<Vec<String>>,
    #[prop(optional)] footer: Option<Vec<String>>,
) -> impl IntoView {
    view! {
        <table>
            <Show when=move || headers.clone().is_some()>
                "headers are present"
                // <thead>{headers.clone().into_iter().map(|h| move || view! { <th>"h"</th> }).collect_view()}</thead>
            </Show>
            <tbody>
                {body
                    .iter()
                    .map(|row| {
                        view! { <tr>{row.iter().map(|cell| cell).collect_view()}</tr> }
                    })
                    .collect_view()}
            </tbody>
        <Show when=move || footer.clone().is_some()>
            "footer is present"
        // <tfoot>{footer.iter().map(|f| view! { <th>{f}</th> }).collect_view()}</tfoot>
        </Show>
        </table>
    }
}
