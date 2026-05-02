// List of components

use bevy::ecs::component::Component;
use cad_base::{
    body::PlaneRef,
    id::{EdgeId, FaceId},
};

/// Enum of types of object in CAD. This uses to pick, edit, and move.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum ObjectType {
    /// A plane. Plane is only on a body
    Plane(PlaneRef),

    /// A face. Face is in a feature
    Face(FaceId),

    /// An edge. Edge is in a feature
    Edge(EdgeId),

    /// A point.
    Point,

    /// An edge in the sketch. Only in sketch perspective.
    SketchEdge,

    /// A point in the sketch
    SketchPoint,
}
