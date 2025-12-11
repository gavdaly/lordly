use alloc::string::ToString;

use alloc::format;



use alloc::string::String;

use crate::data_type::Color;
use leptos::prelude::*;

/// A Progress Bar component for showing completion status
/// - `value`: Current progress value (0-100)
/// - `color`: The color scheme of the progress bar
/// - `striped`: Whether to show striped pattern
/// - `animated`: Whether to animate the progress bar
/// - `label`: Optional text label to show inside the progress bar
#[component]
pub fn ProgressBar(
    #[prop(into)] value: Signal<f64>,
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(optional)] striped: bool,
    #[prop(optional)] animated: bool,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    // Ensure value stays between 0-100
    let bounded_value = Signal::derive(move || {
        let val = value.get();
        val.max(0.0).min(100.0)
    });

    view! {
        <div class="progress">
            // aria_role="progressbar"
            // aria-valuemin="0"
            // aria-valuemax="100"
            // aria-valuenow={move || bounded_value.get().to_string()}
            <div
                class="progress-bar"
                data-color=color
                data-striped=striped.to_string()
                data-animated=animated.to_string()
                style:width=move || format!("{:.1}%", bounded_value.get())
            >
                {move || label.clone().map(|l| view! { <span class="progress-label">{l}</span> })}
            </div>
        </div>
    }
}

/// A circular progress/spinner component
/// - `value`: Current progress value (0-100), or None for indeterminate spinner
/// - `color`: The color scheme of the spinner
/// - `size`: Size in pixels
#[component]
pub fn Spinner(
    #[prop(into, optional)] value: Option<Signal<f64>>,
    #[prop(default={Color::Primary}, into)] color: Color,
    #[prop(optional, default = 32)] size: u32,
) -> impl IntoView {
    // For determinate progress, ensure value stays between 0-100
    let bounded_value = move || {
        value.map(|v| {
            let val = v.get();
            val.max(0.0).min(100.0)
        })
    };

    // Calculate SVG parameters for circular progress
    let radius = 42;
    let circumference = 2.0 * core::f64::consts::PI * radius as f64;

    let stroke_dashoffset = move || {
        if let Some(val) = bounded_value() {
            let percent_complete = val / 100.0;
            circumference * (1.0 - percent_complete)
        } else {
            0.0 // For indeterminate state
        }
    };

    view! {
        <svg
            class="spinner"
            width=size.to_string()
            height=size.to_string()
            viewBox="0 0 100 100"
            data-color=color
            data-indeterminate=bounded_value().is_none().to_string()
        >
            <circle
                class="spinner-track"
                cx="50"
                cy="50"
                r=radius.to_string()
                fill="none"
                stroke-width="8"
            />
            <circle
                class="spinner-progress"
                cx="50"
                cy="50"
                r=radius.to_string()
                fill="none"
                stroke-width="8"
                stroke-dasharray=circumference.to_string()
                stroke-dashoffset=move || stroke_dashoffset().to_string()
                transform="rotate(-90 50 50)"
            />
        </svg>
    }
}
