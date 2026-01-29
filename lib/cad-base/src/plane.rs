use std::marker::PhantomData;

use anyhow::{self, Result};
use epsilon::{DefaultEpsilon, Epsilon, approx_zero};

use crate::{edge::Edge, point::Point, vector3d::Vector3d};

/// Simple plane definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Plane<E: Epsilon = DefaultEpsilon> {
    /// normal vector of the vector
    normal: Vector3d,

    /// point on the plane
    r0: Point,

    _data: PhantomData<E>,
}

impl<E: Epsilon> Plane<E> {
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
                _data: PhantomData,
            })
        }
    }

    /// A new XY-plane. It contains origin and Z-unit vector.
    pub fn new_xy() -> Self {
        Plane {
            normal: Vector3d::new_z_unit(),
            r0: Point::zero(),
            _data: PhantomData,
        }
    }

    /// A new XZ-plane. It contains origin and Y-unit vector.
    pub fn new_xz() -> Self {
        Plane {
            normal: Vector3d::new_y_unit(),
            r0: Point::zero(),
            _data: PhantomData,
        }
    }

    /// A new YZ-plane. It contains origin and X-unit vector.
    pub fn new_yz() -> Self {
        Plane {
            normal: Vector3d::new_x_unit(),
            r0: Point::zero(),
            _data: PhantomData,
        }
    }

    /// Get normal
    pub fn normal(&self) -> &Vector3d {
        &self.normal
    }

    /// Check the [point] on the plane or not
    pub fn is_on_plane(&self, point: &Point) -> bool {
        let r0: Vector3d = self.r0.into();
        let r: Vector3d = point.into();

        let ret = self.normal.dot(&(r0 - r));

        approx_zero::<E>(ret.abs())
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

    type Plane = super::Plane<DefaultEpsilon>;

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
            assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.z, 1.0, epsilon = 1e-5);
        }

        #[test]
        fn new_xy_creates_plane_with_z_normal() {
            // Arrange & Act
            let plane = Plane::new_xy();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.z, 1.0, epsilon = 1e-5);
        }

        #[test]
        fn new_xz_creates_plane_with_y_normal() {
            // Arrange & Act
            let plane = Plane::new_xz();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.y, 1.0, epsilon = 1e-5);
            assert_relative_eq!(normal.z, 0.0, epsilon = 1e-5);
        }

        #[test]
        fn new_yz_creates_plane_with_x_normal() {
            // Arrange & Act
            let plane = Plane::new_yz();

            // Assert
            let normal = plane.normal();
            assert_relative_eq!(normal.x, 1.0, epsilon = 1e-5);
            assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.z, 0.0, epsilon = 1e-5);
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

            assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
            assert_relative_eq!(normal.z, 1.0, epsilon = 1e-5);
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
            assert_relative_eq!(normal1.x, -normal2.x, epsilon = 1e-5);
            assert_relative_eq!(normal1.y, -normal2.y, epsilon = 1e-5);
            assert_relative_eq!(normal1.z, -normal2.z, epsilon = 1e-5);
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
            assert_relative_eq!(normal1.x, normal2.x, epsilon = 1e-5);
            assert_relative_eq!(normal1.y, normal2.y, epsilon = 1e-5);
            assert_relative_eq!(normal1.z, normal2.z, epsilon = 1e-5);
        }
    }

    mod is_on_plane {
        use super::*;

        #[test]
        fn returns_true_for_point_at_origin_on_xy_plane() {
            // Arrange
            let plane = Plane::new_xy();
            let point = p(0.0, 0.0, 0.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_true_for_point_on_xy_plane() {
            // Arrange
            let plane = Plane::new_xy();
            let point = p(5.0, 3.0, 0.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_false_for_point_above_xy_plane() {
            // Arrange
            let plane = Plane::new_xy();
            let point = p(1.0, 1.0, 1.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(!result);
        }

        #[test]
        fn returns_false_for_point_below_xy_plane() {
            // Arrange
            let plane = Plane::new_xy();
            let point = p(0.0, 0.0, -2.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(!result);
        }

        #[test]
        fn returns_true_for_point_on_xz_plane() {
            // Arrange
            let plane = Plane::new_xz();
            let point = p(3.0, 0.0, 5.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_true_for_point_on_yz_plane() {
            // Arrange
            let plane = Plane::new_yz();
            let point = p(0.0, 2.0, 4.0);

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_true_for_edge_start_point() {
            // Arrange
            let edge1 = edge(1.0, 2.0, 3.0, 4.0, 2.0, 3.0);
            let edge2 = edge(1.0, 2.0, 3.0, 1.0, 5.0, 3.0);
            let plane = Plane::new(&edge1, &edge2).expect("should create plane");

            // Act
            let result = plane.is_on_plane(edge1.start());

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_true_for_edge_end_point() {
            // Arrange
            let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let plane = Plane::new(&edge1, &edge2).expect("should create plane");

            // Act
            let result = plane.is_on_plane(edge1.end());

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_true_for_point_on_custom_plane() {
            // Arrange
            let edge1 = edge(1.0, 1.0, 1.0, 2.0, 1.0, 1.0);
            let edge2 = edge(1.0, 1.0, 1.0, 1.0, 2.0, 1.0);
            let plane = Plane::new(&edge1, &edge2).expect("should create plane");
            let point = p(5.0, 5.0, 1.0); // Any point with z=1

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(result);
        }

        #[test]
        fn returns_false_for_point_not_on_custom_plane() {
            // Arrange
            let edge1 = edge(1.0, 1.0, 1.0, 2.0, 1.0, 1.0);
            let edge2 = edge(1.0, 1.0, 1.0, 1.0, 2.0, 1.0);
            let plane = Plane::new(&edge1, &edge2).expect("should create plane");
            let point = p(5.0, 5.0, 2.0); // z != 1

            // Act
            let result = plane.is_on_plane(&point);

            // Assert
            assert!(!result);
        }
    }
}
