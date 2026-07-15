use immutable::Im;

use crate::{body::Body, id::BodyId, plane::Plane};

/// A internal reference of BodyPlane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BodyPlane {
    X,
    Y,
    Z,
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
    pub fn with_x(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::X)
    }

    /// Create a new PlaneRef for the Y plane of the given body ID.
    pub fn with_y(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::Y)
    }

    /// Create a new PlaneRef for the Z plane of the given body ID.
    pub fn with_z(body_id: BodyId) -> Self {
        PlaneRef::new(body_id, BodyPlane::Z)
    }
}

/// A scope that holds a reference to a plane and its associated body.
pub struct PlaneScope<'a> {
    /// The body that the plane reference is associated with.
    pub body: &'a Body,

    original_ref: PlaneRef,
}

impl<'a> PlaneScope<'a> {
    /// Create a new PlaneScope with the given body and plane reference.
    pub fn new(body: &'a Body, plane_ref: PlaneRef) -> Self {
        PlaneScope {
            body,
            original_ref: plane_ref,
        }
    }

    /// Get the plane entity from the body
    pub fn to_plane(&self) -> Plane {
        self.original_ref.to_plane_from(self.body)
    }

    /// Get the plane entity from the body
    pub(crate) fn to_plane(&self) -> Plane {
        match self.plane {
            BodyPlane::X => (*self.body.x_plane).clone(),
            BodyPlane::Y => (*self.body.y_plane).clone(),
            BodyPlane::Z => (*self.body.z_plane).clone(),
        }
    }
}
