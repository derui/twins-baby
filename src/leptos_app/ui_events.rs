use leptos::prelude::Set;

use crate::{
    events::PerspectiveKind,
    leptos_app::ui_state::{UiReducer, UiStore},
};

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedEvent {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiReducer for PerspectiveChangedEvent {
    fn apply(&self, state: &UiStore) {
        state.perspective.set(self.next);
    }
}
