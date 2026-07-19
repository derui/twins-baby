#[cfg(test)]
mod tests;

mod constraint;
pub mod edge;
mod geometry;
mod perspective;
mod point2;
mod scope;

pub use constraint::*;
pub use geometry::*;
pub use perspective::*;
pub use point2::*;
use tracing::instrument;

use std::collections::HashMap;

use crate::{
    id::{BodyId, GeometryId, IdStore, VariableId},
    plane::Plane,
    refs::{FaceRef, PlaneRef, PlaneScope, Resolve},
    sketch::{
        edge::SketchEdge,
        scope::{ConstraintArena, VariableArena},
    },
};

use color_eyre::eyre::{Result, eyre};
use immutable::Im;

/// Target of sketch attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachableTarget {
    /// attaching to a plane. such as base plane of the body
    Plane(PlaneRef),
    /// atthching to a face, in some solid.
    Face(FaceRef),
}

impl AttachableTarget {
    /// Get plane ref if this target is plane.
    pub fn to_plane_ref(&self) -> Option<PlaneRef> {
        match self {
            AttachableTarget::Plane(plane_ref) => Some(*plane_ref),
            _ => None,
        }
    }

    /// Get face ref if this target is face.
    pub fn to_face_ref(&self) -> Option<FaceRef> {
        match self {
            AttachableTarget::Face(face_ref) => Some(face_ref.clone()),
            _ => None,
        }
    }

    /// Make plane from target.
    pub fn to_plane<'a, T: Resolve<'a, PlaneRef, PlaneScope<'a>>>(
        &self,
        reader: &'a T,
    ) -> Option<Plane> {
        match self {
            AttachableTarget::Plane(plane_ref) => reader.resolve(*plane_ref).map(|s| s.to_plane()),
            AttachableTarget::Face(_face_ref) => todo!(),
        }
    }
}

/// The sketch of base of modeling.
///
/// [Sketch] has these values:
///
/// - geometries defined as some basic geometres
/// - attached Plane with plane id.
/// - constraints equations for points (not implemented yet)
///
#[derive(Debug, Clone)]
pub struct Sketch {
    /// Name of this sketch
    pub name: Im<String>,

    /// The body that this sketch belongs to
    pub body: Im<BodyId>,

    geometory_id_gen: IdStore,

    /// Geometries in this sketch
    geometries: HashMap<GeometryId, Geometry>,

    /// variable scope.
    variables: VariableArena,

    /// Constraint scope
    constraints: ConstraintArena,

    /// A plane atteched to sketch
    pub attach_target: Im<AttachableTarget>,
}

impl Sketch {
    /// Create a new sketch with builder
    pub fn new(name: &str, body: BodyId, attach_target: &AttachableTarget) -> Self {
        Sketch {
            body: body.into(),
            name: name.to_string().into(),
            geometory_id_gen: IdStore::of(),
            geometries: HashMap::new(),
            variables: VariableArena::new(),
            constraints: ConstraintArena::new(),
            attach_target: attach_target.clone().into(),
        }
    }

    /// Set name for the sketch.
    ///
    /// # Errors
    /// Returns error when the new name is empty string.
    #[instrument(err)]
    fn set_name(&mut self, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(eyre!("Do not allow empty string"));
        }

        self.name = new_name.trim().to_string().into();

        Ok(())
    }

    /// Add a geometry to this sketch with a geometry maker function
    pub fn add_geometry<F>(&mut self, maker: F) -> GeometryId
    where
        F: FnOnce(&mut VariableArena) -> Geometry,
    {
        let geometry = maker(&mut self.variables);

        let id = self.geometory_id_gen.generate();
        self.geometries.insert(id, geometry);
        id
    }

    /// Remove a geometry from this sketch
    pub fn remove_geometry(&mut self, id: &GeometryId) -> Option<Geometry> {
        self.geometries.remove(id)
    }

    /// Get a point2 from raw point.
    #[tracing::instrument(err)]
    fn resolve_point(&self, raw_point: &Im<(VariableId, VariableId)>) -> Result<Point2> {
        let Some(start) = self.variables.get(&raw_point.0) else {
            return Err(eyre!("Do not found variable for {}", &raw_point.0));
        };

        let Some(end) = self.variables.get(&raw_point.1) else {
            return Err(eyre!("Do not found variable for {}", &raw_point.1));
        };

        Ok(Point2::new(start.into(), end.into()))
    }

    /// Get all [SketchEdge] as concreted value.
    #[tracing::instrument(err)]
    pub fn resolve_edges(&self) -> Result<Vec<SketchEdge>> {
        let mut ret = Vec::new();

        for geometry in self.geometries.values() {
            match geometry {
                Geometry::LineSegment(line_segment) => {
                    let start = self.resolve_point(&line_segment.start_points)?;
                    let end = self.resolve_point(&line_segment.end_points)?;

                    ret.push(SketchEdge::new(&start, &end));
                }
            }
        }

        Ok(ret)
    }
}
