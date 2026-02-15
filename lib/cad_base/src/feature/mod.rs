pub mod operation;
mod perspective;

use std::error::Error;

use color_eyre::eyre::Result;
use immutable::Im;
use thiserror::Error;

use crate::{
    feature::operation::Operation,
    id::{SketchId, SolidId},
    plane::Plane,
    sketch::Sketch,
    solid::{Solid, face::Face},
};
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

    /// Solid Id what created by this feature.
    pub solid: Im<Option<SolidId>>,

    _immutable: (),
}

impl Feature {
    /// Get new feature
    #[tracing::instrument(err)]
    pub fn new(name: &str, sketch: SketchId, operation: &Operation) -> Result<Self> {
        if name.trim().is_empty() {
            return Err(color_eyre::eyre::eyre!("Name must not be empty"));
        }

        Ok(Feature {
            name: name.trim().to_string().into(),
            sketch: sketch.into(),
            operation: operation.clone().into(),
            status: FeatureStatus::Stale.into(),
            solid: (None as Option<SolidId>).into(),
            _immutable: (),
        })
    }

    /// Update name with [name]
    #[tracing::instrument(err)]
    fn set_name(&mut self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(color_eyre::eyre::eyre!("Name must not be empty"));
        }

        self.name = name.trim().to_string().into();
        Ok(())
    }

    /// Set new operation.
    pub fn set_operation(&mut self, operation: &Operation) {
        self.operation = operation.clone().into()
    }

    /// Update status
    fn set_status(&mut self, status: &FeatureStatus) {
        self.status = status.clone().into()
    }
}

/// Attached Target of the sketch.
#[derive(Debug, Clone)]
pub enum AttachedTarget<'a> {
    Plane(&'a Plane),
    Face(&'a Face),
}

///
#[derive(Debug, Clone)]
pub struct FeatureContext<'a> {
    /// Sketches based on feature operation. For example, pad operation must only have 1 sketch for.
    pub sketches: Im<Vec<&'a Sketch>>,
    /// Targets of sketches. This must be same size of sketches and keep index
    pub target: Im<Vec<AttachedTarget<'a>>>,
}

#[derive(Debug, Error)]
pub enum EvaluateError {
    /// Sketch does not have
    #[error("No sketches in the context")]
    InsufficientSketch,

    #[error("Given sketch can not make closed surface | {0}")]
    HaveSomeInvalidSketches(#[from] Box<dyn Error>),
}

pub trait Evaluate {
    /// Evaluate a feature with context, and make solid from feature.
    ///
    /// This is a main objective of feature, each implementation is not depended on
    /// types in base.
    fn evaluate<'a>(
        feature: &Feature,
        context: &FeatureContext<'a>,
    ) -> Result<Vec<Solid>, EvaluateError>;
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
