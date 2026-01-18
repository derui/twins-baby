use super::*;
use crate::point::Point;
use anyhow::{Context, Result};
use approx::assert_relative_eq;
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
fn new_creates_sketch_with_attached_plane() -> Result<()> {
    // Arrange
    let sketch_id = SketchId::new(1);
    let plane_id = PlaneId::new(10);
    let builder = SketchBuilder {
        attached_plane: Some(plane_id),
        ..Default::default()
    };

    // Act
    let sketch = Sketch::new(sketch_id, builder)?;

    // Assert
    assert_eq!(sketch.id, sketch_id);
    assert_eq!(sketch.attached_plane, plane_id);
    assert_eq!(sketch.points.len(), 0);
    assert_eq!(sketch.edges.len(), 0);

    Ok(())
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
fn new_initializes_empty_collections() -> Result<()> {
    // Arrange
    let sketch_id = SketchId::new(42);
    let builder = SketchBuilder {
        attached_plane: Some(PlaneId::new(1)),
        ..Default::default()
    };

    // Act
    let sketch = Sketch::new(sketch_id, builder)?;

    // Assert
    assert!(sketch.points.is_empty());
    assert!(sketch.edges.is_empty());

    Ok(())
}

#[test]
fn new_uses_provided_id_generators() -> Result<()> {
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
    let mut sketch = Sketch::new(sketch_id, builder)?;

    // Assert
    assert_eq!(sketch.edge_id_gen.generate(), EdgeId::new(1));
    assert_eq!(sketch.point_id_gen.generate(), PointId::new(1));

    Ok(())
}

#[test]
fn sketch_can_be_cloned() -> Result<()> {
    // Arrange
    let sketch_id = SketchId::new(1);
    let plane_id = PlaneId::new(10);
    let builder = SketchBuilder {
        attached_plane: Some(plane_id),
        ..Default::default()
    };
    let sketch = Sketch::new(sketch_id, builder)?;

    // Act
    let cloned = sketch.clone();

    // Assert
    assert_eq!(cloned.id, sketch.id);
    assert_eq!(cloned.attached_plane, sketch.attached_plane);
    assert_eq!(cloned.points.len(), sketch.points.len());
    assert_eq!(cloned.edges.len(), sketch.edges.len());

    Ok(())
}

#[test]
fn add_point_stores_point_in_sketch() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(1.0, 2.0, 3.0);

    // Act
    sketch.add_point(&point);

    // Assert
    assert_eq!(sketch.points.len(), 1);

    Ok(())
}

#[test]
fn add_point_creates_xyz_variables_in_environment() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(10.0, 20.0, 30.0);

    // Act
    sketch.add_point(&point);

    // Assert
    let x_var = sketch.variables.get_variable("x1");
    let y_var = sketch.variables.get_variable("y1");
    let z_var = sketch.variables.get_variable("z1");
    assert!(x_var.is_some());
    assert!(y_var.is_some());
    assert!(z_var.is_some());

    Ok(())
}

#[test]
fn add_point_variables_have_correct_values() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(10.0, 20.0, 30.0);

    // Act
    sketch.add_point(&point);

    // Assert
    let x_var = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let y_var = sketch
        .variables
        .get_variable("y1")
        .context("y1 not found")?;
    let z_var = sketch
        .variables
        .get_variable("z1")
        .context("z1 not found")?;
    assert_relative_eq!(x_var.value(), 10.0);
    assert_relative_eq!(y_var.value(), 20.0);
    assert_relative_eq!(z_var.value(), 30.0);

    Ok(())
}

#[test]
fn add_point_multiple_points_have_sequential_ids() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(4.0, 5.0, 6.0);

    // Act
    sketch.add_point(&point1);
    sketch.add_point(&point2);

    // Assert
    assert_eq!(sketch.points.len(), 2);
    assert!(sketch.variables.get_variable("x1").is_some());
    assert!(sketch.variables.get_variable("x2").is_some());

    Ok(())
}

