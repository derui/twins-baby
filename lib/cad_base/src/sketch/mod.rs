#[cfg(test)]
mod tests;

mod constraint;
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
    id::{FaceId, GeometryId, IdStore, PlaneId},
    sketch::scope::{ConstraintScope, VariableScope},
};

use color_eyre::eyre::{Result, eyre};
use immutable::Im;

/// Target of sketch attachiment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachableTarget {
    /// attaching to a plane. such as base plane of the body
    Plane(PlaneId),
    /// atthching to a face, in some solid.
    Face(FaceId),
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

    geometory_id_gen: IdStore<GeometryId>,

    /// Geometries in this sketch
    geometries: HashMap<GeometryId, Box<Geometry>>,

    /// variable scope.
    variables: VariableScope,

    /// Constraint scope
    constraints: ConstraintScope,

    /// A plane atteched to sketch
    attach_target: AttachableTarget,
}

impl Sketch {
    /// Create a new sketch with builder
    pub fn new(name: &str, attach_target: &AttachableTarget) -> Self {
        Sketch {
            name: name.to_string().into(),
            geometory_id_gen: IdStore::of(),
            geometries: HashMap::new(),
            variables: VariableScope::new(),
            constraints: ConstraintScope::new(),
            attach_target: attach_target.clone(),
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
        F: FnOnce(&mut VariableScope) -> Geometry,
    {
        let geometry = maker(&mut self.variables);

        let id = self.geometory_id_gen.generate();
        self.geometries.insert(id, Box::new(geometry));
        id
    }

    /// Remove a geometry from this sketch
    pub fn remove_geometry(&mut self, id: &GeometryId) -> Option<Box<Geometry>> {
        self.geometries.remove(id)
    }
}
