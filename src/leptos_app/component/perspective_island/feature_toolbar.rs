use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};
use ui_event::FeatureTool;
use ui_event::command::CreateBodyCommand;

use crate::leptos_app::command_sender::CommandSender;

/// Toolbar displayed when the perspective is set to Feature.
#[component]
pub fn FeatureToolbar() -> impl IntoView {
    let sender = use_context::<CommandSender>().expect("should be set before");
    let (event, set_event) = signal(None::<FeatureTool>);

    let on_click_body = move |_ev: leptos::web_sys::MouseEvent| {
        sender.send(|id| {
            CreateBodyCommand {
                id: id.into(),
                name: "Body".to_string().into(),
            }
            .into()
        })
    };

    let on_click_sketch = move |_ev: leptos::web_sys::MouseEvent| todo!();

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton
                icon=IconType::Cube(IconSize::Medium)
                label="Body"
                on:click=move |ev| on_click_body(ev)
            />
            <ToolButton
                icon=IconType::Sketch(IconSize::Medium)
                label="Sketch"
                on:click=move |ev| on_click_sketch(ev)
            />
        </div>
    }
}
