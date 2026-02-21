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
            IconSize::Small => "w-4 h-4".to_string(),
            IconSize::Medium => "w-6 h-6".to_string(),
            IconSize::Large => "w-8 h-8".to_string(),
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
    pub(crate) fn to_class(&self) -> String {
        let (url_class, size) = match self {
            IconType::Axis(s) => ("bg-[url('/assets/icons/axis.svg')]", s),
            IconType::BooleanIntersect(s) => ("bg-[url('/assets/icons/boolean-intersect.svg')]", s),
            IconType::BooleanSubtract(s) => ("bg-[url('/assets/icons/boolean-subtract.svg')]", s),
            IconType::BooleanUnion(s) => ("bg-[url('/assets/icons/boolean-union.svg')]", s),
            IconType::Chamfer(s) => ("bg-[url('/assets/icons/chamfer.svg')]", s),
            IconType::Cube(s) => ("bg-[url('/assets/icons/cube.svg')]", s),
            IconType::Delete(s) => ("bg-[url('/assets/icons/delete.svg')]", s),
            IconType::Dimension(s) => ("bg-[url('/assets/icons/dimension.svg')]", s),
            IconType::Duplicate(s) => ("bg-[url('/assets/icons/duplicate.svg')]", s),
            IconType::Export(s) => ("bg-[url('/assets/icons/export.svg')]", s),
            IconType::Extrude(s) => ("bg-[url('/assets/icons/extrude.svg')]", s),
            IconType::Fillet(s) => ("bg-[url('/assets/icons/fillet.svg')]", s),
            IconType::GridSnap(s) => ("bg-[url('/assets/icons/grid-snap.svg')]", s),
            IconType::Group(s) => ("bg-[url('/assets/icons/group.svg')]", s),
            IconType::Import(s) => ("bg-[url('/assets/icons/import.svg')]", s),
            IconType::Layers(s) => ("bg-[url('/assets/icons/layers.svg')]", s),
            IconType::Mirror(s) => ("bg-[url('/assets/icons/mirror.svg')]", s),
            IconType::Move(s) => ("bg-[url('/assets/icons/move.svg')]", s),
            IconType::Orbit(s) => ("bg-[url('/assets/icons/orbit.svg')]", s),
            IconType::Redo(s) => ("bg-[url('/assets/icons/redo.svg')]", s),
            IconType::Rotate(s) => ("bg-[url('/assets/icons/rotate.svg')]", s),
            IconType::Scale(s) => ("bg-[url('/assets/icons/scale.svg')]", s),
            IconType::SectionCut(s) => ("bg-[url('/assets/icons/section-cut.svg')]", s),
            IconType::Select(s) => ("bg-[url('/assets/icons/select.svg')]", s),
            IconType::Sketch(s) => ("bg-[url('/assets/icons/sketch.svg')]", s),
            IconType::SolidView(s) => ("bg-[url('/assets/icons/solid-view.svg')]", s),
            IconType::Undo(s) => ("bg-[url('/assets/icons/undo.svg')]", s),
            IconType::Wireframe(s) => ("bg-[url('/assets/icons/wireframe.svg')]", s),
            IconType::ZoomFit(s) => ("bg-[url('/assets/icons/zoom-fit.svg')]", s),
        };
        format!(
            "inline-block shrink-0 bg-no-repeat bg-center {} {}",
            size.to_class(),
            url_class
        )
    }
}
