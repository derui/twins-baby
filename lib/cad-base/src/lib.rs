#![allow(clippy::manual_non_exhaustive)]
pub mod body;
pub mod feature;
pub mod id;
pub mod plane;
pub mod point;
pub mod sketch;
pub mod solid;
pub mod transaction;
pub mod vector3;

use crate::transaction::{Transaction, registry::PerspectiveRegistry};

/// Whole engine state of CAD
pub struct CadEngine {
    /// central registry. can not mutable out of this crate.
    registry: PerspectiveRegistry,
}

impl CadEngine {
    /// Create a new CAD engine.
    pub fn new() -> Self {
        Self {
            registry: PerspectiveRegistry::new(),
        }
    }

    /// A simple transaction undo
    pub fn undo(&mut self) -> bool {
        self.registry.undo()
    }

    /// A simple redo transaction
    pub fn redo(&mut self) -> bool {
        self.registry.redo()
    }

    /// Begin a transaction. This function can execute each one of the system
    pub fn begin(&mut self) -> Transaction<'_> {
        self.registry.begin()
    }
}
