#![allow(clippy::manual_non_exhaustive)]
pub mod body;
pub mod feature;
pub mod id;
pub mod plane;
pub mod point;
pub mod refs;
pub mod sketch;
pub mod solid;
pub mod tag;
pub mod transaction;
pub mod vector3;

use crate::{
    body::{Body, BodyPerspective, BodyReader, PlaneRef},
    feature::FeaturePerspective,
    id::BodyId,
    refs::{PlaneScope, Resolve},
    sketch::SketchPerspective,
    transaction::{Baseline, Transaction, registry::PerspectiveRegistry},
};

/// Whole engine state of CAD
pub struct CadEngine {
    /// central registry. can not mutable out of this crate.
    registry: PerspectiveRegistry,
}

impl CadEngine {
    /// Create a new CAD engine.
    pub fn new() -> Self {
        let mut registry = PerspectiveRegistry::new();
        registry.register(BodyPerspective::new());
        registry.register(SketchPerspective::new());
        registry.register(FeaturePerspective::new());
        Self { registry }
    }

    /// A simple transaction undo
    pub fn undo(&mut self) -> bool {
        self.registry.undo()
    }

    /// Get baseline snapshot of transaction.
    pub fn baseline(&self) -> Baseline {
        self.registry.baseline()
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

impl Default for CadEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyReader for Baseline {
    fn read_body(&self, id: BodyId) -> Option<&Body> {
        self.read::<BodyPerspective>()?.get(&id)
    }
}

impl<'a> Resolve<'a, PlaneRef, PlaneScope<'a>> for Baseline {
    type Output = PlaneScope<'a>;
    fn resolve(&'a self, ref_: PlaneRef) -> Option<Self::Output> {
        self.read::<BodyPerspective>()?
            .get(&*ref_.body_id)
            .map(|b| PlaneScope::new(b, ref_))
    }
}
