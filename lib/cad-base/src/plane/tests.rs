use super::*;
use approx::assert_relative_eq;

fn p(x: f32, y: f32, z: f32) -> Point {
    Point::new(x, y, z)
}

fn edge(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> (Point, Point) {
    (p(x1, y1, z1), p(x2, y2, z2))
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
        let plane = Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1));

        // Assert
        let plane = plane.expect("should create plane");
        let normal = plane.normal;
        assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
        assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
        assert_relative_eq!(normal.z, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn new_xy_creates_plane_with_z_normal() {
        // Arrange & Act
        let plane = Plane::new_xy();

        // Assert
        let normal = plane.normal;
        assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
        assert_relative_eq!(normal.y, 0.0, epsilon = 1e-5);
        assert_relative_eq!(normal.z, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn new_xz_creates_plane_with_y_normal() {
        // Arrange & Act
        let plane = Plane::new_xz();

        // Assert
        let normal = plane.normal;
        assert_relative_eq!(normal.x, 0.0, epsilon = 1e-5);
        assert_relative_eq!(normal.y, 1.0, epsilon = 1e-5);
        assert_relative_eq!(normal.z, 0.0, epsilon = 1e-5);
    }

    #[test]
    fn new_yz_creates_plane_with_x_normal() {
        // Arrange & Act
        let plane = Plane::new_yz();

        // Assert
        let normal = plane.normal;
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
        let plane = Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1));

        // Assert
        let plane = plane.expect("should create plane");
        let normal = plane.normal;

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
        let plane = Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1));

        // Assert
        let plane = plane.expect("should create plane");
        let normal = plane.normal;
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
        let result = Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1));

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
        let result = Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1));

        // Assert
        let err = result.expect_err("should fail for antiparallel edges");
        assert!(err.to_string().contains("same edges"));
    }

    #[test]
    fn new_fails_for_same_edge() {
        // Arrange
        let e = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Act
        let result = Plane::new((&e.0, &e.1), (&e.0, &e.1));

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
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let normal = plane.normal;
        let v1 = Vector3::from_points(&edge1.0, &edge1.1);
        let v2 = Vector3::from_points(&edge2.0, &edge2.1);

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
        let plane1 =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let plane2 =
            Plane::new((&edge2.0, &edge2.1), (&edge1.0, &edge1.1)).expect("should create plane");
        let normal1 = plane1.normal;
        let normal2 = plane2.normal;

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
        let plane1 =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let plane2 = Plane::new(
            (&scaled_edge1.0, &scaled_edge1.1),
            (&scaled_edge2.0, &scaled_edge2.1),
        )
        .expect("should create plane");
        let normal1 = plane1.normal;
        let normal2 = plane2.normal;

        // Assert
        assert_relative_eq!(normal1.x, normal2.x, epsilon = 1e-5);
        assert_relative_eq!(normal1.y, normal2.y, epsilon = 1e-5);
        assert_relative_eq!(normal1.z, normal2.z, epsilon = 1e-5);
    }
}

mod point_from_2d {
    use super::*;

    fn pt2(x: f32, y: f32) -> Point2 {
        Point2::new(x, y)
    }

