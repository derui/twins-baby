use std::collections::HashMap;

use immutable::Im;

use crate::{
    id::{EdgeId, FaceId, IdStore, VertexId},
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

    edge_id_gen: IdStore<EdgeId>,
    vertex_id_gen: IdStore<VertexId>,
    face_id_gen: IdStore<FaceId>,
}

impl Default for SolidBuilder {
    fn default() -> Self {
        Self {
            faces: Default::default(),
            edges: Default::default(),
            vertices: Default::default(),
            edge_id_gen: IdStore::of(),
            vertex_id_gen: IdStore::of(),
            face_id_gen: IdStore::of(),
        }
    }
}

impl SolidBuilder {
    /// Add edges to builder. If [edges] contained same vertex set, it has same id of the edge.
    pub fn add_edges(&mut self, edges: &[Edge]) -> Vec<EdgeId> {
        let mut result = Vec::new();
        for edge in edges {
            if let Some((id, _)) = self.edges.iter().find(|(_, e)| **e == *edge) {
                result.push(*id);
                continue;
            }

            let id = self.edge_id_gen.generate();
            self.edges.insert(id, edge.clone());
            result.push(id);
        }

        result
    }

    /// Add vertices to builder. If [vertices] contained same coordinate, it has same id of the vertex.
    pub fn add_vertices(&mut self, vertices: &[Vertex]) -> Vec<VertexId> {
        let mut result = Vec::new();
        for vertex in vertices {
            if let Some((id, _)) = self.vertices.iter().find(|(_, v)| **v == *vertex) {
                result.push(*id);
                continue;
            }

            let id = self.vertex_id_gen.generate();
            self.vertices.insert(id, vertex.clone());
            result.push(id);
        }

        result
    }

    /// Add faces to builder.
    pub fn add_faces(&mut self, faces: &[Face]) -> Vec<FaceId> {
        let mut result = Vec::new();
        for face in faces {
            let id = self.face_id_gen.generate();
            self.faces.insert(id, face.clone());
            result.push(id);
        }

        result
    }

    /// Build solid. Builder can not reuse.
    pub fn build(self) -> Solid {
        Solid {
            faces: (self.faces).into(),
            edges: (self.edges).into(),
            vertices: (self.vertices).into(),
            _immutable: (),
        }
    }
}
