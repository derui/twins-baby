use std::{any::TypeId, collections::HashMap};

use crate::transaction::{
    PerspectiveBaseline, PerspectiveHistory, Snapshot, TypedSnapshot, registry::PerspectiveRegistry,
};

/// A transaction to manage some changes commit/restore.
pub struct Transaction<'a> {
    pub(super) register: &'a mut PerspectiveRegistry,
    pub(super) affected: Vec<TypeId>,
    pub(super) committed: bool,
}

impl<'a> Transaction<'a> {
    /// Get a mutable reference to modify the perspective. If call this method after committed, this do not anything
    pub fn modify<S: Snapshot>(&mut self) -> Option<&mut S> {
        // can not change after committed
        if self.committed {
            return None;
        }

        let type_id = TypeId::of::<S>();
        if !self.affected.contains(&type_id) {
            if let Some(perspective) = self.register.perspectives.get_mut(&type_id) {
                perspective.save_snapshot();
            }
            self.affected.push(type_id);
        }

        self.register.get_mut::<S>()
    }

    /// Get a read reference. This method can call anytimes/anywhere in this transaction.
    pub fn read<S: Snapshot>(&self) -> Option<&S> {
        self.register.get::<S>()
    }

    /// Commit this transaction
    pub fn commit(&mut self) {
        if self.committed {
            // can not commit twice
            return;
        }

        self.register.transaction_log.push(self.affected.clone());
        self.register.redo_log.clear();
        self.committed = true;
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }

        // undo all affected types
        for type_id in &self.affected {
            if let Some(history) = self.register.perspectives.get_mut(type_id) {
                history.undo();
            }
        }
    }
}

/// Read-only snapshot. This have only cloned data, and can not edit any data in this.
pub struct Baseline {
    baselines: HashMap<TypeId, Box<dyn PerspectiveBaseline>>,
}

impl Baseline {
    pub fn new(registry: &HashMap<TypeId, Box<dyn PerspectiveHistory>>) -> Self {
        Self {
            baselines: registry
                .iter()
                .map(|(key, snapshot)| (*key, snapshot.snapshot_baseline()))
                .collect(),
        }
    }

    /// Get a read reference. This method can call anytimes/anywhere in this transaction.
    pub fn read<S: Snapshot>(&self) -> Option<&S> {
        let s = self.baselines.get(&TypeId::of::<S>())?;

        s.as_any()
            .downcast_ref::<TypedSnapshot<S>>()
            .map(|v| &v.state)
    }
}
