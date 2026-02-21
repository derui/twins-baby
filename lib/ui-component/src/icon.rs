use leptos::{IntoView, component, prelude::ClassAttribute, view};

/// Definition of icon sizes
#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Small,
    Medium,
    Large,
}

impl IconSize {
    fn to_class(&self) -> String {
        match self {
            IconSize::Small => "w-4 h-4".to_string(),
            IconSize::Medium => "w-6 h-6".to_string(),
            IconSize::Large => "w-8 h-8".to_string(),
        }
    }
}

/// Definition of icons
#[derive(Debug, Clone, Copy)]
pub enum IconType {
    Axis,
    BooleanIntersect,
    BooleanSubtract,
    BooleanUnion,
    Chamfer,
    Cube,
    Delete,
    Dimension,
    Duplicate,
    Export,
    Extrude,
    Fillet,
    GridSnap,
    Group,
    Import,
    Layers,
    Mirror,
    Move,
    Orbit,
    Redo,
    Rotate,
    Scale,
    SectionCut,
    Select,
    Sketch,
    SolidView,
    Undo,
    Wireframe,
    ZoomFit,
}

impl IconType {
    fn to_class(&self) -> String {
        match self {
            IconType::Axis => "bg-[url('/assets/icons/axis.svg')]".to_string(),
            IconType::BooleanIntersect => {
                "bg-[url('/assets/icons/boolean-intersect.svg')]".to_string()
            }
            IconType::BooleanSubtract => {
                "bg-[url('/assets/icons/boolean-subtract.svg')]".to_string()
            }
            IconType::BooleanUnion => "bg-[url('/assets/icons/boolean-union.svg')]".to_string(),
            IconType::Chamfer => "bg-[url('/assets/icons/chamfer.svg')]".to_string(),
            IconType::Cube => "bg-[url('/assets/icons/cube.svg')]".to_string(),
            IconType::Delete => "bg-[url('/assets/icons/delete.svg')]".to_string(),
            IconType::Dimension => "bg-[url('/assets/icons/dimension.svg')]".to_string(),
            IconType::Duplicate => "bg-[url('/assets/icons/duplicate.svg')]".to_string(),
            IconType::Export => "bg-[url('/assets/icons/export.svg')]".to_string(),
            IconType::Extrude => "bg-[url('/assets/icons/extrude.svg')]".to_string(),
            IconType::Fillet => "bg-[url('/assets/icons/fillet.svg')]".to_string(),
            IconType::GridSnap => "bg-[url('/assets/icons/grid-snap.svg')]".to_string(),
            IconType::Group => "bg-[url('/assets/icons/group.svg')]".to_string(),
            IconType::Import => "bg-[url('/assets/icons/import.svg')]".to_string(),
            IconType::Layers => "bg-[url('/assets/icons/layers.svg')]".to_string(),
            IconType::Mirror => "bg-[url('/assets/icons/mirror.svg')]".to_string(),
            IconType::Move => "bg-[url('/assets/icons/move.svg')]".to_string(),
            IconType::Orbit => "bg-[url('/assets/icons/orbit.svg')]".to_string(),
            IconType::Redo => "bg-[url('/assets/icons/redo.svg')]".to_string(),
            IconType::Rotate => "bg-[url('/assets/icons/rotate.svg')]".to_string(),
            IconType::Scale => "bg-[url('/assets/icons/scale.svg')]".to_string(),
            IconType::SectionCut => "bg-[url('/assets/icons/section-cut.svg')]".to_string(),
            IconType::Select => "bg-[url('/assets/icons/select.svg')]".to_string(),
            IconType::Sketch => "bg-[url('/assets/icons/sketch.svg')]".to_string(),
            IconType::SolidView => "bg-[url('/assets/icons/solid-view.svg')]".to_string(),
            IconType::Undo => "bg-[url('/assets/icons/undo.svg')]".to_string(),
            IconType::Wireframe => "bg-[url('/assets/icons/wireframe.svg')]".to_string(),
            IconType::ZoomFit => "bg-[url('/assets/icons/zoom-fit.svg')]".to_string(),
        }
    }
}

#[component]
pub fn Icon(typ: IconType, #[prop(optional)] size: Option<IconSize>) -> impl IntoView {
    let size = size.unwrap_or(IconSize::Medium).to_class();
    let class = typ.to_class();
    let class = format!("{} bg-no-repeat bg-center {}", size, class);

    view! {
         <span class={class}></span>
    }
}
