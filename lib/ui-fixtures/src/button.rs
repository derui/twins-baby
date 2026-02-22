use leptos::prelude::*;

use ui_component::button::ToolButton;
use ui_component::icon::{IconSize, IconType};

#[component]
fn DefaultButtonBehavior() -> impl IntoView {
    let (clicked, set_clicked) = signal("not clicked");

    let on_click = Callback::new(move |_| set_clicked.set("clicked"));

    view! {
        <div data-fixture="button-default">
            <span data-fixture="-clicked-label">{clicked}</span>
            <ToolButton icon=IconType::Axis(IconSize::Medium) label="Axis" on_click=on_click />
        </div>
    }
}

#[component]
pub fn ButtonFixtures() -> impl IntoView {
    view! {
        <DefaultButtonBehavior />

        <div data-fixture="button-with-axis">
            <ToolButton icon=IconType::Axis(IconSize::Medium) label="Axis" />
        </div>
        <div data-fixture="button-with-boolean-intersect">
            <ToolButton
                icon=IconType::BooleanIntersect(IconSize::Medium)
                label="BooleanIntersect"
            />
        </div>
        <div data-fixture="button-with-boolean-subtract">
            <ToolButton icon=IconType::BooleanSubtract(IconSize::Medium) label="BooleanSubtract" />
        </div>
        <div data-fixture="button-with-boolean-union">
            <ToolButton icon=IconType::BooleanUnion(IconSize::Medium) label="BooleanUnion" />
        </div>
        <div data-fixture="button-with-chamfer">
            <ToolButton icon=IconType::Chamfer(IconSize::Medium) label="Chamfer" />
        </div>
        <div data-fixture="button-with-cube">
            <ToolButton icon=IconType::Cube(IconSize::Medium) label="Cube" />
        </div>
        <div data-fixture="button-with-delete">
            <ToolButton icon=IconType::Delete(IconSize::Medium) label="Delete" />
        </div>
        <div data-fixture="button-with-dimension">
            <ToolButton icon=IconType::Dimension(IconSize::Medium) label="Dimension" />
        </div>
        <div data-fixture="button-with-duplicate">
            <ToolButton icon=IconType::Duplicate(IconSize::Medium) label="Duplicate" />
        </div>
        <div data-fixture="button-with-export">
            <ToolButton icon=IconType::Export(IconSize::Medium) label="Export" />
        </div>
        <div data-fixture="button-with-extrude">
            <ToolButton icon=IconType::Extrude(IconSize::Medium) label="Extrude" />
        </div>
        <div data-fixture="button-with-fillet">
            <ToolButton icon=IconType::Fillet(IconSize::Medium) label="Fillet" />
        </div>
        <div data-fixture="button-with-grid-snap">
            <ToolButton icon=IconType::GridSnap(IconSize::Medium) label="GridSnap" />
        </div>
        <div data-fixture="button-with-group">
            <ToolButton icon=IconType::Group(IconSize::Medium) label="Group" />
        </div>
        <div data-fixture="button-with-import">
            <ToolButton icon=IconType::Import(IconSize::Medium) label="Import" />
        </div>
        <div data-fixture="button-with-layers">
            <ToolButton icon=IconType::Layers(IconSize::Medium) label="Layers" />
        </div>
        <div data-fixture="button-with-mirror">
            <ToolButton icon=IconType::Mirror(IconSize::Medium) label="Mirror" />
        </div>
        <div data-fixture="button-with-move">
            <ToolButton icon=IconType::Move(IconSize::Medium) label="Move" />
        </div>
        <div data-fixture="button-with-orbit">
            <ToolButton icon=IconType::Orbit(IconSize::Medium) label="Orbit" />
        </div>
        <div data-fixture="button-with-redo">
            <ToolButton icon=IconType::Redo(IconSize::Medium) label="Redo" />
        </div>
        <div data-fixture="button-with-rotate">
            <ToolButton icon=IconType::Rotate(IconSize::Medium) label="Rotate" />
        </div>
        <div data-fixture="button-with-scale">
            <ToolButton icon=IconType::Scale(IconSize::Medium) label="Scale" />
        </div>
        <div data-fixture="button-with-section-cut">
            <ToolButton icon=IconType::SectionCut(IconSize::Medium) label="SectionCut" />
        </div>
        <div data-fixture="button-with-select">
            <ToolButton icon=IconType::Select(IconSize::Medium) label="Select" />
        </div>
        <div data-fixture="button-with-sketch">
            <ToolButton icon=IconType::Sketch(IconSize::Medium) label="Sketch" />
        </div>
        <div data-fixture="button-with-solid-view">
            <ToolButton icon=IconType::SolidView(IconSize::Medium) label="SolidView" />
        </div>
        <div data-fixture="button-with-undo">
            <ToolButton icon=IconType::Undo(IconSize::Medium) label="Undo" />
        </div>
        <div data-fixture="button-with-wireframe">
            <ToolButton icon=IconType::Wireframe(IconSize::Medium) label="Wireframe" />
        </div>
        <div data-fixture="button-with-zoom-fit">
            <ToolButton icon=IconType::ZoomFit(IconSize::Medium) label="ZoomFit" />
        </div>
    }
}
