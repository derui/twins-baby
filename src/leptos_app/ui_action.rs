use leptos::prelude::Set;
use ui_event::PerspectiveKind;

use crate::leptos_app::ui_state::{UiReducer, UiStore};

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedAction {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiReducer for PerspectiveChangedAction {
    fn apply(&self, state: &UiStore) {
        state.perspective.set(self.next);
    }
}
