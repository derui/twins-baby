use bevy::math::Vec3;
use cad_base::{point::Point, vector3::Vector3};

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
