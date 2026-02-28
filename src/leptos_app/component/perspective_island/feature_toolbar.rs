use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::{
    button::ToolButton,
    icon::{IconSize, IconType},
};
use ui_event::FeatureTool;

/// Toolbar displayed when the perspective is set to Feature.
#[component]
pub fn FeatureToolbar() -> impl IntoView {
    let make_on_click = move |_t: FeatureTool| move |_ev: leptos::web_sys::MouseEvent| {};

    view! {
        <div class="flex flex-row gap-2 p-2">
            <ToolButton
                icon=IconType::Cube(IconSize::Medium)
                label="Body"
                on_click=Callback::new(make_on_click(FeatureTool::Body))
            />
            <ToolButton
                icon=IconType::Sketch(IconSize::Medium)
                label="Sketch"
                on_click=Callback::new(make_on_click(FeatureTool::Sketch))
            />
        </div>
    }
}
