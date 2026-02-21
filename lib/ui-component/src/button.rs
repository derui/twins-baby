use leptos::{
    component,
    ev::MouseEvent,
    html::{self},
    prelude::*,
};

#[component]
pub fn Button(
    #[prop(optional)] _tabindex: Option<i32>,
    #[prop(optional)] _on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let node_ref = NodeRef::<html::Button>::new();

    Effect::new(move |_| if let Some(_node) = node_ref.get() {});

    view! {
        <button></button>
    }
}
