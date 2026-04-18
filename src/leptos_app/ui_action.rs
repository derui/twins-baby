use leptos::prelude::Set;
use ui_event::{
    CommandId, PerspectiveKind,
    command::{Commands, CreateBodyCommand},
};

use crate::leptos_app::use_action::{ActionContext, UiAction};

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedAction {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiAction for PerspectiveChangedAction {
    fn apply(&self, _id: CommandId, context: &ActionContext) -> Option<Commands> {
        context.ui_store.perspective.set(self.next);

        None
    }
}

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct BodyCreatedAction {
    /// The perspective changed
    pub name: String,
}

impl UiAction for BodyCreatedAction {
    fn apply(&self, id: CommandId, _context: &ActionContext) -> Option<Commands> {
        Some(Commands::CreateBody(CreateBodyCommand {
            id: id.into(),
            name: format!("Body{}", id).into(),
        }))
    }
}
