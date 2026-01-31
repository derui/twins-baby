#[cfg(test)]
mod tests;

mod constraint;
mod geometry;
pub mod point2;
pub mod scope;

use std::collections::HashMap;

use crate::{
    id::{GeometryId, IdStore, PlaneId, SketchId},
    sketch::{
        geometry::Geometry,
        scope::{ConstraintScope, VariableScope},
    },
};

pub use point2::*;

/// The root data model of Sketch parsepective
#[derive(Debug, Clone)]
pub struct SketchPerspective {
    sketches: HashMap<SketchId, Sketch>,
    sketch_id_gen: IdStore<SketchId>,
}

impl SketchPerspective {
    pub fn new() -> Self {
        SketchPerspective {
            sketches: HashMap::new(),
            sketch_id_gen: IdStore::of(),
        }
    }

    /// Get a sketch reference of the id
    pub fn get(&self, id: &SketchId) -> Option<&Sketch> {
        self.sketches.get(id)
    }

    /// Get a sketch mutable reference of the id
    pub fn get_mut(&mut self, id: &SketchId) -> Option<&mut Sketch> {
        self.sketches.get_mut(id)
    }

    /// Add a new sketch to the perpective
    pub fn add_sketch(&mut self, plane: &PlaneId) -> SketchId {
        let id = self.sketch_id_gen.generate();
        let sketch = Sketch::new(id, plane);

        self.sketches.insert(id, sketch);
        id
    }

    /// Remove sketch from perspective
    pub fn remove_sketch(&mut self, id: &SketchId) -> Option<Sketch> {
        self.sketches.remove(id)
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
    id: SketchId,

    geometory_id_gen: IdStore<GeometryId>,

    /// Geometries in this sketch
    geometries: HashMap<GeometryId, Box<Geometry>>,

    /// variable scope.
    variables: VariableScope,

    /// Constraint scope
    constraints: ConstraintScope,

    /// A plane atteched to sketch
    attached_plane: PlaneId,
}

impl Sketch {
    /// Create a new sketch with builder
    fn new(id: SketchId, attached_plane: &PlaneId) -> Self {
        Sketch {
            id,
            geometory_id_gen: IdStore::of(),
            geometries: HashMap::new(),
            variables: VariableScope::new(),
            constraints: ConstraintScope::new(),
            attached_plane: *attached_plane,
        }
    }
}
