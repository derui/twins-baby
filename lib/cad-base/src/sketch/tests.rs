use super::*;
use pretty_assertions::assert_eq;

#[test]
fn sketch_builder_default_creates_with_default_values() {
    // Arrange & Act
    let builder = SketchBuilder::default();

    // Assert
    assert!(builder.attached_plane.is_none());
    assert!(builder.edge_id_gen.clone_box().generate() == EdgeId::new(1));
    assert!(builder.point_id_gen.clone_box().generate() == PointId::new(1));
}

#[test]
fn new_creates_sketch_with_attached_plane() {
    // Arrange
    let sketch_id = SketchId::new(1);
    let plane_id = PlaneId::new(10);
    let builder = SketchBuilder {
        attached_plane: Some(plane_id),
        ..Default::default()
    };

    // Act
    let result = Sketch::new(sketch_id, builder);

    // Assert
    let sketch = result.expect("should create sketch");
    assert_eq!(sketch.id, sketch_id);
    assert_eq!(sketch.attached_plane, plane_id);
    assert_eq!(sketch.points.len(), 0);
    assert_eq!(sketch.edges.len(), 0);
}

#[test]
fn new_fails_when_attached_plane_is_none() {
    // Arrange
    let sketch_id = SketchId::new(1);
    let builder = SketchBuilder {
        attached_plane: None,
        ..Default::default()
    };

    // Act
    let result = Sketch::new(sketch_id, builder);

    // Assert
    let err = result.expect_err("should fail without attached plane");
    assert!(err.to_string().contains("Must set attached_plane"));
}

#[test]
fn new_initializes_empty_collections() {
    // Arrange
    let sketch_id = SketchId::new(42);
    let builder = SketchBuilder {
        attached_plane: Some(PlaneId::new(1)),
        ..Default::default()
    };

    // Act
    let result = Sketch::new(sketch_id, builder);

    // Assert
    let sketch = result.expect("should create sketch");
    assert!(sketch.points.is_empty());
    assert!(sketch.edges.is_empty());
}

#[test]
fn new_uses_provided_id_generators() {
    // Arrange
    let sketch_id = SketchId::new(1);
    let edge_id_gen = Box::new(DefaultIdGenerator::<EdgeId>::default());
    let point_id_gen = Box::new(DefaultIdGenerator::<PointId>::default());
    let builder = SketchBuilder {
        edge_id_gen,
        point_id_gen,
        attached_plane: Some(PlaneId::new(5)),
    };

    // Act
    let result = Sketch::new(sketch_id, builder);

    // Assert
    let mut sketch = result.expect("should create sketch");
    assert_eq!(sketch.edge_id_gen.generate(), EdgeId::new(1));
    assert_eq!(sketch.point_id_gen.generate(), PointId::new(1));
}

#[test]
fn sketch_can_be_cloned() {
    // Arrange
    let sketch_id = SketchId::new(1);
    let plane_id = PlaneId::new(10);
    let builder = SketchBuilder {
        attached_plane: Some(plane_id),
        ..Default::default()
    };
    let sketch = Sketch::new(sketch_id, builder).expect("should create sketch");

    // Act
    let cloned = sketch.clone();

    // Assert
    assert_eq!(cloned.id, sketch.id);
    assert_eq!(cloned.attached_plane, sketch.attached_plane);
    assert_eq!(cloned.points.len(), sketch.points.len());
    assert_eq!(cloned.edges.len(), sketch.edges.len());
}
