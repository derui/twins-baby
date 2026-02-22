use leptos::prelude::Set;

use crate::{
    events::PerspectiveKind,
    leptos_app::ui_state::{UiReducer, UiState},
};

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedEvent {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiReducer for PerspectiveChangedEvent {
    fn apply(&self, state: &UiState) {
        state.perspective.set(self.next);
    }
}
