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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        id::IdStore,
        plane::Plane,
        solid::{
            SolidBuilder,
            edge::Edge,
            face::{Face, PlanarSurface},
            vertex::Vertex,
        },
        point::Point,
    };

    fn v(x: f32, y: f32, z: f32) -> Vertex {
        Point::new(x, y, z).into()
    }

    fn make_face() -> Face {
        let mut store: IdStore<crate::id::EdgeId> = IdStore::of();
        let edge_ids: Vec<_> = (0..4).map(|_| store.generate()).collect();
        Face::Planar(PlanarSurface::new(&edge_ids, &Plane::new_xy()).unwrap())
    }

    #[test]
    fn add_vertices_returns_same_id_for_duplicate_vertex() {
        // Arrange
        let mut builder = SolidBuilder::default();

        // Act
        let ids1 = builder.add_vertices(&[v(1.0, 2.0, 3.0)]);
        let ids2 = builder.add_vertices(&[v(1.0, 2.0, 3.0)]);

        // Assert
        assert_eq!(ids1[0], ids2[0]);
    }

    #[test]
    fn add_vertices_returns_different_ids_for_different_vertices() {
        // Arrange
        let mut builder = SolidBuilder::default();

        // Act
        let ids = builder.add_vertices(&[v(1.0, 0.0, 0.0), v(2.0, 0.0, 0.0)]);

        // Assert
        assert_ne!(ids[0], ids[1]);
    }

    #[test]
    fn add_edges_returns_same_id_for_duplicate_edge() {
        // Arrange
        let mut builder = SolidBuilder::default();
        let vids = builder.add_vertices(&[v(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0)]);
        let edge = Edge::new(vids[0], vids[1]).expect("valid edge");

        // Act
        let ids1 = builder.add_edges(&[edge.clone()]);
        let ids2 = builder.add_edges(&[edge]);

        // Assert
        assert_eq!(ids1[0], ids2[0]);
    }

    #[test]
    fn add_edges_returns_different_ids_for_different_edges() {
        // Arrange
        let mut builder = SolidBuilder::default();
        let vids = builder.add_vertices(&[v(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0), v(2.0, 0.0, 0.0)]);
        let e1 = Edge::new(vids[0], vids[1]).unwrap();
        let e2 = Edge::new(vids[1], vids[2]).unwrap();

        // Act
        let ids = builder.add_edges(&[e1, e2]);

        // Assert
        assert_ne!(ids[0], ids[1]);
    }

    #[test]
    fn add_faces_always_creates_new_id() {
        // Arrange
        let mut builder = SolidBuilder::default();
        let face = make_face();

        // Act
        let ids1 = builder.add_faces(&[face.clone()]);
        let ids2 = builder.add_faces(&[face]);

        // Assert
        assert_ne!(ids1[0], ids2[0]);
    }

    #[test]
    fn build_creates_solid_with_correct_counts() {
        // Arrange
        let mut builder = SolidBuilder::default();
        let vids = builder.add_vertices(&[v(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0), v(2.0, 0.0, 0.0)]);
        builder.add_edges(&[Edge::new(vids[0], vids[1]).unwrap()]);
        builder.add_faces(&[make_face()]);

        // Act
        let solid = builder.build();

        // Assert
        assert_eq!(solid.vertices.len(), 3);
        assert_eq!(solid.edges.len(), 1);
        assert_eq!(solid.faces.len(), 1);
    }
}
