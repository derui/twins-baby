use cad_base::{
    feature::AttachedTarget,
    id::{FaceId, PlaneId},
    plane::Plane,
    sketch::{AttachableTarget, Geometry, LineSegment, Point2, Sketch},
};
use epsilon::DefaultEpsilon;

use super::{Sketcher, SketcherError};

fn plane_sketch() -> Sketch {
    let target = AttachableTarget::Plane(PlaneId::from(1));
    Sketch::new("test", &target)
}

fn face_sketch() -> Sketch {
    let target = AttachableTarget::Face(FaceId::from(1));
    Sketch::new("test", &target)
}

fn add_segment(sketch: &mut Sketch, start: (f32, f32), end: (f32, f32)) {
    sketch.add_geometry(|vars| {
        Geometry::LineSegment(LineSegment::from_points(
            &Point2::new(start.0, start.1),
            &Point2::new(end.0, end.1),
            vars,
        ))
    });
}

fn triangle_sketch() -> Sketch {
    // A(0,0)→B(1,0)→C(0,1)→A(0,0)
    let mut sketch = plane_sketch();
    add_segment(&mut sketch, (0.0, 0.0), (1.0, 0.0));
    add_segment(&mut sketch, (1.0, 0.0), (0.0, 1.0));
    add_segment(&mut sketch, (0.0, 1.0), (0.0, 0.0));
    sketch
}

mod sketcher_new {
    use super::*;

    #[test]
    fn accepts_plane_sketch_with_plane_target() {
        // Arrange
        let sketch = plane_sketch();
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);

        // Act
        let result = Sketcher::new(&sketch, &target);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_plane_sketch_with_face_target() {
        // Arrange – Face cannot be easily constructed, so test the symmetric mismatch:
        // face sketch + plane target
        let sketch = face_sketch();
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);

        // Act
        let result = Sketcher::new(&sketch, &target);

        // Assert
        assert!(result.is_err());
    }
}

mod calculate_jordan_curves {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn returns_error_when_sketch_has_no_edges() {
        // Arrange
        let sketch = plane_sketch(); // no geometries added
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let result = sketcher.calculate_jordan_corves::<DefaultEpsilon>();

        // Assert
        assert!(matches!(result, Err(SketcherError::SketchNotHaveEdge)));
    }

    #[test]
    fn returns_error_when_graph_has_branch() {
        // Arrange – two edges share a start point (branch, not a valid Jordan curve)
        let mut sketch = plane_sketch();
        add_segment(&mut sketch, (0.0, 0.0), (1.0, 0.0));
        add_segment(&mut sketch, (0.0, 0.0), (0.0, 1.0));

        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let result = sketcher.calculate_jordan_corves::<DefaultEpsilon>();

        // Assert
        assert!(matches!(result, Err(SketcherError::SketchHasNoJordanCurve)));
    }

    #[test]
    fn triangle_returns_one_curve_with_three_points() {
        // Arrange
        let sketch = triangle_sketch();
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let curves = sketcher
            .calculate_jordan_corves::<DefaultEpsilon>()
            .expect("should calculate curves");

        // Assert
        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].points.len(), 3);
    }

    #[test]
    fn triangle_points_lie_on_the_plane() {
        // Arrange
        let sketch = triangle_sketch();
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let curves = sketcher
            .calculate_jordan_corves::<DefaultEpsilon>()
            .expect("should calculate curves");

        // Assert – every projected 3D point must lie on the XY plane (z == 0)
        for point in &curves[0].points {
            assert_relative_eq!(*point.z, 0.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn square_returns_one_curve_with_four_points() {
        // Arrange – A(0,0)→B(1,0)→C(1,1)→D(0,1)→A(0,0)
        let mut sketch = plane_sketch();
        add_segment(&mut sketch, (0.0, 0.0), (1.0, 0.0));
        add_segment(&mut sketch, (1.0, 0.0), (1.0, 1.0));
        add_segment(&mut sketch, (1.0, 1.0), (0.0, 1.0));
        add_segment(&mut sketch, (0.0, 1.0), (0.0, 0.0));

        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let curves = sketcher
            .calculate_jordan_corves::<DefaultEpsilon>()
            .expect("should calculate curves");

        // Assert
        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].points.len(), 4);
    }

    #[test]
    fn edges_count_matches_points_count() {
        // Arrange
        let sketch = triangle_sketch();
        let plane = Plane::<DefaultEpsilon>::new_xy();
        let target = AttachedTarget::Plane(&plane);
        let sketcher = Sketcher::new(&sketch, &target).expect("should create sketcher");

        // Act
        let curves = sketcher
            .calculate_jordan_corves::<DefaultEpsilon>()
            .expect("should calculate curves");

        // Assert – edges connect adjacent point indices (0..n-1) → (1..n)
        let curve = &curves[0];
        let expected_edge_count = curve.points.len() - 1;
        assert_eq!(curve.edges.len(), expected_edge_count);
    }
}
