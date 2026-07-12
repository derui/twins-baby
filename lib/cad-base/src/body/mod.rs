use std::collections::HashMap;

use color_eyre::eyre::{self, Result};
use immutable::Im;

use crate::{
    id::{BodyId, IdStore, SketchId},
    plane::Plane,
    vector3::Vector3,
};

#[cfg(test)]
mod tests;

/// A internal reference of BodyPlane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BodyPlane {
    X,
    Y,
    Z,
}

/// A trait for reading body information by its ID. Implementors of this trait should provide a method to retrieve a `Body` instance given its `BodyId`.
pub trait BodyReader {
    /// Read a body by its ID.
    fn read(&self, id: BodyId) -> Option<Body>;
}

/// A id-like reference of the plane. Plane is tightly coupled on the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneRef {
    pub body_id: Im<BodyId>,
    plane: BodyPlane,
}

impl PlaneRef {
    /// Create a new PlaneRef with the given body ID and plane.
    fn new(body_id: BodyId, plane: BodyPlane) -> Self {
        PlaneRef {
            body_id: body_id.into(),
            plane,
        }
    }

    /// Create a new PlaneRef for the X plane of the given body ID.
    pub fn new_with_x(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::X)
    }

    /// Create a new PlaneRef for the Y plane of the given body ID.
    pub fn new_with_y(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::Y)
    }

    /// Create a new PlaneRef for the Z plane of the given body ID.
    pub fn new_with_z(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::Z)
    }

    /// Get the plane entity from the body
    pub fn to_plane_from(&self, body: &Body) -> Plane {
        match self.plane {
            BodyPlane::X => (*body.x_plane).clone(),
            BodyPlane::Y => (*body.y_plane).clone(),
            BodyPlane::Z => (*body.z_plane).clone(),
        }
    }
}

#[derive(Clone)]
pub struct BodyPerspective {
    /// All bodies in application
    bodies: HashMap<BodyId, Body>,

    /// body id generator
    body_id_gen: IdStore<BodyId>,
}

impl Default for BodyPerspective {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyPerspective {
    /// Create a new perspective
    pub fn new() -> Self {
        BodyPerspective {
            bodies: HashMap::new(),
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

    /// Get the current size of bodies
    pub fn bodies(&self) -> impl Iterator<Item = &Body> {
        self.bodies.values()
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

    /// Rename the body. Return Ok with old string when succeeded.
    ///
    /// # Returns
    /// * Ok - when the name is not duplicated
    /// * Err - when the name is duplicated
    pub fn rename_body(&mut self, id: &BodyId, name: &str) -> Result<String> {
        if !self.bodies.contains_key(id) {
            return Err(eyre::eyre!("Do not found id : {:?}", id));
        }

        let names_other = self
            .bodies
            .iter()
            .filter_map(|(k, v)| if k == id { None } else { Some(v.name.clone()) })
            .collect::<Vec<_>>();

        if names_other.iter().all(|v| **v != name) {
            let old = self.bodies[id].name.clone();
            let body = self.bodies.get_mut(id).expect("Sholud be found");
            body.name = name.to_string().into();
            Ok((*old).clone())
        } else {
            Err(color_eyre::eyre::eyre!("The name duplicated : {}", name))
        }
    }

    /// Get X-plane reference for the body
    pub fn to_x_plane_ref(&self, id: &BodyId) -> Option<PlaneRef> {
        self.bodies
            .get(id)
            .map(|_| PlaneRef::new(*id, BodyPlane::X))
    }

    /// Get Y-plane reference for the body
    pub fn to_y_plane_ref(&self, id: &BodyId) -> Option<PlaneRef> {
        self.bodies
            .get(id)
            .map(|_| PlaneRef::new(*id, BodyPlane::Y))
    }

    /// Get Z-plane reference for the body
    pub fn to_z_plane_ref(&self, id: &BodyId) -> Option<PlaneRef> {
        self.bodies
            .get(id)
            .map(|_| PlaneRef::new(*id, BodyPlane::Z))
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

    /// Stkethes attached to a body
    sketches: Vec<SketchId>,
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
            sketches: Vec::new(),
        }
    }

    /// Add a sketch to the body if it is not already attached
    ///
    /// # Arguments
    /// * `sketch` - The ID of the sketch to be added to the body
    pub fn add_sketch(&mut self, sketch: &SketchId) {
        if self.sketches.contains(sketch) {
            return;
        }

        self.sketches.push(*sketch);
    }

    /// Remove a sketch from the body if it is attached, returning the removed sketch ID if successful
    ///
    /// # Arguments
    /// * `sketch` - The ID of the sketch to be removed from the body
    ///
    /// # Returns
    /// * `Some(SketchId)` - The ID of the removed sketch if it was attached to the body
    pub fn remove_sketch(&mut self, sketch: SketchId) -> Option<SketchId> {
        if let Some(position) = self.sketches.iter().position(|id| *id == sketch) {
            let id = self.sketches.remove(position);
            Some(id)
        } else {
            None
        }
    }

    /// Check if the body has any features (sketches) attached to it
    pub fn has_feature(&self) -> bool {
        !self.sketches.is_empty()
    }
}
