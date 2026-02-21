use pretty_assertions::assert_eq;
use rstest::rstest;
use solver::equation::Equation;

use crate::feature::operation::{Operation, Pad};
use crate::id::{FeatureId, SketchId};

use super::FeaturePerspective;

fn make_operation() -> Operation {
    let eq: Equation = 10.0.into();
    Pad::new(&eq).into()
}

fn make_sketch_id() -> SketchId {
    SketchId::from(1)
}

#[test]
fn test_add_feature_returns_retrievable_feature() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let sketch = make_sketch_id();
    let op = make_operation();

    // Act
    let id = perspective.add_feature(&sketch, &op);

    // Assert
    assert!(perspective.get(&id).is_some());
}

#[test]
fn test_add_multiple_features_generates_unique_ids() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let sketch = make_sketch_id();
    let op = make_operation();

    // Act
    let id1 = perspective.add_feature(&sketch, &op);
    let id2 = perspective.add_feature(&sketch, &op);

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
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(&sketch, &op);

    // Act
    let removed = perspective.remove_feature(&id);

    // Assert
    assert!(removed.is_some());
}

#[test]
fn test_remove_feature_makes_feature_inaccessible() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(&sketch, &op);

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
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(&sketch, &op);

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
    let sketch = make_sketch_id();
    let op = make_operation();
    let id1 = perspective.add_feature(&sketch, &op);
    let id2 = perspective.add_feature(&sketch, &op);
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
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(&sketch, &op);

    // Act
    let result = perspective.rename_feature(&id, name);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_rename_feature_trims_whitespace() {
    // Arrange
    let mut perspective = FeaturePerspective::new();
    let sketch = make_sketch_id();
    let op = make_operation();
    let id = perspective.add_feature(&sketch, &op);

    // Act
    let result = perspective.rename_feature(&id, "  Trimmed  ");

    // Assert
    assert!(result.is_ok());
    assert_eq!(*perspective.get(&id).unwrap().name, "Trimmed");
}
