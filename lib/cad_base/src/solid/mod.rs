use std::collections::HashMap;

use immutable::Im;

use crate::{id::{EdgeId, SurfaceId, VertexId}, solid::{edge::Edge, face::Face, vertex::Vertex}};

pub mod edge;
pub mod face;
pub mod vertex;

/// The struct for a solid
pub struct Solid {
    /// Surfaces that constructs the solid. Each edges
    pub surfaces: Im<HashMap<SurfaceId, Face>>,
    pub edges: Im<HashMap<EdgeId, Edge>>,
    pub vertices: Im<HashMap<VertexId, Vertex>>,
}