    #[test]
    fn origin_maps_to_plane_anchor_on_xy_plane() {
        // Arrange
        let plane = Plane::new_xy();
        let point2 = pt2(0.0, 0.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert_relative_eq!(*result.x, 0.0, epsilon = 1e-5);
        assert_relative_eq!(*result.y, 0.0, epsilon = 1e-5);
        assert_relative_eq!(*result.z, 0.0, epsilon = 1e-5);
    }

    #[test]
    fn result_lies_on_the_plane_for_xy_plane() {
        // Arrange
        let plane = Plane::new_xy();
        let point2 = pt2(3.0, 4.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn result_lies_on_the_plane_for_yz_plane() {
        // Arrange
        let plane = Plane::new_yz();
        let point2 = pt2(3.0, 4.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn result_lies_on_the_plane_for_xz_plane() {
        // Arrange
        let plane = Plane::new_xz();
        let point2 = pt2(3.0, 4.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn result_lies_on_the_plane_for_custom_offset_plane() {
        // Arrange
        let edge1 = edge(1.0, 1.0, 1.0, 2.0, 1.0, 1.0);
        let edge2 = edge(1.0, 1.0, 1.0, 1.0, 2.0, 1.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let point2 = pt2(2.0, 3.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn negative_coordinates_produce_point_on_plane() {
        // Arrange
        let plane = Plane::new_xy();
        let point2 = pt2(-3.0, -5.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn result_lies_on_the_plane_for_tilted_plane() {
        // Arrange
        let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 1.0);
        let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let point2 = pt2(2.0, 3.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert!(plane.is_on_plane(&result));
    }

    #[test]
    fn origin_on_offset_plane_maps_to_anchor_point() {
        // Arrange
        let edge1 = edge(2.0, 3.0, 4.0, 3.0, 3.0, 4.0);
        let edge2 = edge(2.0, 3.0, 4.0, 2.0, 4.0, 4.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let point2 = pt2(0.0, 0.0);

        // Act
        let result = plane.point_from_2d(&point2);

        // Assert
        assert_relative_eq!(*result.x, 2.0, epsilon = 1e-5);
        assert_relative_eq!(*result.y, 3.0, epsilon = 1e-5);
        assert_relative_eq!(*result.z, 4.0, epsilon = 1e-5);
    }
}

mod normal_inverted {
    use super::*;

    #[test]
    fn inverted_normal_is_negated() {
        // Arrange
        let plane = Plane::new_xy();

        // Act
        let inverted = plane.normal_inverted();

        // Assert
        assert_relative_eq!(inverted.normal.x, -plane.normal.x, epsilon = 1e-5);
        assert_relative_eq!(inverted.normal.y, -plane.normal.y, epsilon = 1e-5);
        assert_relative_eq!(inverted.normal.z, -plane.normal.z, epsilon = 1e-5);
    }

    #[test]
    fn inverted_plane_keeps_same_anchor_point() {
        // Arrange
        let edge1 = edge(2.0, 3.0, 4.0, 3.0, 3.0, 4.0);
        let edge2 = edge(2.0, 3.0, 4.0, 2.0, 4.0, 4.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");

        // Act
        let inverted = plane.normal_inverted();

        // Assert
        assert_relative_eq!(*inverted.r0.x, *plane.r0.x, epsilon = 1e-5);
        assert_relative_eq!(*inverted.r0.y, *plane.r0.y, epsilon = 1e-5);
        assert_relative_eq!(*inverted.r0.z, *plane.r0.z, epsilon = 1e-5);
    }

    #[test]
    fn double_inversion_restores_original_normal() {
        // Arrange
        let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 1.0);
        let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");

        // Act
        let restored = plane.normal_inverted().normal_inverted();

        // Assert
        assert_relative_eq!(restored.normal.x, plane.normal.x, epsilon = 1e-5);
        assert_relative_eq!(restored.normal.y, plane.normal.y, epsilon = 1e-5);
        assert_relative_eq!(restored.normal.z, plane.normal.z, epsilon = 1e-5);
    }

    #[test]
    fn inverted_normal_remains_unit_length() {
        // Arrange
        let edge1 = edge(0.0, 0.0, 0.0, 1.0, 2.0, 0.0);
        let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 1.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");

        // Act
        let inverted = plane.normal_inverted();

        // Assert
        let norm = inverted.normal.norm2().sqrt();
        assert_relative_eq!(norm, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn points_on_original_plane_are_on_inverted_plane() {
        // Arrange
        let edge1 = edge(1.0, 1.0, 1.0, 2.0, 1.0, 1.0);
        let edge2 = edge(1.0, 1.0, 1.0, 1.0, 2.0, 1.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let point = p(5.0, 5.0, 1.0);

        // Act
        let inverted = plane.normal_inverted();

        // Assert
        assert!(plane.is_on_plane(&point));
        assert!(inverted.is_on_plane(&point));
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
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");

        // Act
        let result = plane.is_on_plane(&edge1.0);

        // Assert
        assert!(result);
    }

    #[test]
    fn returns_true_for_edge_end_point() {
        // Arrange
        let edge1 = edge(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let edge2 = edge(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");

        // Act
        let result = plane.is_on_plane(&edge1.1);

        // Assert
        assert!(result);
    }

    #[test]
    fn returns_true_for_point_on_custom_plane() {
        // Arrange
        let edge1 = edge(1.0, 1.0, 1.0, 2.0, 1.0, 1.0);
        let edge2 = edge(1.0, 1.0, 1.0, 1.0, 2.0, 1.0);
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
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
        let plane =
            Plane::new((&edge1.0, &edge1.1), (&edge2.0, &edge2.1)).expect("should create plane");
        let point = p(5.0, 5.0, 2.0); // z != 1

        // Act
        let result = plane.is_on_plane(&point);

        // Assert
        assert!(!result);
    }
}

mod plane_perspective {
    use super::*;

    type Perspective = PlanePerspective<DefaultEpsilon>;

    #[test]
    fn add_plane_returns_retrievable_id() {
        // Arrange
        let mut perspective = Perspective::new();
        let plane = Plane::new_xy();

        // Act
        let id = perspective.add_plane(plane);

        // Assert
        assert!(perspective.get(&id).is_some());
    }

    #[test]
    fn get_returns_none_for_unknown_id() {
        // Arrange
        let mut perspective = Perspective::new();
        let plane = Plane::new_xy();
        let id = perspective.add_plane(plane);

        // Act
        perspective.remove(&id);
        let result = perspective.get(&id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_returns_the_added_plane() {
        // Arrange
        let mut perspective = Perspective::new();
        let plane = Plane::new_xy();
        let expected_normal = *plane.normal;

        // Act
        let id = perspective.add_plane(plane);

        // Assert
        let retrieved = perspective.get(&id).expect("plane should exist");
        assert_relative_eq!(retrieved.normal.x, expected_normal.x, epsilon = 1e-5);
        assert_relative_eq!(retrieved.normal.y, expected_normal.y, epsilon = 1e-5);
        assert_relative_eq!(retrieved.normal.z, expected_normal.z, epsilon = 1e-5);
    }

    #[test]
    fn get_mut_allows_normal_update() {
        // Arrange
        let mut perspective = Perspective::new();
        let plane = Plane::new_xy();
        let id = perspective.add_plane(plane);

        // Act
        let retrieved = perspective.get_mut(&id).expect("plane should exist");
        *retrieved = Plane::new_yz();

        // Assert
        let updated = perspective.get(&id).expect("plane should still exist");
        assert_relative_eq!(updated.normal.x, 1.0, epsilon = 1e-5);
        assert_relative_eq!(updated.normal.y, 0.0, epsilon = 1e-5);
        assert_relative_eq!(updated.normal.z, 0.0, epsilon = 1e-5);
    }

    #[test]
    fn remove_returns_none_for_already_removed_id() {
        // Arrange
        let mut perspective = Perspective::new();
        let id = perspective.add_plane(Plane::new_xy());
        perspective.remove(&id);

        // Act
        let result = perspective.remove(&id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn remove_returns_the_plane() {
        // Arrange
        let mut perspective = Perspective::new();
        let plane = Plane::new_xy();
        let expected_normal = *plane.normal;
        let id = perspective.add_plane(plane);

        // Act
        let removed = perspective.remove(&id);

        // Assert
        let removed = removed.expect("should return the removed plane");
        assert_relative_eq!(removed.normal.x, expected_normal.x, epsilon = 1e-5);
        assert_relative_eq!(removed.normal.y, expected_normal.y, epsilon = 1e-5);
        assert_relative_eq!(removed.normal.z, expected_normal.z, epsilon = 1e-5);
    }

    #[test]
    fn multiple_planes_have_distinct_ids() {
        // Arrange
        let mut perspective = Perspective::new();

        // Act
        let id1 = perspective.add_plane(Plane::new_xy());
        let id2 = perspective.add_plane(Plane::new_xz());
        let id3 = perspective.add_plane(Plane::new_yz());

        // Assert
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn each_id_retrieves_its_own_plane() {
        // Arrange
        let mut perspective = Perspective::new();
        let id1 = perspective.add_plane(Plane::new_xy());
        let id2 = perspective.add_plane(Plane::new_yz());

        // Act & Assert
        let plane1 = perspective.get(&id1).expect("plane1 should exist");
        assert_relative_eq!(plane1.normal.z, 1.0, epsilon = 1e-5);

        let plane2 = perspective.get(&id2).expect("plane2 should exist");
        assert_relative_eq!(plane2.normal.x, 1.0, epsilon = 1e-5);
    }
}
