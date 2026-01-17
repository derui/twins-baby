use std::fmt::Display;

use anyhow::Result;

use crate::point::Point;

/// Edge implementation.
///
/// This structure is totally immutable.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge(Point, Point);

impl Edge {
    pub fn new(start: Point, end: Point) -> Result<Self> {
        if start == end {
            Err(anyhow::anyhow!("Can not define edge between same point"))
        } else {
            Ok(Edge(start, end))
        }
    }

    #[inline]
    pub fn start(&self) -> &Point {
        &self.0
    }

    #[inline]
    pub fn end(&self) -> &Point {
        &self.1
    }

    /// Create a new [Edge] with the given start point.
    pub fn with_start(&self, point: Point) -> Result<Self> {
        Self::new(point, self.1)
    }

    /// Create a new [Edge] with the given end point.
    pub fn with_end(&self, point: Point) -> Result<Self> {
        Self::new(self.0, point)
    }
}

impl From<(Point, Point)> for Edge {
    fn from(value: (Point, Point)) -> Self {
        Edge::new(value.0, value.1).unwrap()
    }
}

impl From<Edge> for (Point, Point) {
    fn from(value: Edge) -> Self {
        (value.0, value.1)
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn p(x: f32, y: f32, z: f32) -> Point {
        Point::new(x, y, z)
    }

    #[test]
    fn new_creates_edge_with_different_points() {
        // Arrange
        let start = p(0.0, 0.0, 0.0);
        let end = p(1.0, 1.0, 1.0);

        // Act
        let edge = Edge::new(start, end);

        // Assert
        let edge = edge.expect("should create edge");
        assert_eq!(*edge.start(), start);
        assert_eq!(*edge.end(), end);
    }

    #[test]
    fn new_fails_with_same_points() {
        // Arrange
        let point = p(1.0, 1.0, 1.0);

        // Act
        let result = Edge::new(point, point);

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
        let new_edge = edge.with_start(new_start);

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(*new_edge.start(), new_start);
        assert_eq!(*new_edge.end(), *edge.end());
    }

    #[test]
    fn with_start_fails_when_same_as_end() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let result = edge.with_start(*edge.end());

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn with_end_creates_new_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();
        let new_end = p(2.0, 2.0, 2.0);

        // Act
        let new_edge = edge.with_end(new_end);

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(*new_edge.start(), *edge.start());
        assert_eq!(*new_edge.end(), new_end);
    }

    #[test]
    fn with_end_fails_when_same_as_start() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let result = edge.with_end(*edge.start());

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn from_tuple_creates_edge() {
        // Arrange
        let start = p(0.0, 0.0, 0.0);
        let end = p(1.0, 1.0, 1.0);

        // Act
        let edge: Edge = (start, end).into();

        // Assert
        assert_eq!(*edge.start(), start);
        assert_eq!(*edge.end(), end);
    }

    #[test]
    fn into_tuple_converts_edge() {
        // Arrange
        let edge = Edge::new(p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)).unwrap();

        // Act
        let tuple: (Point, Point) = edge.clone().into();

        // Assert
        assert_eq!(tuple, (*edge.start(), *edge.end()));
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
}
