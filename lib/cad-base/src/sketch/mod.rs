mod point2;
mod registrar;
mod shape;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use anyhow::Result;
use solver::{environment::Environment, variable::Variable};

use crate::{
    edge::Edge,
    id::{DefaultIdGenerator, EdgeId, GenerateId, GeometryId, PlaneId, PointId, SketchId},
    point::Point,
    sketch::shape::{Basic, Shape},
};

pub use point2::*;

/// The root data model of Sketch parsepective
#[derive(Debug, Clone)]
pub struct SketchPerspective {
    sketches: HashMap<SketchId, Sketch>,
    sketch_id_gen: Box<dyn GenerateId<SketchId>>,
    geometry_id_gen: Box<dyn GenerateId<GeometryId>>,
}

/// Builder of perspective
#[derive(Debug)]
pub struct SketchPerspectiveBuilder {
    pub sketch_id_gen: Box<dyn GenerateId<SketchId>>,
    pub geometry_id_gen: Box<dyn GenerateId<GeometryId>>,
}

impl Default for SketchPerspectiveBuilder {
    fn default() -> Self {
        Self {
            sketch_id_gen: Box::new(DefaultIdGenerator::default()),
            geometry_id_gen: Box::new(DefaultIdGenerator::default()),
        }
    }
}

impl SketchPerspectiveBuilder {
    /// Build a [SketchPerspective] with variables
    pub fn build(self) -> Result<SketchPerspective> {
        Ok(SketchPerspective {
            sketches: HashMap::new(),
            sketch_id_gen: self.sketch_id_gen,
            geometry_id_gen: self.geometry_id_gen,
        })
    }
}

impl SketchPerspective {
    /// Get a sketch reference of the id
    pub fn get(&self, id: &SketchId) -> Option<&Sketch> {
        self.sketches.get(id)
    }

    /// Get a sketch mutable reference of the id
    pub fn get_mut(&mut self, id: &SketchId) -> Option<&mut Sketch> {
        self.sketches.get_mut(id)
    }

    /// Add a new sketch to the perpective
    pub fn add_sketch<F>(&mut self, plane: &PlaneId) -> SketchId {
        let id = self.sketch_id_gen.generate();
        let sketch = Sketch::new(id, plane, self.geometry_id_gen.clone());

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
    geometory_id_gen: Box<dyn GenerateId<GeometryId>>,

    /// Geometries in this sketch
    geometries: HashMap<GeometryId, Box<Basic>>,

    /// variables for solver
    variables: Environment,

    /// A plane atteched to sketch
    attached_plane: PlaneId,
}

impl Sketch {
    /// Create a new sketch with builder
    ///
    /// ```rust
    /// Sketch::new(SketchId::new(1), SketchBuilder {
    ///   attached_plane: Some(PlaneId(3)),
    ///   ..Default::default()
    /// })
    /// ```
    fn new(
        id: SketchId,
        attached_plane: &PlaneId,
        id_gen: Box<dyn GenerateId<GeometryId>>,
    ) -> Self {
        Sketch {
            id,
            geometory_id_gen: id_gen,
            geometries: HashMap::new(),
            variables: Environment::empty(),
            attached_plane: *attached_plane,
        }
    }
}