#[test]
fn add_point_multiple_points_have_separate_variables() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(10.0, 20.0, 30.0);

    // Act
    sketch.add_point(&point1);
    sketch.add_point(&point2);

    // Assert
    let x1 = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let x2 = sketch
        .variables
        .get_variable("x2")
        .context("x2 not found")?;
    assert_relative_eq!(x1.value(), 1.0);
    assert_relative_eq!(x2.value(), 10.0);

    Ok(())
}

#[test]
fn add_point_with_zero_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::zero();

    // Act
    sketch.add_point(&point);

    // Assert
    let x_var = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let y_var = sketch
        .variables
        .get_variable("y1")
        .context("y1 not found")?;
    let z_var = sketch
        .variables
        .get_variable("z1")
        .context("z1 not found")?;
    assert_relative_eq!(x_var.value(), 0.0);
    assert_relative_eq!(y_var.value(), 0.0);
    assert_relative_eq!(z_var.value(), 0.0);

    Ok(())
}

#[test]
fn add_point_with_negative_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(-5.0, -10.0, -15.0);

    // Act
    sketch.add_point(&point);

    // Assert
    let x_var = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let y_var = sketch
        .variables
        .get_variable("y1")
        .context("y1 not found")?;
    let z_var = sketch
        .variables
        .get_variable("z1")
        .context("z1 not found")?;
    assert_relative_eq!(x_var.value(), -5.0);
    assert_relative_eq!(y_var.value(), -10.0);
    assert_relative_eq!(z_var.value(), -15.0);

    Ok(())
}

#[test]
fn remove_point_returns_removed_point() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(10.0, 20.0, 30.0);
    let point_id = sketch.add_point(&point);

    // Act
    let removed_point = sketch.remove_point(&point_id);

    // Assert
    let removed = removed_point.context("should return removed point")?;
    assert_relative_eq!(*removed.x(), 10.0);
    assert_relative_eq!(*removed.y(), 20.0);
    assert_relative_eq!(*removed.z(), 30.0);

    Ok(())
}

#[test]
fn remove_point_returns_none_for_nonexistent_point() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let nonexistent_id = PointId::new(999);

    // Act
    let result = sketch.remove_point(&nonexistent_id);

    // Assert
    assert!(result.is_none());

    Ok(())
}

#[test]
fn remove_point_from_empty_sketch_returns_none() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point_id = PointId::new(1);

    // Act
    let result = sketch.remove_point(&point_id);

    // Assert
    assert!(result.is_none());

    Ok(())
}

#[test]
fn remove_point_removes_point_so_second_removal_returns_none() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(1.0, 2.0, 3.0);
    let point_id = sketch.add_point(&point);

    // Act
    let first_removal = sketch.remove_point(&point_id);
    let second_removal = sketch.remove_point(&point_id);

    // Assert
    assert!(first_removal.is_some());
    assert!(second_removal.is_none());

    Ok(())
}

#[test]
fn remove_point_with_zero_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::zero();
    let point_id = sketch.add_point(&point);

    // Act
    let removed_point = sketch.remove_point(&point_id);

    // Assert
    let removed = removed_point.context("should return removed point")?;
    assert_relative_eq!(*removed.x(), 0.0);
    assert_relative_eq!(*removed.y(), 0.0);
    assert_relative_eq!(*removed.z(), 0.0);

    Ok(())
}

#[test]
fn remove_point_with_negative_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point = Point::new(-5.0, -10.0, -15.0);
    let point_id = sketch.add_point(&point);

    // Act
    let removed_point = sketch.remove_point(&point_id);

    // Assert
    let removed = removed_point.context("should return removed point")?;
    assert_relative_eq!(*removed.x(), -5.0);
    assert_relative_eq!(*removed.y(), -10.0);
    assert_relative_eq!(*removed.z(), -15.0);

    Ok(())
}

#[test]
fn remove_point_only_removes_specified_point() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(10.0, 20.0, 30.0);
    let point3 = Point::new(100.0, 200.0, 300.0);
    let id1 = sketch.add_point(&point1);
    let id2 = sketch.add_point(&point2);
    let id3 = sketch.add_point(&point3);

    // Act
    let removed = sketch.remove_point(&id2);

    // Assert
    let removed_point = removed.context("should return removed point")?;
    assert_relative_eq!(*removed_point.x(), 10.0);
    assert_relative_eq!(*removed_point.y(), 20.0);
    assert_relative_eq!(*removed_point.z(), 30.0);
    assert!(sketch.remove_point(&id1).is_some());
    assert!(sketch.remove_point(&id2).is_none());
    assert!(sketch.remove_point(&id3).is_some());

    Ok(())
}

