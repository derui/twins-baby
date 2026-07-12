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
    pub(crate) fn to_plane_from(&self, body: &Body) -> Plane {
        match self.plane {
            BodyPlane::X => (*body.x_plane).clone(),
            BodyPlane::Y => (*body.y_plane).clone(),
            BodyPlane::Z => (*body.z_plane).clone(),
        }
    }
}
