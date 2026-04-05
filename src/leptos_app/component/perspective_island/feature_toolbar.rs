use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};

use crate::leptos_app::ui_action::BodyCreatedAction;
use crate::leptos_app::use_action::{UseActionReturn, use_action};

/// Toolbar displayed when the perspective is set to Feature.
#[component]
pub fn FeatureToolbar() -> impl IntoView {
    let UseActionReturn { dispatch, .. } = use_action();

    let on_click_body = move |_ev: leptos::web_sys::MouseEvent| {
        dispatch(Box::new(BodyCreatedAction {
            name: "Body".to_string(),
        }))
    };

    let on_click_sketch = move |_ev: leptos::web_sys::MouseEvent| todo!();

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton icon=IconType::Cube(IconSize::Medium) label="Body" on:click=on_click_body />
            <ToolButton
                icon=IconType::Sketch(IconSize::Medium)
                label="Sketch"
                on:click=on_click_sketch
            />
        </div>
    }
}