#[test]
fn remove_point_can_remove_all_points_sequentially() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(10.0, 20.0, 30.0);
    let id1 = sketch.add_point(&point1);
    let id2 = sketch.add_point(&point2);

    // Act
    let removed1 = sketch.remove_point(&id1);
    let removed2 = sketch.remove_point(&id2);

    // Assert
    assert!(removed1.is_some());
    assert!(removed2.is_some());
    assert!(sketch.remove_point(&id1).is_none());
    assert!(sketch.remove_point(&id2).is_none());

    Ok(())
}

#[test]
fn add_edge_stores_edge_in_sketch() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    assert_eq!(sketch.edges.len(), 1);

    Ok(())
}

#[test]
fn add_edge_creates_start_and_end_points() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    assert_eq!(sketch.points.len(), 2);

    Ok(())
}

#[test]
fn add_edge_creates_xyz_variables_for_both_points() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(10.0, 20.0, 30.0), Point::new(40.0, 50.0, 60.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    assert!(sketch.variables.get_variable("x1").is_some());
    assert!(sketch.variables.get_variable("y1").is_some());
    assert!(sketch.variables.get_variable("z1").is_some());
    assert!(sketch.variables.get_variable("x2").is_some());
    assert!(sketch.variables.get_variable("y2").is_some());
    assert!(sketch.variables.get_variable("z2").is_some());

    Ok(())
}

#[test]
fn add_edge_creates_length_variable() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 4.0, 0.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    let length_var = sketch
        .variables
        .get_variable("edge_1")
        .context("edge_1 variable not found")?;
    assert_relative_eq!(length_var.value(), 5.0);

    Ok(())
}

#[test]
fn add_edge_multiple_edges_have_sequential_ids() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge1 = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0))?;
    let edge2 = Edge::new(Point::new(2.0, 0.0, 0.0), Point::new(3.0, 0.0, 0.0))?;

    // Act
    sketch.add_edge(&edge1);
    sketch.add_edge(&edge2);

    // Assert
    assert_eq!(sketch.edges.len(), 2);
    assert!(sketch.variables.get_variable("edge_1").is_some());
    assert!(sketch.variables.get_variable("edge_2").is_some());

    Ok(())
}

#[test]
fn add_edge_with_zero_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::zero(), Point::new(1.0, 0.0, 0.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    let x1 = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let y1 = sketch
        .variables
        .get_variable("y1")
        .context("y1 not found")?;
    let z1 = sketch
        .variables
        .get_variable("z1")
        .context("z1 not found")?;
    assert_relative_eq!(x1.value(), 0.0);
    assert_relative_eq!(y1.value(), 0.0);
    assert_relative_eq!(z1.value(), 0.0);

    Ok(())
}

#[test]
fn add_edge_with_negative_coordinates() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(-1.0, -2.0, -3.0), Point::new(-4.0, -5.0, -6.0))?;

    // Act
    sketch.add_edge(&edge);

    // Assert
    let x1 = sketch
        .variables
        .get_variable("x1")
        .context("x1 not found")?;
    let y1 = sketch
        .variables
        .get_variable("y1")
        .context("y1 not found")?;
    let z1 = sketch
        .variables
        .get_variable("z1")
        .context("z1 not found")?;
    assert_relative_eq!(x1.value(), -1.0);
    assert_relative_eq!(y1.value(), -2.0);
    assert_relative_eq!(z1.value(), -3.0);

    Ok(())
}

#[test]
fn remove_edge_returns_removed_edge() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 4.0, 0.0))?;
    let edge_id = sketch.add_edge(&edge);

    // Act
    let removed_edge = sketch.remove_edge(&edge_id);

    // Assert
    let removed = removed_edge.context("should return removed edge")?;
    assert_relative_eq!(*removed.start().x(), 0.0);
    assert_relative_eq!(*removed.start().y(), 0.0);
    assert_relative_eq!(*removed.start().z(), 0.0);
    assert_relative_eq!(*removed.end().x(), 3.0);
    assert_relative_eq!(*removed.end().y(), 4.0);
    assert_relative_eq!(*removed.end().z(), 0.0);

    Ok(())
}

