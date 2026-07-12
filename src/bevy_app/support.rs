use bevy::math::Vec3;
use cad_base::{body::BodyReader, point::Point, sketch::AttachableTarget, vector3::Vector3};

/// convenience support for converting Point to Vec3
pub trait Vec3Ext {
    fn to_vec3(&self) -> Vec3;
}

impl Vec3Ext for Point {
    /// Convert Point to Vec3
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: *self.x,
            y: *self.y,
            z: *self.z,
        }
    }
}

impl Vec3Ext for Vector3 {
    /// Convert Point to Vec3
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

pub trait PlaneExt {
    /// Get the normal vector of the plane as Vec3
    fn normal_vec3<T: BodyReader>(&self, reader: T) -> Vec3;
}

impl PlaneExt for AttachableTarget {
    fn normal_vec3<T: BodyReader>(&self, reader: T) -> Vec3 {
        match &self {
            &AttachableTarget::Plane(plane_ref) => {
                let body = reader
                    .read(*plane_ref.body_id)
                    .expect(format!("can not found body: {:?}", *plane_ref.body_id).as_str());
                let plane = plane_ref.to_plane_from(&body);
                plane.normal.to_vec3()
            }
            &AttachableTarget::Face(_) => {
                // TODO: derive normal from solid face
                Vec3::Z
            }
        }
    }
}
