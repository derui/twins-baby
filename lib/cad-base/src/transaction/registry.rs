use std::{any::TypeId, collections::HashMap};

use crate::transaction::{PerspectiveHistory, Snapshot};

/// Simple registry implementation for perspective
pub(crate) struct PerspectiveRegistry {
    /// All histories for perspective, each `TypeId` must be type of the state of history.
    perspectives: HashMap<TypeId, Box<dyn PerspectiveHistory>>,

    /// All transaction logs for undo
    transaction_log: Vec<Vec<TypeId>>,

    /// All redo logs for affected perspectives
    redo_log: Vec<Vec<TypeId>>,
}

impl PerspectiveRegistry {
    /// Make a new registry
    pub fn new() -> Self {
        PerspectiveRegistry {
            perspectives: HashMap::new(),
            transaction_log: Vec::new(),
            redo_log: Vec::new(),
        }
    }

    /// Register a new register with type `P` .
    ///
    /// # Summary
    /// This function will register a history for the type `P` . This will replace the history if already exists.
    /// Old transaction and redo logs must be trancated. because it can not be restored
    pub fn register<P: Snapshot>(
        &mut self,
        history: Box<dyn PerspectiveHistory>,
    ) -> Option<Box<dyn PerspectiveHistory>> {
        let type_id = TypeId::of::<P>();

        self.transaction_log.clear();
        self.redo_log.clear();
        self.perspectives.insert(type_id, history)
    }

    /// Get a current reference of a snapshot.
    ///
    /// Notice: this might occur panic when your history implementation and type are mismatch. be careful.
    pub fn fetch<S: Snapshot>(&self) -> Option<&S> {
        let Some(s) = self.perspectives.get(&TypeId::of::<S>()) else {
            return None;
        };

        s.as_any().downcast_ref::<S>()
    }

    /// Get a current mutable reference of a snapshot.
    ///
    /// Notice: this might occur panic when your history implementation and type are mismatch. be careful.
    pub fn modify<S: Snapshot>(&mut self) -> Option<&mut S> {
        let Some(s) = self.perspectives.get_mut(&TypeId::of::<S>()) else {
            return None;
        };

        s.as_any_mut().downcast_mut::<S>()
    }
}
