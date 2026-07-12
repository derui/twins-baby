use immutable::Im;

use crate::{
    id::{FaceId, SolidId},
    plane::Plane,
    solid::{Solid, face::Face},
};

/// A id-like reference of a face in a solid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceRef {
    pub solid_id: Im<SolidId>,
    pub face_id: Im<FaceId>,
}

impl FaceRef {
    /// Create a new FaceRef with the given solid ID and face ID.
    pub fn new(solid_id: SolidId, face_id: FaceId) -> Self {
        FaceRef {
            solid_id: solid_id.into(),
            face_id: face_id.into(),
        }
    }

    /// Get the plane entity from the solid's face. Returns None if the face is missing.
    pub(crate) fn to_plane_from(&self, solid: &Solid) -> Option<Plane> {
        match solid.faces.get(&self.face_id)? {
            Face::Planar(surface) => Some((*surface.plane).clone()),
        }
    }
}
