use std::ops::Deref;

use crate::point::Point;

/// Immutable vertex for solid.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex(Point);

impl Vertex {
    /// Get new vertex
    pub fn new(point: &Point) -> Self {
        Vertex(point.clone())
    }
}

// Default must be origin point.
impl Default for Vertex {
    fn default() -> Self {
        Vertex::new(&Point::zero())
    }
}

impl From<Point> for Vertex {
    fn from(point: Point) -> Self {
        Vertex(point)
    }
}

impl Deref for Vertex {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, 0.0)]
    #[case(1.0, 2.0, 3.0)]
    #[case(-1.5, 0.0, 99.9)]
    fn new_creates_vertex_with_given_coordinates(#[case] x: f32, #[case] y: f32, #[case] z: f32) {
        // Act
        let v = Vertex::new(&Point::new(x, y, z));

        // Assert
        assert_relative_eq!(*v.x, x);
        assert_relative_eq!(*v.y, y);
        assert_relative_eq!(*v.z, z);
    }

    #[test]
    fn default_is_origin() {
        // Act
        let v = Vertex::default();

        // Assert
        assert_relative_eq!(*v.x, 0.0);
        assert_relative_eq!(*v.y, 0.0);
        assert_relative_eq!(*v.z, 0.0);
    }

    #[test]
    fn equality_for_same_coordinates() {
        // Arrange
        let v1 = Vertex::new(&Point::new(1.0, 2.0, 3.0));
        let v2 = Vertex::new(&Point::new(1.0, 2.0, 3.0));

        // Assert
        assert_eq!(v1, v2);
    }

    #[test]
    fn inequality_for_different_coordinates() {
        // Arrange
        let v1 = Vertex::new(&Point::new(1.0, 2.0, 3.0));
        let v2 = Vertex::new(&Point::new(1.0, 2.0, 4.0));

        // Assert
        assert_ne!(v1, v2);
    }

    #[test]
    fn clone_produces_equal_vertex() {
        // Arrange
        let v1: Vertex = Point::new(1.0, 2.0, 3.0).into();

        // Act
        let v2 = v1.clone();

        // Assert
        assert_eq!(v1, v2);
    }
}
