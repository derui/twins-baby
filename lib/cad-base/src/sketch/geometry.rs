use immutable::Im;

use crate::sketch::{
    Point2,
    scope::{VariableArena, VariableIndex},
};

/// A basic structure of the sketch. This is representation of a line and points.
#[derive(Debug, Clone)]
pub struct LineSegment {
    /// ID mapping for points.
    pub start_points: Im<(VariableIndex, VariableIndex)>,
    pub end_points: Im<(VariableIndex, VariableIndex)>,
}

impl LineSegment {
    /// Make a new line with points.
    pub fn from_points(start: &Point2, end: &Point2, registrar: &mut VariableArena) -> Self {
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
