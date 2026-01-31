use solver::equation::Equation;

use crate::{
    id::{GeometryId, VariableId},
    sketch::{Point2, scope::VariableScope},
};

pub trait Shape: std::fmt::Debug {
    /// Get the shape id of itself.
    ///
    /// When the shape does not have registered, this should be panic
    fn shape_id(&self) -> &GeometryId;
}

/// A trait for constraint definition by the shape.
pub trait Constraint {
    /// Resolve default constraints from
    fn default_constraints(&self, holder: &VariableScope) -> Vec<Equation>;
}

/// A basic structure of the sketch. This is representation of a straight line.
#[derive(Debug, Clone)]
pub struct Line {
    id: GeometryId,
    /// ID mapping for points.
    start_vars: (VariableId, VariableId),
    end_vars: (VariableId, VariableId),
}

impl Line {
    /// Make a new line with points.
    pub fn from_points(
        id: GeometryId,
        start: &Point2,
        end: &Point2,
        registrar: &mut VariableScope,
    ) -> Self {
        let start_ids = (registrar.register(start.x), registrar.register(start.y));
        let end_ids = (registrar.register(end.x), registrar.register(end.y));

        Line {
            id,
            start_vars: start_ids,
            end_vars: end_ids,
        }
    }
}

impl Shape for Line {
    fn shape_id(&self) -> &GeometryId {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub enum Basic {
    Line(Line),
}
