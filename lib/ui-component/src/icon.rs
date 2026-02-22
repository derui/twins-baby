/// Definition of icon sizes
#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Small,
    Medium,
    Large,
}

impl IconSize {
    pub(crate) fn to_class(&self) -> String {
        match self {
            IconSize::Small => "w-9 h-9".to_string(),
            IconSize::Medium => "w-12 h-12".to_string(),
            IconSize::Large => "w-16 h-16".to_string(),
        }
    }
}

/// Definition of icons with their size
#[derive(Debug, Clone, Copy)]
pub enum IconType {
    Axis(IconSize),
    BooleanIntersect(IconSize),
    BooleanSubtract(IconSize),
    BooleanUnion(IconSize),
    Chamfer(IconSize),
    Cube(IconSize),
    Delete(IconSize),
    Dimension(IconSize),
    Duplicate(IconSize),
    Export(IconSize),
    Extrude(IconSize),
    Fillet(IconSize),
    GridSnap(IconSize),
    Group(IconSize),
    Import(IconSize),
    Layers(IconSize),
    Mirror(IconSize),
    Move(IconSize),
    Orbit(IconSize),
    Redo(IconSize),
    Rotate(IconSize),
    Scale(IconSize),
    SectionCut(IconSize),
    Select(IconSize),
    Sketch(IconSize),
    SolidView(IconSize),
    Undo(IconSize),
    Wireframe(IconSize),
    ZoomFit(IconSize),
}

impl IconType {
    pub(crate) fn to_url(&self) -> &'static str {
        match self {
            IconType::Axis(_) => "/assets/icons/axis.svg",
            IconType::BooleanIntersect(_) => "/assets/icons/boolean-intersect.svg",
            IconType::BooleanSubtract(_) => "/assets/icons/boolean-subtract.svg",
            IconType::BooleanUnion(_) => "/assets/icons/boolean-union.svg",
            IconType::Chamfer(_) => "/assets/icons/chamfer.svg",
            IconType::Cube(_) => "/assets/icons/cube.svg",
            IconType::Delete(_) => "/assets/icons/delete.svg",
            IconType::Dimension(_) => "/assets/icons/dimension.svg",
            IconType::Duplicate(_) => "/assets/icons/duplicate.svg",
            IconType::Export(_) => "/assets/icons/export.svg",
            IconType::Extrude(_) => "/assets/icons/extrude.svg",
            IconType::Fillet(_) => "/assets/icons/fillet.svg",
            IconType::GridSnap(_) => "/assets/icons/grid-snap.svg",
            IconType::Group(_) => "/assets/icons/group.svg",
            IconType::Import(_) => "/assets/icons/import.svg",
            IconType::Layers(_) => "/assets/icons/layers.svg",
            IconType::Mirror(_) => "/assets/icons/mirror.svg",
            IconType::Move(_) => "/assets/icons/move.svg",
            IconType::Orbit(_) => "/assets/icons/orbit.svg",
            IconType::Redo(_) => "/assets/icons/redo.svg",
            IconType::Rotate(_) => "/assets/icons/rotate.svg",
            IconType::Scale(_) => "/assets/icons/scale.svg",
            IconType::SectionCut(_) => "/assets/icons/section-cut.svg",
            IconType::Select(_) => "/assets/icons/select.svg",
            IconType::Sketch(_) => "/assets/icons/sketch.svg",
            IconType::SolidView(_) => "/assets/icons/solid-view.svg",
            IconType::Undo(_) => "/assets/icons/undo.svg",
            IconType::Wireframe(_) => "/assets/icons/wireframe.svg",
            IconType::ZoomFit(_) => "/assets/icons/zoom-fit.svg",
        }
    }

    pub(crate) fn size_class(&self) -> String {
        let size = match self {
            IconType::Axis(s)
            | IconType::BooleanIntersect(s)
            | IconType::BooleanSubtract(s)
            | IconType::BooleanUnion(s)
            | IconType::Chamfer(s)
            | IconType::Cube(s)
            | IconType::Delete(s)
            | IconType::Dimension(s)
            | IconType::Duplicate(s)
            | IconType::Export(s)
            | IconType::Extrude(s)
            | IconType::Fillet(s)
            | IconType::GridSnap(s)
            | IconType::Group(s)
            | IconType::Import(s)
            | IconType::Layers(s)
            | IconType::Mirror(s)
            | IconType::Move(s)
            | IconType::Orbit(s)
            | IconType::Redo(s)
            | IconType::Rotate(s)
            | IconType::Scale(s)
            | IconType::SectionCut(s)
            | IconType::Select(s)
            | IconType::Sketch(s)
            | IconType::SolidView(s)
            | IconType::Undo(s)
            | IconType::Wireframe(s)
            | IconType::ZoomFit(s) => s,
        };
        format!("inline-block shrink-0 {}", size.to_class())
    }
}
