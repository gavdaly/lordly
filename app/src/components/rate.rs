use leptos::*;

#[component]
pub fn Rate<R, S>(rating: R, max: u8, set_rating: S) -> impl IntoView
where
    R: Fn() -> u8 + 'static,
    S: Fn(u8) + Clone + Copy + 'static,
{
    view! {
        <div class="rating">
            {(0..max)
                .into_iter()
                .map(|i| {
                    view! {
                        <i
                            class="rate"
                            data-selected=rating() <= i
                            on:click=move |_| { set_rating(i) }
                        ></i>
                    }
                })
                .collect_view()}
        </div>
    }
}
