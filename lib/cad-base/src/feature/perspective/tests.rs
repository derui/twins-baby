use pretty_assertions::assert_eq;
use rstest::rstest;
use solver::equation::Equation;

use crate::feature::operation::{Operation, Pad};
use crate::feature::{Evaluate, EvaluateError, Feature, FeatureContext};
use crate::id::{BodyId, FeatureId, SketchId, SolidId};
use crate::solid::{Solid, SolidBuilder};

use super::FeaturePerspective;

fn make_operation() -> Operation {
    let eq: Equation = 10.0.into();
    Pad::new(&eq).into()
}

fn make_sketch_id() -> SketchId {
    SketchId::from(1)
}

fn make_body_id() -> BodyId {
    BodyId::from(2)
}

fn make_context<'a>() -> FeatureContext<'a> {
    FeatureContext {
        sketches: vec![].into(),
        target: vec![].into(),
    }
}

struct OneSolidEvaluator;
impl Evaluate for OneSolidEvaluator {
    fn evaluate<'a>(
        _feature: &Feature,
        _context: &FeatureContext<'a>,
    ) -> Result<Vec<Solid>, EvaluateError> {
        Ok(vec![SolidBuilder::default().build()])
    }
}

fn solid_id_of(perspective: &FeaturePerspective, feature_id: &FeatureId) -> SolidId {
    *(*perspective.get(feature_id).unwrap().solids)
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap()
}

#[test]
fn test_evaluate_feature_returns_feature_not_found_for_missing_id() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let nonexistent_id = FeatureId::from(999);
    let context = make_context();

    // Act
    let result = perspective.evaluate_feature::<OneSolidEvaluator>(&nonexistent_id, &context);

    // Assert
    assert!(matches!(result, Err(EvaluateError::FeatureNotFound)));
}

#[test]
fn test_evaluate_feature_mints_unique_solid_ids_across_features() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let sketch = make_sketch_id();
    let body = make_body_id();
    let op = make_operation();
    let id1 = perspective.add_feature(body, sketch, &op);
    let id2 = perspective.add_feature(body, sketch, &op);
    let context = make_context();

    // Act
    perspective
        .evaluate_feature::<OneSolidEvaluator>(&id1, &context)
        .unwrap();
    perspective
        .evaluate_feature::<OneSolidEvaluator>(&id2, &context)
        .unwrap();

    // Assert
    let solid_id1 = solid_id_of(&perspective, &id1);
    let solid_id2 = solid_id_of(&perspective, &id2);
    assert_ne!(solid_id1, solid_id2);
}

#[test]
fn test_read_solid_returns_none_before_evaluation() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    perspective.add_feature(body, sketch, &op);

    // Act
    let result = perspective.read_solid(SolidId::from(1));

    // Assert
    assert!(result.is_none());
}

#[test]
fn test_read_solid_returns_solid_after_evaluation() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);
    let context = make_context();
    perspective
        .evaluate_feature::<OneSolidEvaluator>(&id, &context)
        .unwrap();
    let solid_id = solid_id_of(&perspective, &id);

    // Act
    let result = perspective.read_solid(solid_id);

    // Assert
    assert!(result.is_some());
}

#[test]
fn test_read_solid_returns_none_for_unknown_id() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);
    let context = make_context();
    perspective
        .evaluate_feature::<OneSolidEvaluator>(&id, &context)
        .unwrap();

    // Act
    let result = perspective.read_solid(SolidId::from(999));

    // Assert
    assert!(result.is_none());
}

#[test]
fn test_add_feature_returns_retrievable_feature() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();

    // Act
    let id = perspective.add_feature(body, sketch, &op);

    // Assert
    assert!(perspective.get(&id).is_some());
}

#[test]
fn test_add_multiple_features_generates_unique_ids() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();

    // Act
    let id1 = perspective.add_feature(body, sketch, &op);
    let id2 = perspective.add_feature(body, sketch, &op);

    // Assert
    assert_ne!(id1, id2);
    assert!(perspective.get(&id1).is_some());
    assert!(perspective.get(&id2).is_some());
}

#[test]
fn test_get_returns_none_for_missing_id() {
    // Arrange
    let perspective = FeaturePerspective::new();
    let nonexistent_id = FeatureId::from(999);

    // Act
    let result = perspective.get(&nonexistent_id);

    // Assert
    assert!(result.is_none());
}

#[test]
fn test_get_mut_returns_none_for_missing_id() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let nonexistent_id = FeatureId::from(999);

    // Act
    let result = perspective.get_mut(&nonexistent_id);

    // Assert
    assert!(result.is_none());
}

#[test]
fn test_remove_feature_returns_the_feature() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);

    // Act
    let removed = perspective.remove_feature(&id);

    // Assert
    assert!(removed.is_some());
}

#[test]
fn test_remove_feature_makes_feature_inaccessible() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);

    // Act
    perspective.remove_feature(&id);

    // Assert
    assert!(perspective.get(&id).is_none());
}

#[test]
fn test_remove_feature_returns_none_for_missing_id() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let nonexistent_id = FeatureId::from(999);

    // Act
    let result = perspective.remove_feature(&nonexistent_id);

    // Assert
    assert!(result.is_none());
}

#[test]
fn test_rename_feature_updates_name() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);

    // Act
    let result = perspective.rename_feature(&id, "NewName");

    // Assert
    assert!(result.is_ok());
    assert_eq!(*perspective.get(&id).unwrap().name, "NewName");
}

#[test]
fn test_rename_feature_fails_for_duplicate_name() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id1 = perspective.add_feature(body, sketch, &op);
    let id2 = perspective.add_feature(body, sketch, &op);
    perspective.rename_feature(&id1, "Existing").unwrap();

    // Act
    let result = perspective.rename_feature(&id2, "Existing");

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_rename_feature_fails_for_missing_id() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let nonexistent_id = FeatureId::from(999);

    // Act
    let result = perspective.rename_feature(&nonexistent_id, "SomeName");

    // Assert
    assert!(result.is_err());
}

#[rstest]
#[case("")]
#[case("   ")]
#[case("\t")]
fn test_rename_feature_fails_for_blank_name(#[case] name: &str) {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);

    // Act
    let result = perspective.rename_feature(&id, name);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_rename_feature_trims_whitespace() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let body = make_body_id();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(body, sketch, &op);

    // Act
    let result = perspective.rename_feature(&id, "  Trimmed  ");

    // Assert
    assert!(result.is_ok());
    assert_eq!(*perspective.get(&id).unwrap().name, "Trimmed");
}
