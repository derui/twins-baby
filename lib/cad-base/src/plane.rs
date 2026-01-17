use anyhow::{self, Result};

use crate::{edge::Edge, point::Point, vector3d::Vector3d};

/// Simple plane definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    /// normal vector of the vector
    normal: Vector3d,

    /// point on the plane
    r0: Point,
}

impl Plane {
    /// Create a new plane that makes 2 edges and crossed the 2 edges.
    pub fn new(edge1: &Edge, edge2: &Edge) -> Result<Self> {
        let v1 = Vector3d::from_edge(edge1);
        let v2 = Vector3d::from_edge(edge2);

        let crossed = v1.cross(&v2);

        // If crossed vector near 0, edges are same
        if crossed.norm2().abs() < 1e-5 {
            Err(anyhow::anyhow!("Can not define plane from same edges"))
        } else {
            Ok(Plane {
                normal: crossed.unit(),
                r0: edge1.start().clone(),
            })
        }
    }

    /// Get normal
    pub fn normal(&self) -> &Vector3d {
        &self.normal
    }
}
