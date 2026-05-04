use cad_base::id::BodyId;
use leptos::prelude::{Get, GetUntracked, Read, Set};
use ui_event::{
    CommandId, PerspectiveKind,
    command::{
        Commands, CreateBodyCommand, CreateSketchOnSelectedCommand, SwitchActiveBodyCommand,
    },
};

use crate::leptos_app::{
    app_state::AppStoreStoreFields as _, ui_state::BodyPerspectiveUI, use_action::ActionContext,
};

pub trait UiAction {
    /// Apply state change from the event.
    ///
    /// UiState can not mutate directly, allow only exposed write signal
    fn apply(&self, context: &ActionContext) -> Option<Commands>;
}

/// An event to notice perpective change
#[derive(Debug, Clone)]
pub struct PerspectiveChangedAction {
    /// The perspective changed
    pub next: PerspectiveKind,
}

impl UiAction for PerspectiveChangedAction {
    fn apply(&self, context: &ActionContext) -> Option<Commands> {
        context.store.perspective().set(self.next);

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
    fn apply(&self, _context: &ActionContext) -> Option<Commands> {
        Some(CreateBodyCommand {}.into())
    }
}

/// An event to request to create sketch.
#[derive(Debug, Clone)]
pub struct SketchCreatedAction;

impl UiAction for SketchCreatedAction {
    fn apply(&self, context: &ActionContext) -> Option<Commands> {
        BodyPerspectiveUI::from_store(context.store)
            .can_create_sketch
            .get_untracked()
            .then(|| CreateSketchOnSelectedCommand {}.into())
    }
}

/// An evetn to activate the body
#[derive(Debug, Clone)]
pub struct BodyActivatedAction {
    /// The body id to activate
    pub body_id: BodyId,
}

impl UiAction for BodyActivatedAction {
    fn apply(&self, context: &ActionContext) -> Option<Commands> {
        Some(Commands::SwitchActiveBody(SwitchActiveBodyCommand {
            body_id: self.body_id.into(),
        }))
    }
}
