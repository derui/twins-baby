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
                r0: *edge1.start(),
            })
        }
    }

    /// A new XY-plane. It contains origin and Z-unit vector.
    pub fn new_xy() -> Self {
        Plane {
            normal: Vector3d::new_z_unit(),
            r0: Point::zero(),
        }
    }

    /// A new XZ-plane. It contains origin and Y-unit vector.
    pub fn new_xz() -> Self {
        Plane {
            normal: Vector3d::new_y_unit(),
            r0: Point::zero(),
        }
    }

    /// A new YZ-plane. It contains origin and X-unit vector.
    pub fn new_yz() -> Self {
        Plane {
            normal: Vector3d::new_x_unit(),
            r0: Point::zero(),
        }
    }

    /// Get normal
    pub fn normal(&self) -> &Vector3d {
        &self.normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn p(x: f32, y: f32, z: f32) -> Point {
        Point::new(x, y, z)
    }

    fn edge(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Edge {
        Edge::new(p(x1, y1, z1), p(x2, y2, z2)).unwrap()
    }

    mod construction {
        use super::*;

        #[test]
        fn new_from_two_perpendicular_edges() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0); // X-axis
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0); // Y-axis

            // Act
            let plane = Plane::new(&edge1, &edge2);

            // Assert
            let plane = plane.expect("should create plane");
            let normal = plane.normal();
            assert_relative_eq!(*normal.x(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.y(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.z(), 1.0, epsilon = 1e-5);
        }

        #[test]
        fn new_xy_creates_plane_with_z_normal() {
            // Arrange & Act
            let plane = Plane::new_xy();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(*normal.x(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.y(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.z(), 1.0, epsilon = 1e-5);
        }

        #[test]
        fn new_xz_creates_plane_with_y_normal() {
            // Arrange & Act
            let plane = Plane::new_xz();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(*normal.x(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.y(), 1.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.z(), 0.0, epsilon = 1e-5);
        }

        #[test]
        fn new_yz_creates_plane_with_x_normal() {
            // Arrange & Act
            let plane = Plane::new_yz();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(*normal.x(), 1.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.y(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.z(), 0.0, epsilon = 1e-5);
        }

        #[test]
        fn new_from_two_non_perpendicular_edges() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0); // X-axis
            let edge2 = edge(0.0, 0.0, 0.0, 1.0, 1.0, 0.0); // 45 degrees in XY plane

            // Act
            let plane = Plane::new(&edge1, &edge2);

            // Assert
            let plane = plane.expect("should create plane");
            let normal = plane.normal();

            assert_relative_eq!(*normal.x(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.y(), 0.0, epsilon = 1e-5);
            assert_relative_eq!(*normal.z(), 1.0, epsilon = 1e-5);
        }

        #[test]
        fn new_returns_unit_normal() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 3.0, 0.0, 0.0); // Scaled X-axis
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 5.0, 0.0); // Scaled Y-axis

            // Act
            let plane = Plane::new(&edge1, &edge2);

            // Assert
            let plane = plane.expect("should create plane");
            let normal = plane.normal();
            let norm = normal.norm2().sqrt();
            assert_relative_eq!(norm, 1.0, epsilon = 1e-5);
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn new_fails_for_parallel_edges() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, 2.0, 0.0, 0.0); // Same direction, different magnitude

            // Act
            let result = Plane::new(&edge1, &edge2);

            // Assert
            let err = result.expect_err("should fail for parallel edges");
            assert!(err.to_string().contains("same edges"));
        }

        #[test]
        fn new_fails_for_antiparallel_edges() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, -1.0, 0.0, 0.0); // Opposite direction

            // Act
            let result = Plane::new(&edge1, &edge2);

            // Assert
            let err = result.expect_err("should fail for antiparallel edges");
            assert!(err.to_string().contains("same edges"));
        }

        #[test]
        fn new_fails_for_same_edge() {
            // Arrange
            let e = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

            // Act
            let result = Plane::new(&e, &e);

            // Assert
            let err = result.expect_err("should fail for same edge");
            assert!(err.to_string().contains("same edges"));
        }
    }

    mod properties {
        use super::*;

        #[test]
        fn normal_is_perpendicular_to_both_edges() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 2.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 1.0);

            // Act
            let plane = Plane::new(&edge1, &edge2).expect("should create plane");
            let normal = plane.normal();
            let v1 = Vector3d::from_edge(&edge1);
            let v2 = Vector3d::from_edge(&edge2);

            // Assert
            assert_relative_eq!(normal.dot(&v1), 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.dot(&v2), 0.0, epsilon = 1e-5);
        }

        #[test]
        fn swapping_edges_reverses_normal() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

            // Act
            let plane1 = Plane::new(&edge1, &edge2).expect("should create plane");
            let plane2 = Plane::new(&edge2, &edge1).expect("should create plane");
            let normal1 = plane1.normal();
            let normal2 = plane2.normal();

            // Assert
            assert_relative_eq!(*normal1.x(), -*normal2.x(), epsilon = 1e-5);
            assert_relative_eq!(*normal1.y(), -*normal2.y(), epsilon = 1e-5);
            assert_relative_eq!(*normal1.z(), -*normal2.z(), epsilon = 1e-5);
        }

        #[test]
        fn edge_magnitude_does_not_affect_normal_direction() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let scaled_edge1 = edge(0.0, 0.0, 0.0, 5.0, 0.0, 0.0); // 5x scale
            let scaled_edge2 = edge(0.0, 0.0, 0.0, 0.0, 3.0, 0.0); // 3x scale

            // Act
            let plane1 = Plane::new(&edge1, &edge2).expect("should create plane");
            let plane2 = Plane::new(&scaled_edge1, &scaled_edge2).expect("should create plane");
            let normal1 = plane1.normal();
            let normal2 = plane2.normal();

            // Assert
            assert_relative_eq!(*normal1.x(), *normal2.x(), epsilon = 1e-5);
            assert_relative_eq!(*normal1.y(), *normal2.y(), epsilon = 1e-5);
            assert_relative_eq!(*normal1.z(), *normal2.z(), epsilon = 1e-5);
        }
    }
}
