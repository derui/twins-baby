use leptos::prelude::*;

use ui_component::button::Button;
use ui_component::icon::{Icon, IconType};

#[component]
pub fn ButtonFixtures() -> impl IntoView {
    view! {
        <div data-fixture="button-default">
            <Button>{"button"}</Button>
        </div>
        <div data-fixture="button-with-axis">
            <Button icon=|| view! { <Icon typ=IconType::Axis /> }>{"Axis"}</Button>
        </div>
        <div data-fixture="button-with-boolean-intersect">
            <Button icon=|| {
                view! { <Icon typ=IconType::BooleanIntersect /> }
            }>{"BooleanIntersect"}</Button>
        </div>
        <div data-fixture="button-with-boolean-subtract">
            <Button icon=|| {
                view! { <Icon typ=IconType::BooleanSubtract /> }
            }>{"BooleanSubtract"}</Button>
        </div>
        <div data-fixture="button-with-boolean-union">
            <Button icon=|| view! { <Icon typ=IconType::BooleanUnion /> }>{"BooleanUnion"}</Button>
        </div>
        <div data-fixture="button-with-chamfer">
            <Button icon=|| view! { <Icon typ=IconType::Chamfer /> }>{"Chamfer"}</Button>
        </div>
        <div data-fixture="button-with-cube">
            <Button icon=|| view! { <Icon typ=IconType::Cube /> }>{"Cube"}</Button>
        </div>
        <div data-fixture="button-with-delete">
            <Button icon=|| view! { <Icon typ=IconType::Delete /> }>{"Delete"}</Button>
        </div>
        <div data-fixture="button-with-dimension">
            <Button icon=|| view! { <Icon typ=IconType::Dimension /> }>{"Dimension"}</Button>
        </div>
        <div data-fixture="button-with-duplicate">
            <Button icon=|| view! { <Icon typ=IconType::Duplicate /> }>{"Duplicate"}</Button>
        </div>
        <div data-fixture="button-with-export">
            <Button icon=|| view! { <Icon typ=IconType::Export /> }>{"Export"}</Button>
        </div>
        <div data-fixture="button-with-extrude">
            <Button icon=|| view! { <Icon typ=IconType::Extrude /> }>{"Extrude"}</Button>
        </div>
        <div data-fixture="button-with-fillet">
            <Button icon=|| view! { <Icon typ=IconType::Fillet /> }>{"Fillet"}</Button>
        </div>
        <div data-fixture="button-with-grid-snap">
            <Button icon=|| view! { <Icon typ=IconType::GridSnap /> }>{"GridSnap"}</Button>
        </div>
        <div data-fixture="button-with-group">
            <Button icon=|| view! { <Icon typ=IconType::Group /> }>{"Group"}</Button>
        </div>
        <div data-fixture="button-with-import">
            <Button icon=|| view! { <Icon typ=IconType::Import /> }>{"Import"}</Button>
        </div>
        <div data-fixture="button-with-layers">
            <Button icon=|| view! { <Icon typ=IconType::Layers /> }>{"Layers"}</Button>
        </div>
        <div data-fixture="button-with-mirror">
            <Button icon=|| view! { <Icon typ=IconType::Mirror /> }>{"Mirror"}</Button>
        </div>
        <div data-fixture="button-with-move">
            <Button icon=|| view! { <Icon typ=IconType::Move /> }>{"Move"}</Button>
        </div>
        <div data-fixture="button-with-orbit">
            <Button icon=|| view! { <Icon typ=IconType::Orbit /> }>{"Orbit"}</Button>
        </div>
        <div data-fixture="button-with-redo">
            <Button icon=|| view! { <Icon typ=IconType::Redo /> }>{"Redo"}</Button>
        </div>
        <div data-fixture="button-with-rotate">
            <Button icon=|| view! { <Icon typ=IconType::Rotate /> }>{"Rotate"}</Button>
        </div>
        <div data-fixture="button-with-scale">
            <Button icon=|| view! { <Icon typ=IconType::Scale /> }>{"Scale"}</Button>
        </div>
        <div data-fixture="button-with-section-cut">
            <Button icon=|| view! { <Icon typ=IconType::SectionCut /> }>{"SectionCut"}</Button>
        </div>
        <div data-fixture="button-with-select">
            <Button icon=|| view! { <Icon typ=IconType::Select /> }>{"Select"}</Button>
        </div>
        <div data-fixture="button-with-sketch">
            <Button icon=|| view! { <Icon typ=IconType::Sketch /> }>{"Sketch"}</Button>
        </div>
        <div data-fixture="button-with-solid-view">
            <Button icon=|| view! { <Icon typ=IconType::SolidView /> }>{"SolidView"}</Button>
        </div>
        <div data-fixture="button-with-undo">
            <Button icon=|| view! { <Icon typ=IconType::Undo /> }>{"Undo"}</Button>
        </div>
        <div data-fixture="button-with-wireframe">
            <Button icon=|| view! { <Icon typ=IconType::Wireframe /> }>{"Wireframe"}</Button>
        </div>
        <div data-fixture="button-with-zoom-fit">
            <Button icon=|| view! { <Icon typ=IconType::ZoomFit /> }>{"ZoomFit"}</Button>
        </div>
    }
}
