use std::fmt::Display;

use anyhow::Result;

use super::vertex::Vertex;

/// Edge implementation.
///
/// This structure is totally immutable.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
}

impl Edge {
    pub fn new(start: Vertex, end: Vertex) -> Result<Self> {
        if start == end {
            Err(anyhow::anyhow!("Can not define edge between same point"))
        } else {
            Ok(Edge { start, end })
        }
    }

    /// Create a new [Edge] with the given start point.
    pub fn with_start(&self, point: Vertex) -> Result<Self> {
        Self::new(point, self.end.clone())
    }

    /// Create a new [Edge] with the given end point.
    pub fn with_end(&self, point: Vertex) -> Result<Self> {
        Self::new(self.start.clone(), point)
    }

    /// Get length of the edge
    pub fn len(&self) -> f32 {
        let l = (*self.end.x - *self.start.x).powi(2)
            + (*self.end.y - *self.start.y).powi(2)
            + (*self.end.z - *self.start.z).powi(2);
        l.sqrt()
    }
}

impl From<(Vertex, Vertex)> for Edge {
    fn from(value: (Vertex, Vertex)) -> Self {
        Edge::new(value.0, value.1).unwrap()
    }
}

impl From<Edge> for (Vertex, Vertex) {
    fn from(value: Edge) -> Self {
        (value.start.clone(), value.end.clone())
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(({}, {}, {}) -> ({}, {}, {}))",
            *self.start.x, *self.start.y, *self.start.z, *self.end.x, *self.end.y, *self.end.z
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::solid::vertex::Vertex;

    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    fn p(x: f32, y: f32, z: f32) -> Vertex {
        Vertex::new(x, y, z)
    }

    #[test]
    fn new_creates_edge_with_different_points() {
        // Arrange
        let start = p(0.0, 0.0, 0.0);
        let end = p(1.0, 1.0, 1.0);

        // Act
        let edge = Edge::new(start.clone(), end.clone());

        // Assert
        let edge = edge.expect("should create edge");
        assert_eq!(edge.start, start);
        assert_eq!(edge.end, end);
    }

    #[test]
    fn new_fails_with_same_points() {
        // Arrange
        let point = p(1.0, 1.0, 1.0);

        // Act
        let result = Edge::new(point.clone(), point);

        // Assert
        let err = result.expect_err("should fail");
        assert!(err.to_string().contains("same point"));
    }

    #[test]
    fn with_start_creates_new_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();
        let new_start = p(2.0, 2.0, 2.0);

        // Act
        let new_edge = edge.with_start(new_start.clone());

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(new_edge.start, new_start);
        assert_eq!(new_edge.end, edge.end);
    }

    #[test]
    fn with_start_fails_when_same_as_end() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let result = edge.with_start(edge.end.clone());

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn with_end_creates_new_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();
        let new_end = p(2.0, 2.0, 2.0);

        // Act
        let new_edge = edge.with_end(new_end.clone());

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(new_edge.start, edge.start);
        assert_eq!(new_edge.end, new_end);
    }

    #[test]
    fn with_end_fails_when_same_as_start() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let result = edge.with_end(edge.start.clone());

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn from_tuple_creates_edge() {
        // Arrange
        let start = p(0.0, 0.0, 0.0);
        let end = p(1.0, 1.0, 1.0);

        // Act
        let edge: Edge = (start.clone(), end.clone()).into();

        // Assert
        assert_eq!(edge.start, start);
        assert_eq!(edge.end, end);
    }

    #[test]
    fn into_tuple_converts_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let tuple: (Vertex, Vertex) = edge.clone().into();

        // Assert
        assert_eq!(tuple, (edge.start.clone(), edge.end.clone()));
    }

    #[test]
    fn display_formats_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 1.0, 2.0), p(3.0, 4.0, 5.0)).unwrap();

        // Act
        let result = format!("{}", edge);

        // Assert
        assert_eq!(result, "((0, 1, 2) -> (3, 4, 5))");
    }

    #[test]
    fn edges_with_same_points_are_equal() {
        // Arrange
        let e1 = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();
        let e2 = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act & Assert
        assert_eq!(e1, e2);
    }

    #[test]
    fn len_calculates_length_along_single_axis() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(3.0, 0.0, 0.0)).unwrap();

        // Act
        let length = edge.len();

        // Assert
        assert_relative_eq!(length, 3.0, epsilon = 1e-6);
    }

    #[test]
    fn len_calculates_length_in_2d_plane() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(3.0, 4.0, 0.0)).unwrap();

        // Act
        let length = edge.len();

        // Assert
        assert_relative_eq!(length, 5.0, epsilon = 1e-6);
    }

    #[test]
    fn len_calculates_length_in_3d_space() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let length = edge.len();

        // Assert
        assert_relative_eq!(length, 3.0_f32.sqrt(), epsilon = 1e-6);
    }

    #[test]
    fn len_handles_negative_coordinates() {
        // Arrange
        let edge = Edge::new(p(-1.0, -1.0, -1.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let length = edge.len();

        // Assert
        assert_relative_eq!(length, 12.0_f32.sqrt(), epsilon = 1e-6);
    }

    #[test]
    fn len_calculates_unit_length() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0)).unwrap();

        // Act
        let length = edge.len();

        // Assert
        assert_relative_eq!(length, 1.0, epsilon = 1e-6);
    }
}
