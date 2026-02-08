pub mod operation;
mod perspective;

use anyhow::Result;
use immutable::Im;

use crate::{feature::operation::Operation, id::SketchId};
pub use perspective::*;

/// Status of feature evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureStatus {
    Valid,
    Stale,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub name: Im<String>,

    /// sketch id to apply operation.
    pub sketch: Im<SketchId>,

    /// Feature operation
    pub operation: Im<Operation>,

    /// status of feature
    pub status: Im<FeatureStatus>,

    _immutable: (),
}

impl Feature {
    /// Get new feature
    pub fn new(name: &str, sketch: SketchId, operation: &Operation) -> Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name must not be empty"));
        }

        Ok(Feature {
            name: name.trim().to_string().into(),
            sketch: sketch.into(),
            operation: operation.clone().into(),
            status: FeatureStatus::Stale.into(),
            _immutable: (),
        })
    }

    /// Update name with [name]
    fn set_name(&mut self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name must not be empty"));
        }

        self.name = name.trim().to_string().into();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::operation::{Operation, Pad};
    use crate::id::SketchId;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use solver::equation::Equation;

    fn make_operation() -> Operation {
        let eq: Equation = 10.0.into();
        Pad::new(&eq).into()
    }

    fn make_sketch_id() -> SketchId {
        SketchId::from(1)
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[case("\t")]
    fn test_new_returns_error_for_blank_name(#[case] name: &str) {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();

        // act
        let result = Feature::new(name, sketch, &op);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_new_creates_feature_with_stale_status() {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();

        // act
        let feature = Feature::new("Pad1", sketch, &op).unwrap();

        // assert
        assert_eq!(*feature.status, FeatureStatus::Stale);
    }

    #[test]
    fn test_new_trims_name() {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();

        // act
        let feature = Feature::new("  Pad1  ", sketch, &op).unwrap();

        // assert
        assert_eq!(*feature.name, "Pad1");
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[case("\t")]
    fn test_set_name_returns_error_for_blank_name(#[case] name: &str) {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();
        let mut feature = Feature::new("Pad1", sketch, &op).unwrap();

        // act
        let result = feature.set_name(name);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_set_name_updates_name() {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();
        let mut feature = Feature::new("Pad1", sketch, &op).unwrap();

        // act
        feature.set_name("NewName").unwrap();

        // assert
        assert_eq!(*feature.name, "NewName");
    }

    #[test]
    fn test_set_name_trims_name() {
        // arrange
        let sketch = make_sketch_id();
        let op = make_operation();
        let mut feature = Feature::new("Pad1", sketch, &op).unwrap();

        // act
        feature.set_name("  Trimmed  ").unwrap();

        // assert
        assert_eq!(*feature.name, "Trimmed");
    }
}
