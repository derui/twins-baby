#[cfg(test)]
mod tests;

use std::{any::Any, mem::replace};

/// Marker trait for snapshot
trait Snapshot: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Snapshot for T {}

/// A simple snapshot history
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
    pub fn current(&self) -> &S {
        &self.current
    }

    /// Push the cloned history to stack. This truncates redo stack.
    pub fn push_history(&mut self, current: &S) {
        // remove overflowed histories
        if self.undo_stack.len() >= self.max_history
            && let Some((_, rest)) = self
                .undo_stack
                .split_at_checked(self.undo_stack.len() - self.max_history + 1)
        {
            self.undo_stack = rest.to_vec();
        }

        let current = replace(&mut self.current, current.clone());
        self.undo_stack.push(current);
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
pub trait PerspectiveHistory: Any + Send + Sync + 'static {
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

pub struct TypedPerspective<S: Snapshot> {
    history: SnapshotHistory<S>,
}
