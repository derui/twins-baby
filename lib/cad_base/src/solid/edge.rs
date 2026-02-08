
use anyhow::Result;
use immutable::Im;

use crate::id::VertexId;


/// Edge implementation.
///
/// This structure is totally immutable.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub start: Im<VertexId>,
    pub end: Im<VertexId>,
    _immutable: (),
}

impl Edge {
    pub fn new(start: VertexId, end: VertexId) -> Result<Self> {
        if start == end {
            Err(anyhow::anyhow!("Can not define edge between same point"))
        } else {
            Ok(Edge {
                start: start.into(),
                end: end.into(),
                _immutable: (),
            })
        }
    }

    /// Create a new [Edge] with the given start point.
    pub fn with_start(&self, point: VertexId) -> Result<Self> {
        Self::new(point, *self.end)
    }

    /// Create a new [Edge] with the given end point.
    pub fn with_end(&self, point: VertexId) -> Result<Self> {
        Self::new(*self.start, point)
    }
}

impl From<(VertexId, VertexId)> for Edge {
    fn from(value: (VertexId, VertexId)) -> Self {
        Edge::new(value.0, value.1).unwrap()
    }
}

impl From<Edge> for (VertexId, VertexId) {
    fn from(value: Edge) -> Self {
        (*value.start, *value.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::id::VertexId;

    use super::*;
    use pretty_assertions::assert_eq;

    fn vid(n: u64) -> VertexId {
        VertexId::from(n)
    }

    #[test]
    fn new_creates_edge_with_different_ids() {
        // Arrange
        let start = vid(1);
        let end = vid(2);

        // Act
        let edge = Edge::new(start, end);

        // Assert
        let edge = edge.expect("should create edge");
        assert_eq!(*edge.start, start);
        assert_eq!(*edge.end, end);
    }

    #[test]
    fn new_fails_with_same_ids() {
        // Arrange
        let point = vid(1);

        // Act
        let result = Edge::new(point, point);

        // Assert
        let err = result.expect_err("should fail");
        assert!(err.to_string().contains("same point"));
    }

    #[test]
    fn with_start_creates_new_edge() {
        // Arrange
        let edge = Edge::new(vid(1), vid(2)).unwrap();
        let new_start = vid(3);

        // Act
        let new_edge = edge.with_start(new_start);

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(*new_edge.start, new_start);
        assert_eq!(new_edge.end, edge.end);
    }

    #[test]
    fn with_start_fails_when_same_as_end() {
        // Arrange
        let edge = Edge::new(vid(1), vid(2)).unwrap();

        // Act
        let result = edge.with_start(*edge.end);

        // Assert
        result.expect_err("should fail when same as end");
    }

    #[test]
    fn with_end_creates_new_edge() {
        // Arrange
        let edge = Edge::new(vid(1), vid(2)).unwrap();
        let new_end = vid(3);

        // Act
        let new_edge = edge.with_end(new_end);

        // Assert
        let new_edge = new_edge.expect("should create edge");
        assert_eq!(new_edge.start, edge.start);
        assert_eq!(*new_edge.end, new_end);
    }

    #[test]
    fn with_end_fails_when_same_as_start() {
        // Arrange
        let edge = Edge::new(vid(1), vid(2)).unwrap();

        // Act
        let result = edge.with_end(*edge.start);

        // Assert
        result.expect_err("should fail when same as start");
    }

    #[test]
    fn from_tuple_creates_edge() {
        // Arrange
        let start = vid(1);
        let end = vid(2);

        // Act
        let edge: Edge = (start, end).into();

        // Assert
        assert_eq!(*edge.start, start);
        assert_eq!(*edge.end, end);
    }

    #[test]
    fn into_tuple_converts_edge() {
        // Arrange
        let edge = Edge::new(vid(1), vid(2)).unwrap();

        // Act
        let tuple: (VertexId, VertexId) = edge.clone().into();

        // Assert
        assert_eq!(tuple, (*edge.start, *edge.end));
    }

    #[test]
    fn edges_with_same_ids_are_equal() {
        // Arrange
        let e1 = Edge::new(vid(1), vid(2)).unwrap();
        let e2 = Edge::new(vid(1), vid(2)).unwrap();

        // Act & Assert
        assert_eq!(e1, e2);
    }
}
