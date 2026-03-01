use super::*;

mod body {
    use pretty_assertions::assert_eq;

    use crate::plane::Plane;
    use crate::vector3::Vector3;

    use super::*;

    #[test]
    fn new_creates_body_with_given_name() {
        // Arrange & Act
        let body = Body::new("TestBody".to_string());

        // Assert
        assert_eq!(*body.name, "TestBody");
    }

    #[test]
    fn new_creates_body_with_default_planes() {
        // Arrange & Act
        let body = Body::new("TestBody".to_string());

        // Assert
        assert_eq!(*body.x_plane, Plane::new_yz());
        assert_eq!(*body.y_plane, Plane::new_xz());
        assert_eq!(*body.z_plane, Plane::new_xy());
    }

    #[test]
    fn new_creates_body_at_origin() {
        // Arrange & Act
        let body = Body::new("TestBody".to_string());

        // Assert
        assert_eq!(*body.position, Vector3::new(0.0, 0.0, 0.0));
    }
}

mod body_perspective {
    use pretty_assertions::assert_eq;

    use crate::id::BodyId;
    use crate::vector3::Vector3;
    use immutable::Im;

    use super::*;

    #[test]
    fn new_creates_empty_perspective() {
        // Arrange & Act
        let perspective = BodyPerspective::new();

        // Assert
        assert_eq!(perspective.bodies.len(), 0);
    }

    #[test]
    fn add_body_returns_unique_ids() {
        // Arrange
        let mut perspective = BodyPerspective::new();

        // Act
        let id1 = perspective.add_body();
        let id2 = perspective.add_body();

        // Assert
        assert_ne!(id1, id2);
    }

    #[test]
    fn add_body_makes_body_retrievable() {
        // Arrange
        let mut perspective = BodyPerspective::new();

        // Act
        let id = perspective.add_body();

        // Assert
        assert!(perspective.get(&id).is_some());
    }

    #[test]
    fn add_body_assigns_name_from_id() {
        // Arrange
        let mut perspective = BodyPerspective::new();

        // Act
        let id = perspective.add_body();

        // Assert
        let body = perspective.get(&id).unwrap();
        assert_eq!(*body.name, format!("{}", id));
    }

    #[test]
    fn get_returns_none_for_unknown_id() {
        // Arrange
        let perspective = BodyPerspective::new();
        let unknown_id = BodyId::new(999);

        // Act
        let result = perspective.get(&unknown_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_mut_allows_mutation() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();
        let new_position = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let body = perspective.get_mut(&id).unwrap();
        body.position = Im::new(new_position);

        // Assert
        let body = perspective.get(&id).unwrap();
        assert_eq!(*body.position, new_position);
    }

    #[test]
    fn get_mut_returns_none_for_unknown_id() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let unknown_id = BodyId::new(999);

        // Act
        let result = perspective.get_mut(&unknown_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn remove_body_returns_removed_body() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();

        // Act
        let removed = perspective.remove_body(&id);

        // Assert
        assert!(removed.is_some());
    }

    #[test]
    fn remove_body_makes_body_unretrievable() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();

        // Act
        perspective.remove_body(&id);

        // Assert
        assert!(perspective.get(&id).is_none());
    }

    #[test]
    fn remove_body_returns_none_for_unknown_id() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let unknown_id = BodyId::new(999);

        // Act
        let result = perspective.remove_body(&unknown_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn rename_body_returns_old_name_on_success() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();
        let old_name = (*perspective.get(&id).unwrap().name).clone();

        // Act
        let result = perspective.rename_body(&id, "NewName");

        // Assert
        assert_eq!(result.unwrap(), old_name);
    }

    #[test]
    fn rename_body_updates_name() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();

        // Act
        perspective.rename_body(&id, "Renamed").unwrap();

        // Assert
        assert_eq!(*perspective.get(&id).unwrap().name, "Renamed");
    }

    #[test]
    fn rename_body_fails_for_unknown_id() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let unknown_id = BodyId::new(999);

        // Act
        let result = perspective.rename_body(&unknown_id, "AnyName");

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn rename_body_fails_when_name_already_used_by_another_body() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id1 = perspective.add_body();
        let id2 = perspective.add_body();
        perspective.rename_body(&id1, "Taken").unwrap();

        // Act
        let result = perspective.rename_body(&id2, "Taken");

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn rename_body_allows_same_name_for_same_body() {
        // Arrange
        let mut perspective = BodyPerspective::new();
        let id = perspective.add_body();
        perspective.rename_body(&id, "SameName").unwrap();

        // Act - rename to the same name it already has
        let result = perspective.rename_body(&id, "SameName");

        // Assert
        assert!(result.is_ok());
    }
}
