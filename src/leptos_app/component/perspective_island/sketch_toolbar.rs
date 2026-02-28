use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};
use ui_event::SketchTool;

/// Toolbar displayed when the perspective is set to Sketch.
#[component]
pub fn SketchToolbar() -> impl IntoView {
    let make_on_click = move |_t: SketchTool| move |_ev: leptos::web_sys::MouseEvent| {};

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton
                icon=IconType::SketchLine(IconSize::Medium)
                label="Line"
                on_click=Callback::new(make_on_click(SketchTool::Line))
            />
            <ToolButton
                icon=IconType::SketchCircle(IconSize::Medium)
                label="Circle"
                on_click=Callback::new(make_on_click(SketchTool::Circle))
            />
            <ToolButton
                icon=IconType::SketchRectangle(IconSize::Medium)
                label="Rectangle"
                on_click=Callback::new(make_on_click(SketchTool::Rectangle))
            />
        </div>
    }
}
