use leptos::prelude::*;
use leptos::{IntoView, component, view};
use reactive_stores::Store;
use ui_component::button::IndicatorState;
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};

use crate::leptos_app::app_state::AppStore;
use crate::leptos_app::ui_action::{BodyCreatedAction, SketchCreatedAction};
use crate::leptos_app::ui_state::BodyPerspectiveUI;
use crate::leptos_app::use_action::{UseActionReturn, use_action};

/// Toolbar displayed when the perspective is set to Feature.
#[component]
pub fn FeatureToolbar() -> impl IntoView {
    let UseActionReturn { dispatch, .. } = use_action();
    let store = use_context::<Store<AppStore>>().expect("Should be defined before");

    let dispatch_event = dispatch.clone();
    let ui = BodyPerspectiveUI::from_store(store);
    let on_click_body =
        move |_ev: leptos::web_sys::MouseEvent| dispatch_event(Box::new(BodyCreatedAction {}));

    let on_click_sketch =
        move |_ev: leptos::web_sys::MouseEvent| dispatch(Box::new(SketchCreatedAction));

    let sketch_indicator = Signal::derive(move || {
        if ui.can_create_sketch.get() {
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
