use super::*;
use crate::id::PlaneId;
use crate::sketch::AttachableTarget;
use crate::sketch::geometry::{Geometry, LineSegment};

mod sketch {
    use super::*;

    mod add_geometry {
        use super::*;

        #[test]
        fn add_geometry_returns_geometry_id() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let start = Point2::new(0.0, 0.0);
            let end = Point2::new(1.0, 1.0);

            // Act
            let geometry_id = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });

            // Assert
            assert!(sketch.remove_geometry(&geometry_id).is_some());
        }

        #[test]
        fn add_geometry_generates_unique_ids() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let start = Point2::new(0.0, 0.0);
            let end = Point2::new(1.0, 1.0);

            // Act
            let geometry_id1 = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });
            let geometry_id2 = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });

            // Assert
            assert_ne!(geometry_id1, geometry_id2);
        }
    }

    mod resolve_edges {
        use super::*;
        use approx::assert_relative_eq;
        use pretty_assertions::assert_eq;

        #[test]
        fn returns_empty_vec_when_no_geometries() {
            // Arrange
            let sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn resolves_single_line_segment() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let start = Point2::new(1.0, 2.0);
            let end = Point2::new(3.0, 4.0);
            sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 1);
            assert_relative_eq!(*edges[0].start.x, 1.0);
            assert_relative_eq!(*edges[0].start.y, 2.0);
            assert_relative_eq!(*edges[0].end.x, 3.0);
            assert_relative_eq!(*edges[0].end.y, 4.0);
        }

        #[test]
        fn resolves_multiple_line_segments() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(
                    &Point2::new(0.0, 0.0),
                    &Point2::new(1.0, 0.0),
                    scope,
                ))
            });
            sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(
                    &Point2::new(1.0, 0.0),
                    &Point2::new(1.0, 1.0),
                    scope,
                ))
            });
            sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(
                    &Point2::new(1.0, 1.0),
                    &Point2::new(0.0, 0.0),
                    scope,
                ))
            });

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 3);
        }

        #[test]
        fn does_not_resolve_removed_geometry() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let id = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(
                    &Point2::new(0.0, 0.0),
                    &Point2::new(1.0, 1.0),
                    scope,
                ))
            });
            sketch.remove_geometry(&id);

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn resolves_edges_with_negative_coordinates() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(
                    &Point2::new(-5.0, -3.0),
                    &Point2::new(-1.0, -2.0),
                    scope,
                ))
            });

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 1);
            assert_relative_eq!(*edges[0].start.x, -5.0);
            assert_relative_eq!(*edges[0].start.y, -3.0);
            assert_relative_eq!(*edges[0].end.x, -1.0);
            assert_relative_eq!(*edges[0].end.y, -2.0);
        }
    }

    mod remove_geometry {
        use super::*;

        #[test]
        fn remove_geometry_returns_removed_geometry() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let start = Point2::new(0.0, 0.0);
            let end = Point2::new(1.0, 1.0);
            let geometry_id = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });

            // Act
            let result = sketch.remove_geometry(&geometry_id);

            // Assert
            assert!(result.is_some());
            assert!(sketch.remove_geometry(&geometry_id).is_none());
        }

        #[test]
        fn remove_geometry_returns_none_for_nonexistent() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let nonexistent_id = GeometryId::from(999);

            // Act
            let result = sketch.remove_geometry(&nonexistent_id);

            // Assert
            assert!(result.is_none());
        }

        #[test]
        fn remove_geometry_does_not_affect_other_geometries() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", &AttachableTarget::Plane(PlaneId::new(1)));
            let start = Point2::new(0.0, 0.0);
            let end = Point2::new(1.0, 1.0);
            let geometry_id1 = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });
            let geometry_id2 = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });
            let geometry_id3 = sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });

            // Act
            sketch.remove_geometry(&geometry_id2);

            // Assert
            assert!(sketch.remove_geometry(&geometry_id1).is_some());
            assert!(sketch.remove_geometry(&geometry_id2).is_none());
            assert!(sketch.remove_geometry(&geometry_id3).is_some());
        }
    }
}
