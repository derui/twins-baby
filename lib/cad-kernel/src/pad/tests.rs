use cad_base::{
    feature::{
        AttachedTarget, Evaluate, EvaluateError, Feature, FeatureContext,
        operation::{Operation, Pad, PadDirection},
    },
    id::{PlaneId, SketchId},
    plane::Plane,
    sketch::{AttachableTarget, Geometry, LineSegment, Point2, Sketch},
};
use epsilon::DefaultEpsilon;
use pretty_assertions::assert_eq;
use solver::equation::Equation;

use super::PadKernel;

/// Create a pentagon sketch (5 edges, closed).
/// Pentagon produces a JordanCurve with 5 points and 5 edges,
fn make_pentagon_sketch() -> Sketch {
    let target = AttachableTarget::Plane(PlaneId::from(1));
    let mut sketch = Sketch::new("pentagon", &target);
    for (s, e) in [
        ((0.0_f32, 0.0_f32), (2.0_f32, 0.0_f32)),
        ((2.0, 0.0), (3.0, 1.0)),
        ((3.0, 1.0), (1.5, 2.0)),
        ((1.5, 2.0), (0.0, 1.0)),
        ((0.0, 1.0), (0.0, 0.0)),
    ] {
        sketch.add_geometry(|vars| {
            Geometry::LineSegment(LineSegment::from_points(
                &Point2::new(s.0, s.1),
                &Point2::new(e.0, e.1),
                vars,
            ))
        });
    }
    sketch
}

/// Create a sketch with an open path (not a closed Jordan curve).
fn make_open_sketch() -> Sketch {
    let target = AttachableTarget::Plane(PlaneId::from(1));
    let mut sketch = Sketch::new("open", &target);
    sketch.add_geometry(|vars| {
        Geometry::LineSegment(LineSegment::from_points(
            &Point2::new(0.0, 0.0),
            &Point2::new(1.0, 0.0),
            vars,
        ))
    });
    sketch.add_geometry(|vars| {
        Geometry::LineSegment(LineSegment::from_points(
            &Point2::new(1.0, 0.0),
            &Point2::new(1.0, 1.0),
            vars,
        ))
    });
    sketch
}

fn make_feature(eq_value: f32) -> Feature {
    let eq: Equation = eq_value.into();
    let op = Operation::Pad(Pad::new(&eq));
    Feature::new("Pad1", SketchId::from(1), &op).unwrap()
}

fn make_feature_with_direction(eq_value: f32, direction: &PadDirection) -> Feature {
    let eq: Equation = eq_value.into();
    let mut pad = Pad::new(&eq);
    pad.change_direction(direction);
    let op = Operation::Pad(pad);
    Feature::new("Pad1", SketchId::from(1), &op).unwrap()
}

fn make_context<'a>(sketch: &'a Sketch, plane: &'a Plane) -> FeatureContext<'a> {
    FeatureContext {
        sketches: vec![sketch].into(),
        target: vec![AttachedTarget::Plane(plane)].into(),
    }
}

#[test]
fn returns_error_when_no_sketches() {
    // Arrange
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature(5.0);
    let context = FeatureContext {
        sketches: vec![].into(),
        target: vec![AttachedTarget::Plane(&plane)].into(),
    };

    // Act
    let result = PadKernel::evaluate(&feature, &context);

    // Assert
    assert!(matches!(result, Err(EvaluateError::InsufficientSketch)));
}

#[test]
fn returns_error_when_sketch_is_not_closed() {
    // Arrange
    let sketch = make_open_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature(5.0);
    let context = make_context(&sketch, &plane);

    // Act
    let result = PadKernel::evaluate(&feature, &context);

    // Assert
    assert!(matches!(
        result,
        Err(EvaluateError::HaveSomeInvalidSketches(_))
    ));
}

#[test]
fn normal_pad_produces_one_solid() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature(5.0);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    assert_eq!(solids.len(), 1);
}

#[test]
fn normal_pad_solid_has_correct_topology() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature(5.0);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    let solid = &solids[0];
    // Pentagon prism: 2 top/bottom faces + 4 surrounding faces = 6 faces, 10 vertices
    assert_eq!(solid.faces.len(), 6);
    assert_eq!(solid.vertices.len(), 10);
}

#[test]
fn inverted_normal_pad_produces_one_solid() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature_with_direction(5.0, &PadDirection::InveredNormal);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    assert_eq!(solids.len(), 1);
    assert_eq!(solids[0].vertices.len(), 10);
}

#[test]
fn symmetric_pad_produces_one_solid() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature_with_direction(5.0, &PadDirection::Symmetric);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    assert_eq!(solids.len(), 1);
    assert_eq!(solids[0].vertices.len(), 10);
}

#[test]
fn normal_pad_vertices_are_offset_along_normal() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let length = 5.0_f32;
    let feature = make_feature(length);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    // XZ plane normal is (0,1,0). First face stays at y=0, second face at y=length.
    let solid = &solids[0];
    let y_values: Vec<f32> = solid.vertices.values().map(|v| *v.y).collect();
    for y in &y_values {
        assert!(
            (*y - 0.0).abs() < 1e-5 || (*y - length).abs() < 1e-5,
            "vertex y={y} should be 0.0 or {length}"
        );
    }
}

#[test]
fn symmetric_pad_vertices_are_offset_both_directions() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let length = 5.0_f32;
    let feature = make_feature_with_direction(length, &PadDirection::Symmetric);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    // Symmetric: first face offset along +normal (y=+length), second along -normal (y=-length).
    let solid = &solids[0];
    let y_values: Vec<f32> = solid.vertices.values().map(|v| *v.y).collect();
    for y in &y_values {
        assert!(
            (*y - length).abs() < 1e-5 || (*y + length).abs() < 1e-5,
            "vertex y={y} should be {length} or -{length}"
        );
    }
}

#[test]
fn inverted_normal_pad_vertices_are_offset_in_negative_direction() {
    // Arrange
    let sketch = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let length = 5.0_f32;
    let feature = make_feature_with_direction(length, &PadDirection::InveredNormal);
    let context = make_context(&sketch, &plane);

    // Act
    let solids = PadKernel::evaluate(&feature, &context).unwrap();

    // Assert
    // InveredNormal: first face at y=0, second face offset along -normal (y=-length).
    let solid = &solids[0];
    let y_values: Vec<f32> = solid.vertices.values().map(|v| *v.y).collect();
    for y in &y_values {
        assert!(
            (*y - 0.0).abs() < 1e-5 || (*y + length).abs() < 1e-5,
            "vertex y={y} should be 0.0 or -{length}"
        );
    }
}

#[test]
fn returns_error_when_multiple_sketches() {
    // Arrange
    let sketch1 = make_pentagon_sketch();
    let sketch2 = make_pentagon_sketch();
    let plane = Plane::<DefaultEpsilon>::new_xz();
    let feature = make_feature(5.0);
    let context = FeatureContext {
        sketches: vec![&sketch1, &sketch2].into(),
        target: vec![AttachedTarget::Plane(&plane)].into(),
    };

    // Act
    let result = PadKernel::evaluate(&feature, &context);

    // Assert
    assert!(matches!(result, Err(EvaluateError::InsufficientSketch)));
}