#[test]
fn remove_edge_returns_none_for_nonexistent_edge() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let nonexistent_id = EdgeId::new(999);

    // Act
    let result = sketch.remove_edge(&nonexistent_id);

    // Assert
    assert!(result.is_none());

    Ok(())
}

#[test]
fn remove_edge_from_empty_sketch_returns_none() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge_id = EdgeId::new(1);

    // Act
    let result = sketch.remove_edge(&edge_id);

    // Assert
    assert!(result.is_none());

    Ok(())
}

#[test]
fn remove_edge_removes_associated_points() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0))?;
    let edge_id = sketch.add_edge(&edge);

    // Act
    let removed = sketch.remove_edge(&edge_id);

    // Assert
    assert!(removed.is_some());
    assert_eq!(sketch.points.len(), 0);

    Ok(())
}

#[test]
fn remove_edge_removes_all_variables() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0))?;
    let edge_id = sketch.add_edge(&edge);

    // Act
    sketch.remove_edge(&edge_id);

    // Assert
    assert!(sketch.variables.get_variable("x1").is_none());
    assert!(sketch.variables.get_variable("y1").is_none());
    assert!(sketch.variables.get_variable("z1").is_none());
    assert!(sketch.variables.get_variable("x2").is_none());
    assert!(sketch.variables.get_variable("y2").is_none());
    assert!(sketch.variables.get_variable("z2").is_none());
    assert!(sketch.variables.get_variable("edge_1").is_none());

    Ok(())
}

#[test]
fn remove_edge_second_removal_returns_none() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge = Edge::new(Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0))?;
    let edge_id = sketch.add_edge(&edge);

    // Act
    let first_removal = sketch.remove_edge(&edge_id);
    let second_removal = sketch.remove_edge(&edge_id);

    // Assert
    assert!(first_removal.is_some());
    assert!(second_removal.is_none());

    Ok(())
}

#[test]
fn remove_edge_only_removes_specified_edge() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge1 = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0))?;
    let edge2 = Edge::new(Point::new(2.0, 0.0, 0.0), Point::new(3.0, 0.0, 0.0))?;
    let edge3 = Edge::new(Point::new(4.0, 0.0, 0.0), Point::new(5.0, 0.0, 0.0))?;
    let id1 = sketch.add_edge(&edge1);
    let id2 = sketch.add_edge(&edge2);
    let id3 = sketch.add_edge(&edge3);

    // Act
    let removed = sketch.remove_edge(&id2);

    // Assert
    let removed_edge = removed.context("should return removed edge")?;
    assert_relative_eq!(*removed_edge.start().x(), 2.0);
    assert_relative_eq!(*removed_edge.end().x(), 3.0);
    assert!(sketch.remove_edge(&id1).is_some());
    assert!(sketch.remove_edge(&id2).is_none());
    assert!(sketch.remove_edge(&id3).is_some());

    Ok(())
}

#[test]
fn remove_edge_can_remove_all_edges_sequentially() -> Result<()> {
    // Arrange
    let mut sketch = Sketch::new(
        SketchId::new(1),
        SketchBuilder {
            attached_plane: Some(PlaneId::new(1)),
            ..Default::default()
        },
    )?;
    let edge1 = Edge::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0))?;
    let edge2 = Edge::new(Point::new(2.0, 0.0, 0.0), Point::new(3.0, 0.0, 0.0))?;
    let id1 = sketch.add_edge(&edge1);
    let id2 = sketch.add_edge(&edge2);

    // Act
    let removed1 = sketch.remove_edge(&id1);
    let removed2 = sketch.remove_edge(&id2);

    // Assert
    assert!(removed1.is_some());
    assert!(removed2.is_some());
    assert!(sketch.remove_edge(&id1).is_none());
    assert!(sketch.remove_edge(&id2).is_none());

    Ok(())
}
