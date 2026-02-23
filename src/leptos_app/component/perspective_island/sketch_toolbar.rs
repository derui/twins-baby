use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_bevy_canvas::prelude::LeptosChannelMessageSender as _;
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};

use crate::{
    events::{SketchTool, SketchToolEvent},
    leptos_app::tool_command::ToolCommand,
};

/// Toolbar displayed when the perspective is set to Sketch.
#[component]
pub fn SketchToolbar() -> impl IntoView {
    let tool_command = use_context::<ToolCommand>().expect("ToolCommand context must be provided");

    let make_on_click = move |t: SketchTool| {
        let tool_command = tool_command.clone();
        move |_ev: leptos::web_sys::MouseEvent| {
            let _ = tool_command.0.send(SketchToolEvent { tool: t });
        }
    };

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
