use std::collections::HashMap;

use immutable::Im;

use crate::{id::{BodyId, IdStore}, plane::Plane, vector3::Vector3};

#[cfg(test)]
mod tests;

pub struct BodyPerspective {
    /// All bodies in application
    bodies: HashMap<BodyId, Body>,

    /// body id generator
    body_id_gen: IdStore<BodyId>,
}

impl BodyPerspective {
    /// Create a new perspective
    pub fn new() -> Self {
        BodyPerspective {
            bodies: Default::default(),
            body_id_gen: IdStore::of(),
        }
    }

    /// Add a new body with a generated name and default planes at origin
    pub fn add_body(&mut self) -> BodyId {
        let id = self.body_id_gen.generate();
        let body = Body::new(format!("{}", id));
        self.bodies.insert(id, body);
        id
    }

    /// Get a reference to a body by id
    pub fn get(&self, id: &BodyId) -> Option<&Body> {
        self.bodies.get(id)
    }

    /// Get a mutable reference to a body by id
    pub fn get_mut(&mut self, id: &BodyId) -> Option<&mut Body> {
        self.bodies.get_mut(id)
    }

    /// Remove a body by id, returning it if it existed
    pub fn remove_body(&mut self, id: &BodyId) -> Option<Body> {
        self.bodies.remove(id)
    }
}

/// The body for the rendering target.
#[derive(Debug, Clone)]
pub struct Body {
    /// Name of body. It will generate automatically when it is created
    pub name: Im<String>,

    /// Body-local planes. Default is the same as world axis-based plane.
    pub x_plane: Im<Plane>,
    pub y_plane: Im<Plane>,
    pub z_plane: Im<Plane>,

    /// Position of Body.
    pub position: Im<Vector3>,
}

impl Body {
    /// Create a new body with default axis-aligned planes at origin
    pub fn new(name: String) -> Self {
        Body {
            name: Im::new(name),
            x_plane: Im::new(Plane::new_yz()),
            y_plane: Im::new(Plane::new_xz()),
            z_plane: Im::new(Plane::new_xy()),
            position: Im::new(Vector3::new(0.0, 0.0, 0.0)),
        }
    }
}
