pub mod operation;

use anyhow::Result;
use immutable::Im;

use crate::{feature::operation::Operation, id::SketchId};

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
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name must not be empty"));
        }

        self.name = name.trim().to_string().into();
        Ok(())
    }
}
