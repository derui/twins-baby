use super::*;
use crate::body::BodyPerspective;
use crate::sketch::AttachableTarget;
use crate::sketch::geometry::{Geometry, LineSegment};

fn make_attach_target() -> AttachableTarget {
    let mut bodies = BodyPerspective::new();
    let body_id = bodies.add_body();
    let plane_ref = bodies.to_x_plane_ref(&body_id).unwrap();
    AttachableTarget::Plane(plane_ref)
}

mod attachable_target {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::CadEngine;
    use crate::feature::operation::{Operation, Pad};
    use crate::feature::{Evaluate, EvaluateError, Feature, FeatureContext, FeaturePerspective};
    use crate::id::{EdgeId, FaceId, SketchId};
    use crate::plane::Plane;

    use crate::sketch::AttachableTarget;
    use crate::solid::face::{Face, PlanarSurface};
    use crate::solid::{Solid, SolidBuilder};
    use crate::{body::BodyPerspective, id::BodyId};

    fn make_context<'a>() -> FeatureContext<'a> {
        FeatureContext {
            sketches: vec![].into(),
            target: vec![].into(),
        }
    }

    fn make_operation() -> Operation {
        let eq: solver::equation::Equation = 10.0.into();
        Pad::new(&eq).into()
    }

    struct OneSolidEvaluator;
    impl Evaluate for OneSolidEvaluator {
        fn evaluate<'a>(
            _feature: &Feature,
            _context: &FeatureContext<'a>,
        ) -> Result<Vec<Solid>, EvaluateError> {
            let mut builder = SolidBuilder::default();
            let face = Face::Planar(
                PlanarSurface::new(
                    &[EdgeId::from(1), EdgeId::from(2), EdgeId::from(3)],
                    &Plane::new_xy(),
                )
                .unwrap(),
            );
            builder.add_faces(&[face]);
            Ok(vec![builder.build()])
        }
    }

    /// Build a body and a solid-with-face in a fresh engine, returning the engine plus the ids
    /// needed to construct plane/face refs against it.
    fn make_engine_with_body_and_solid() -> (CadEngine, BodyId, FaceId) {
        let mut engine = CadEngine::new();
        let body_id;
        let face_id;
        {
            let mut transaction = engine.begin();
            body_id = transaction.modify::<BodyPerspective>().unwrap().add_body();

            let feature_perspective = transaction.modify::<FeaturePerspective>().unwrap();
            let feature_id = feature_perspective.add_feature(
                BodyId::from(1),
                SketchId::from(1),
                &make_operation(),
            );
            let context = make_context();
            feature_perspective
                .evaluate_feature::<OneSolidEvaluator>(&feature_id, &context)
                .unwrap();
            let solids = (*feature_perspective.get(&feature_id).unwrap().solids)
                .as_ref()
                .unwrap();
            let solid = solids.iter().next().unwrap();
            face_id = *solid.faces.keys().next().unwrap();
            transaction.commit();
        }
        (engine, body_id, face_id)
    }

    #[rstest]
    #[case::x(BodyPerspective::to_x_plane_ref as fn(&BodyPerspective, &BodyId) -> Option<crate::refs::PlaneRef>)]
    #[case::y(BodyPerspective::to_y_plane_ref as fn(&BodyPerspective, &BodyId) -> Option<crate::refs::PlaneRef>)]
    #[case::z(BodyPerspective::to_z_plane_ref as fn(&BodyPerspective, &BodyId) -> Option<crate::refs::PlaneRef>)]
    fn to_plane_resolves_plane_variant_for_each_axis(
        #[case] to_plane_ref: fn(&BodyPerspective, &BodyId) -> Option<crate::refs::PlaneRef>,
    ) {
        // Arrange
        let (engine, body_id, _) = make_engine_with_body_and_solid();
        let baseline = engine.baseline();
        let plane_ref =
            to_plane_ref(baseline.read::<BodyPerspective>().unwrap(), &body_id).unwrap();
        let target = AttachableTarget::Plane(plane_ref);

        // Act
        let result = target.to_plane(&baseline);

        // Assert
        assert!(result.is_some());
    }

    #[test]
    fn to_plane_returns_none_for_unknown_body_id() {
        // Arrange
        let (engine, _, _) = make_engine_with_body_and_solid();
        let baseline = engine.baseline();
        let target = AttachableTarget::Plane(crate::refs::PlaneRef::with_x(BodyId::from(999)));

        // Act
        let result = target.to_plane(&baseline);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn to_plane_ref_returns_some_for_plane_variant() {
        // Arrange
        let mut bodies = BodyPerspective::new();
        let body_id = bodies.add_body();
        let plane_ref = bodies.to_x_plane_ref(&body_id).unwrap();
        let target = AttachableTarget::Plane(plane_ref.clone());

        // Act
        let result = target.to_plane_ref();

        // Assert
        assert_eq!(result, Some(plane_ref));
    }

    #[test]
    fn to_face_ref_returns_none_for_plane_variant() {
        // Arrange
        let mut bodies = BodyPerspective::new();
        let body_id = bodies.add_body();
        let plane_ref = bodies.to_x_plane_ref(&body_id).unwrap();
        let target = AttachableTarget::Plane(plane_ref);

        // Act
        let result = target.to_face_ref();

        // Assert
        assert!(result.is_none());
    }
}

mod sketch {
    use super::*;

    mod add_geometry {
        use super::*;

        #[test]
        fn add_geometry_returns_geometry_id() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());

            // Act
            let edges = sketch.resolve_edges().unwrap();

            // Assert
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn resolves_single_line_segment() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
            let nonexistent_id = GeometryId::from(999);

            // Act
            let result = sketch.remove_geometry(&nonexistent_id);

            // Assert
            assert!(result.is_none());
        }

        #[test]
        fn remove_geometry_does_not_affect_other_geometries() {
            // Arrange
            let mut sketch = Sketch::new("TestSketch", BodyId::from(1), &make_attach_target());
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
