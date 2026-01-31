use std::{any::TypeId, collections::HashMap};

use crate::transaction::{
    PerspectiveHistory, Snapshot, SnapshotHistory, Transaction, TypedPerspective,
};

/// Simple registry implementation for perspective
pub(crate) struct PerspectiveRegistry {
    /// All histories for perspective, each `TypeId` must be type of the state of history.
    pub(crate) perspectives: HashMap<TypeId, Box<dyn PerspectiveHistory>>,

    /// All transaction logs for undo
    pub(crate) transaction_log: Vec<Vec<TypeId>>,

    /// All redo logs for affected perspectives
    pub(crate) redo_log: Vec<Vec<TypeId>>,

    _private: (),
}

impl PerspectiveRegistry {
    /// Make a new registry
    pub fn new() -> Self {
        PerspectiveRegistry {
            perspectives: HashMap::new(),
            transaction_log: Vec::new(),
            redo_log: Vec::new(),
            _private: (),
        }
    }

    /// Register a new register with type `P` .
    ///
    /// # Summary
    /// This function will register a history for the type `P` . This will replace the history if already exists.
    /// Old transaction and redo logs must be trancated. because it can not be restored
    pub fn register<S: Snapshot>(&mut self, initial: S) {
        let history = TypedPerspective {
            history: SnapshotHistory::new(initial, 100),
        };
        let type_id = TypeId::of::<S>();

        self.transaction_log.clear();
        self.redo_log.clear();
        self.perspectives.insert(type_id, Box::new(history));
    }

    /// Get a current reference of a snapshot.
    ///
    /// Notice: this might occur panic when your history implementation and type are mismatch. be careful.
    pub fn get<S: Snapshot>(&self) -> Option<&S> {
        let Some(s) = self.perspectives.get(&TypeId::of::<S>()) else {
            return None;
        };

        s.as_any()
            .downcast_ref::<TypedPerspective<S>>()
            .map(|v| v.history.state())
    }

    /// Get a current mutable reference of a snapshot.
    ///
    /// Notice: this might occur panic when your history implementation and type are mismatch. be careful.
    pub fn get_mut<S: Snapshot>(&mut self) -> Option<&mut S> {
        let Some(s) = self.perspectives.get_mut(&TypeId::of::<S>()) else {
            return None;
        };

        s.as_any_mut()
            .downcast_mut::<TypedPerspective<S>>()
            .map(|v| v.history.state_mut())
    }

    /// A simple transaction undo
    pub fn undo(&mut self) -> bool {
        let Some(affected) = self.transaction_log.pop() else {
            return false;
        };

        for type_id in &affected {
            if let Some(history) = self.perspectives.get_mut(&type_id) {
                history.undo();
            }
        }
        self.redo_log.push(affected);
        true
    }

    /// A simple redo transaction
    pub fn redo(&mut self) -> bool {
        let Some(affected) = self.redo_log.pop() else {
            return false;
        };

        for type_id in &affected {
            if let Some(history) = self.perspectives.get_mut(&type_id) {
                history.redo();
            }
        }
        self.transaction_log.push(affected);
        true
    }

    /// Begin a transaction. This function can execute each one of the system
    pub fn begin(&mut self) -> Transaction<'_> {
        Transaction {
            register: self,
            affected: Vec::new(),
            committed: false,
        }
    }
}
