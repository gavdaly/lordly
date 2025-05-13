use leptos::prelude::*;

/// A rating component.
/// # Arguments
/// - `max`: The maximum rating.
/// - `name`: The name of the input.
#[component]
pub fn Rate(#[prop(default = 5)] max: u8, #[prop(into)] name: String) -> impl IntoView {
    let (rating, set_rating) = signal(0);
    view! {
        <div class="rating">
            <input
                aria-label="rating"
                class="visually-hidden"
                type="number"
                max=max
                name=name
                value=rating.get()
            />
            {(0..max)
                .map(|i| {
                    view! {
                        <i
                            class="rate"
                            data-selected=rating.get() <= i
                            on:click=move |_| { set_rating.set(i) }
                        >
                            "⭐️"
                        </i>
                    }
                })
                .collect_view()}
        </div>
    }
}
