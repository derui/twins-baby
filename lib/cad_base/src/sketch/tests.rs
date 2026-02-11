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
