use leptos::{html::Dialog, prelude::*};

#[component]
pub fn Dia(children: Children, open: ReadSignal<bool>) -> impl IntoView {
    let d: NodeRef<Dialog> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(dialog) = d.get() {
            if open.get() {
                let _ = dialog.show_modal();
            } else {
                let _ = dialog.close();
            }
        }
    });

    view! { <dialog node_ref=d>{children()}</dialog> }
}
