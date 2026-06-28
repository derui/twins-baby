use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};
use ui_event::SketchGeometryOperation;

use crate::leptos_app::ui_action::SketchGeometryRequestedAction;
use crate::leptos_app::use_action::{UseActionReturn, use_action};

/// Toolbar displayed when the perspective is set to Sketch.
#[component]
pub fn SketchToolbar() -> impl IntoView {
    let UseActionReturn { dispatch, .. } = use_action();

    let dispatch_line = dispatch.clone();
    let line_on_click = move |_ev: leptos::web_sys::MouseEvent| {
        dispatch_line(
            SketchGeometryRequestedAction {
                operation: SketchGeometryOperation::LineSegment,
            }
            .into(),
        );
    };

    let circle_on_click = move |_ev: leptos::web_sys::MouseEvent| {
        // No operation for Circle
    };

    let dispatch_rectangle = dispatch.clone();
    let rect_on_click = move |_ev: leptos::web_sys::MouseEvent| {
        dispatch_rectangle(
            SketchGeometryRequestedAction {
                operation: SketchGeometryOperation::Rectangle,
            }
            .into(),
        );
    };

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton
                icon=IconType::SketchLine(IconSize::Medium)
                label="Line"
                on_click=Callback::new(line_on_click)
            />
            <ToolButton
                icon=IconType::SketchCircle(IconSize::Medium)
                label="Circle"
                on_click=Callback::new(circle_on_click)
            />
            <ToolButton
                icon=IconType::SketchRectangle(IconSize::Medium)
                label="Rectangle"
                on_click=Callback::new(rect_on_click)
            />
        </div>
    }
}
