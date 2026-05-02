use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::button::IndicatorState;
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};

use crate::leptos_app::ui_action::BodyCreatedAction;
use crate::leptos_app::ui_state::UiStore;
use crate::leptos_app::use_action::{UseActionReturn, use_action};

/// Toolbar displayed when the perspective is set to Feature.
#[component]
pub fn FeatureToolbar() -> impl IntoView {
    let UseActionReturn { dispatch, .. } = use_action();
    let store = use_context::<UiStore>().expect("Should be defined before");

    let on_click_body = move |_ev: leptos::web_sys::MouseEvent| {
        dispatch(Box::new(BodyCreatedAction {
            name: "Body".to_string(),
        }))
    };

    let on_click_sketch = move |_ev: leptos::web_sys::MouseEvent| todo!();
    let sketch_indicator = Signal::derive(move || {
        if store.ui.body_perspective.can_create_sketch.get() {
            IndicatorState::On
        } else {
            IndicatorState::Disabled
        }
    });

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton icon=IconType::Cube(IconSize::Medium) label="Body" on:click=on_click_body />
            <ToolButton
                icon=IconType::Sketch(IconSize::Medium)
                indicator=sketch_indicator
                label="Sketch"
                on:click=on_click_sketch
            />
        </div>
    }
}
