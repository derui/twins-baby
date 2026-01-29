#[cfg(test)]
mod tests;

mod registry;

use std::{
    any::{Any, TypeId},
    mem::replace,
};

use crate::transaction::registry::PerspectiveRegistry;

/// Marker trait for snapshot
pub trait Snapshot: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Snapshot for T {}

/// A simple snapshot history
#[derive(Debug, Clone)]
pub struct SnapshotHistory<S: Snapshot> {
    /// Current one
    current: S,
    /// Stack of **previous** history
    undo_stack: Vec<S>,

    /// Stark of **forward** history. This history reset when any command
    /// affect history.
    redo_stack: Vec<S>,

    /// maximum size of history. undo + redo stacks.
    max_history: usize,
}

impl<S: Snapshot> SnapshotHistory<S> {
    /// Create a new history with initial state
    pub fn new(initial: S, max_history: usize) -> Self {
        assert!(max_history > 0, "History size must be greater than 0");

        SnapshotHistory {
            current: initial,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
        }
    }

    /// Read the current snapshot. Snapshot in history can not edit.
    pub fn state(&self) -> &S {
        &self.current
    }

    /// Read the current snapshot. Snapshot in history can not edit.
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.current
    }

    /// Push the cloned history to stack. This truncates redo stack.
    pub fn save_snapshot(&mut self) {
        // remove overflowed histories
        if self.undo_stack.len() >= self.max_history
            && let Some((_, rest)) = self
                .undo_stack
                .split_at_checked(self.undo_stack.len() - self.max_history + 1)
        {
            self.undo_stack = rest.to_vec();
        }

        self.undo_stack.push(self.current.clone());
        self.redo_stack.clear();
    }

    /// Undo from the history. Return if the history done undo
    pub fn undo(&mut self) -> bool {
        let Some(undo) = self.undo_stack.pop() else {
            return false;
        };

        let current = replace(&mut self.current, undo);
        self.redo_stack.push(current);
        true
    }

    /// Redo the history. Return if the history done redo
    pub fn redo(&mut self) -> bool {
        let Some(redo) = self.redo_stack.pop() else {
            return false;
        };

        let current = replace(&mut self.current, redo);
        self.undo_stack.push(current);

        true
    }
}

/// Basic transactoin history of perspective's state
pub trait PerspectiveHistory: Any + Send + Sync {
    /// Save a current snap shot to undo
    fn save_snapshot(&mut self);

    /// Undo and restore previous snapshot of the history.
    ///
    /// Return `true` when restored.
    fn undo(&mut self) -> bool;

    /// Redo and restore undoned snapshot of the history.
    ///
    /// Return `true` when restored.
    fn redo(&mut self) -> bool;

    /// Get referece as Any
    fn as_any(&self) -> &dyn Any;

    /// Get mutable referece as Any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Wrapping state of snapshot. This struct should contain implementation of `PerspectiveHistory`
pub struct TypedPerspective<S: Snapshot> {
    history: SnapshotHistory<S>,
}

// A default wrapper implementation of TypedPerspective
impl<S: Snapshot> PerspectiveHistory for TypedPerspective<S> {
    fn save_snapshot(&mut self) {
        self.history.save_snapshot();
    }

    fn undo(&mut self) -> bool {
        self.history.undo()
    }

    fn redo(&mut self) -> bool {
        self.history.redo()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A transaction to manage some changes commit/restore.
pub struct Transaction<'a> {
    register: &'a mut PerspectiveRegistry,
    affected: Vec<TypeId>,
    committed: bool,
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
