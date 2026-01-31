use enum_dispatch::enum_dispatch;
use immutable::Im;
use solver::equation::Equation;

use crate::{
    id::{GeometryId, VariableId},
    sketch::{Point2, scope::VariableScope},
};

pub trait Geometry: std::fmt::Debug {
    /// Get the geometry id of itself.
    ///
    /// When the geometry does not have registered, this should be panic
    fn geometry_id(&self) -> &GeometryId;
}

/// A trait for constraint definition by the shape.
pub trait Constraint {
    /// Resolve default constraints from
    fn default_constraints(&self, holder: &VariableScope) -> Vec<Equation>;
}

/// A basic structure of the sketch. This is representation of a line and points.
#[derive(Debug, Clone)]
pub struct LineSegment {
    id: GeometryId,
    /// ID mapping for points.
    pub start_points: Im<(VariableId, VariableId)>,
    pub end_points: Im<(VariableId, VariableId)>,
}

impl LineSegment {
    /// Make a new line with points.
    pub fn from_points(
        id: GeometryId,
        start: &Point2,
        end: &Point2,
        registrar: &mut VariableScope,
    ) -> Self {
        let start_ids = (registrar.register(start.x), registrar.register(start.y));
        let end_ids = (registrar.register(end.x), registrar.register(end.y));

        LineSegment {
            id,
            start_points: start_ids.into(),
            end_points: end_ids.into(),
        }
    }
}

impl Geometry for LineSegment {
    fn geometry_id(&self) -> &GeometryId {
        &self.id
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch(Geometry)]
pub enum Basic {
    LineSegment(LineSegment),
}
