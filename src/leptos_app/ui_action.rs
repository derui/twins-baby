use leptos::prelude::Set;
use ui_event::{
    CommandId, PerspectiveKind,
    command::{Commands, CreateBodyCommand},
};

use crate::leptos_app::ui_state::{UiAction, UiStore};

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedAction {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiAction for PerspectiveChangedAction {
    fn apply(&self, state: &UiStore, _id: CommandId) -> Option<Commands> {
        state.perspective.set(self.next);

        None
    }
}

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct BodyCreatedAction {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiAction for BodyCreatedAction {
    fn apply(&self, state: &UiStore, id: CommandId) -> Option<Commands> {
        Some(
            CreateBodyCommand {
                id: id.into(),
                name: format!("Body{}", id).into(),
            }
            .into(),
        )
    }
}
