use std::collections::HashMap;

use immutable::Im;

use crate::{
    id::{EdgeId, FaceId, VertexId},
    solid::{edge::Edge, face::Face, vertex::Vertex},
};

pub mod edge;
pub mod face;
pub mod vertex;

/// The struct for a solid
pub struct Solid {
    /// Surfaces that constructs the solid. Each edges must be contained in the same solid
    pub faces: Im<HashMap<FaceId, Face>>,
    /// Edges that constructs the solid. All edges must be shared by 2 faces
    pub edges: Im<HashMap<EdgeId, Edge>>,
    /// Vertices that constructs the solid. All vertices must be shared by least 2 edges.
    pub vertices: Im<HashMap<VertexId, Vertex>>,

    _immutable: (),
}

pub struct SolidBuilder {
    faces: HashMap<FaceId, Face>,
    edges: HashMap<EdgeId, Edge>,
    vertices: HashMap<VertexId, Vertex>,
}

impl Default for SolidBuilder {
    fn default() -> Self {
        Self { faces: Default::default(), edges: Default::default(), vertices: Default::default() }
    }
}

impl SolidBuilder {
    pub fn add_edges(&mut self, edges: &[Edge]) -> &mut Self {
        

        self
    }
}
