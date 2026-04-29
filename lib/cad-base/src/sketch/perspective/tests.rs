use super::*;
use crate::body::BodyPerspective;
use crate::sketch::AttachableTarget;

fn make_plane_ref() -> (BodyPerspective, crate::body::PlaneRef) {
    let mut bodies = BodyPerspective::new();
    let body_id = bodies.add_body();
    let plane_ref = bodies.as_x_plane_ref(&body_id).unwrap();
    (bodies, plane_ref)
}

mod sketch_perspective {
    use super::*;

    mod construction {
        use super::*;

        #[test]
        fn new_creates_empty_perspective() {
            // Arrange & Act
            let perspective = SketchPerspective::new();

            // Assert
            assert!(perspective.get(&SketchId::new(1)).is_none());
        }
    }

    mod get_sketch {
        use super::*;

        #[test]
        fn get_returns_some_for_existing_sketch() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Act
            let result = perspective.get(&sketch_id);

            // Assert
            assert!(result.is_some());
        }

        #[test]
        fn get_returns_none_for_nonexistent_sketch() {
            // Arrange
            let perspective = SketchPerspective::new();
            let nonexistent_id = SketchId::new(999);

            // Act
            let result = perspective.get(&nonexistent_id);

            // Assert
            assert!(result.is_none());
        }

        #[test]
        fn get_mut_returns_some_for_existing_sketch() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Act
            let result = perspective.get_mut(&sketch_id);

            // Assert
            assert!(result.is_some());
        }

        #[test]
        fn get_mut_returns_none_for_nonexistent_sketch() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let nonexistent_id = SketchId::new(999);

            // Act
            let result = perspective.get_mut(&nonexistent_id);

            // Assert
            assert!(result.is_none());
        }
    }

    mod add_sketch {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn add_sketch_returns_valid_id() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();

            // Act
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Assert
            assert!(perspective.get(&sketch_id).is_some());
        }

        #[test]
        fn add_sketch_generates_unique_ids() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();

            // Act
            let sketch_id1 = perspective.add_sketch(&plane_ref);
            let sketch_id2 = perspective.add_sketch(&plane_ref);

            // Assert
            assert_ne!(sketch_id1, sketch_id2);
        }

        #[test]
        fn add_sketch_creates_sketch_with_correct_plane() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();

            // Act
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Assert
            let sketch = perspective.get(&sketch_id).unwrap();
            assert_eq!(*sketch.attach_target, AttachableTarget::Plane(plane_ref));
        }
    }

    mod remove_sketch {
        use super::*;

        #[test]
        fn remove_sketch_returns_removed_sketch() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Act
            let result = perspective.remove_sketch(&sketch_id);

            // Assert
            assert!(result.is_some());
            assert!(perspective.get(&sketch_id).is_none());
        }

        #[test]
        fn remove_sketch_returns_none_for_nonexistent() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let nonexistent_id = SketchId::new(999);

            // Act
            let result = perspective.remove_sketch(&nonexistent_id);

            // Assert
            assert!(result.is_none());
        }

        #[test]
        fn remove_sketch_does_not_affect_other_sketches() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id1 = perspective.add_sketch(&plane_ref);
            let sketch_id2 = perspective.add_sketch(&plane_ref);
            let sketch_id3 = perspective.add_sketch(&plane_ref);

            // Act
            perspective.remove_sketch(&sketch_id2);

            // Assert
            assert!(perspective.get(&sketch_id1).is_some());
            assert!(perspective.get(&sketch_id2).is_none());
            assert!(perspective.get(&sketch_id3).is_some());
        }
    }

    mod rename_sketch {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn rename_sketch_succeeds_with_valid_name() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);
            let new_name = "NewSketchName";

            // Act
            let result = perspective.remane_sketch(&sketch_id, new_name);

            // Assert
            assert!(result.is_ok());
            let sketch = perspective.get(&sketch_id).unwrap();
            assert_eq!(sketch.name.as_str(), new_name);
        }

        #[test]
        fn rename_sketch_fails_with_empty_name() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Act
            let result = perspective.remane_sketch(&sketch_id, "");

            // Assert
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "Do not allow empty string");
        }

        #[test]
        fn rename_sketch_fails_with_whitespace_only_name() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id = perspective.add_sketch(&plane_ref);

            // Act
            let result = perspective.remane_sketch(&sketch_id, "   ");

            // Assert
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "Do not allow empty string");
        }

        #[test]
        fn rename_sketch_fails_with_duplicate_name() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let (_bodies, plane_ref) = make_plane_ref();
            let sketch_id1 = perspective.add_sketch(&plane_ref);
            let sketch_id2 = perspective.add_sketch(&plane_ref);
            let duplicate_name = "DuplicateName";
            perspective
                .remane_sketch(&sketch_id1, duplicate_name)
                .unwrap();

            // Act
            let result = perspective.remane_sketch(&sketch_id2, duplicate_name);

            // Assert
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                format!("Sketch with name '{}' already exists", duplicate_name)
            );
        }

        #[test]
        fn rename_sketch_fails_for_nonexistent_sketch() {
            // Arrange
            let mut perspective = SketchPerspective::new();
            let nonexistent_id = SketchId::new(999);

            // Act
            let result = perspective.remane_sketch(&nonexistent_id, "SomeName");

            // Assert
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                format!("Sketch with id {} not found", nonexistent_id)
            );
        }
    }
}
