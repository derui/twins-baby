use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::{
    events::{SketchTool, SketchToolEvent},
    leptos_app::tool_command::ToolCommand,
};

/// Toolbar displayed when the perspective is set to Sketch.
#[component]
pub fn SketchIsland() -> impl IntoView {
    let (active_tool, set_active_tool) = signal(SketchTool::default());

    let tool_command = use_context::<ToolCommand>().expect("ToolCommand context must be provided");

    let make_on_click = move |t: SketchTool| {
        let tool_command = tool_command.clone();
        move |_: leptos::web_sys::MouseEvent| {
            set_active_tool.set(t);
            let _ = tool_command.0.send(SketchToolEvent { tool: t });
        }
    };

    let btn_class = move |t: SketchTool| {
        move || {
            format!(
                "px-3 py-1 rounded text-sm font-medium transition-colors {}",
                if active_tool.get() == t {
                    "bg-blue-600 text-white"
                } else {
                    "bg-gray-700 text-gray-200 hover:bg-gray-600"
                }
            )
        }
    };

    view! {
        <div class="flex flex-row gap-2 p-2">
            <button class=btn_class(SketchTool::Select) on:click=make_on_click(SketchTool::Select)>
                "S"
            </button>
            <button class=btn_class(SketchTool::Line) on:click=make_on_click(SketchTool::Line)>
                "L"
            </button>
            <button
                class=btn_class(SketchTool::Circle)
                on:click=make_on_click(SketchTool::Circle)
            >
                "C"
            </button>
            <button
                class=btn_class(SketchTool::Rectangle)
                on:click=make_on_click(SketchTool::Rectangle)
            >
                "R"
            </button>
        </div>
    }
}
