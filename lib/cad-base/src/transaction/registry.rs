use std::{any::TypeId, collections::HashMap};

use crate::transaction::PerspectiveHistory;

/// Simple registry implementation for perspective
pub(crate) struct PerspectiveRegistry {
    /// All histories for perspective
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
    pub fn register<P: 'static>(
        &mut self,
        history: Box<dyn PerspectiveHistory>,
    ) -> Option<Box<dyn PerspectiveHistory>> {
        let type_id = TypeId::of::<P>();

        self.transaction_log.clear();
        self.redo_log.clear();
        self.perspectives.insert(type_id, history)
    }
}
