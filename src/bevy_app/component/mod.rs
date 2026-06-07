// List of components

pub mod body;
pub mod sketch;
pub mod ui;

use bevy::ecs::component::Component;
use cad_base::id::GeometryId;
use ui_event::{ObjectType, SketchGeometryOperation};

/// Enum of types of object in CAD. This uses to pick, edit, and move.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct BodyPartType(pub ObjectType);

/// Type of gizmo of geometry
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct GeometryGizmo(pub GeometryId);

/// A component
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct RequestedGeometryOperation(pub SketchGeometryOperation);
