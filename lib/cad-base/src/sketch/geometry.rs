use immutable::Im;

use crate::{
    id::VariableId,
    sketch::{Point2, scope::VariableScope},
};

/// A basic structure of the sketch. This is representation of a line and points.
#[derive(Debug, Clone)]
pub struct LineSegment {
    /// ID mapping for points.
    pub start_points: Im<(VariableId, VariableId)>,
    pub end_points: Im<(VariableId, VariableId)>,
}

impl LineSegment {
    /// Make a new line with points.
    pub fn from_points(start: &Point2, end: &Point2, registrar: &mut VariableScope) -> Self {
        let start_ids = (registrar.register(*start.x), registrar.register(*start.y));
        let end_ids = (registrar.register(*end.x), registrar.register(*end.y));

        LineSegment {
            start_points: start_ids.into(),
            end_points: end_ids.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Geometry {
    LineSegment(LineSegment),
}
