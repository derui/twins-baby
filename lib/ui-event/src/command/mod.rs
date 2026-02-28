use bevy::ecs::message::Message;
use cad_base::id::{BodyId, PlaneId, SketchId};
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Command;

use crate::CommandId;

/// A command marker trait.
#[enum_dispatch(Commands)]
pub trait Command {
    /// Get ID of the command.
    fn id(&self) -> &CommandId;

    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Command + 'static>(&self) -> Option<&T>;
}

#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Commands {
    InitiateSketchCreation(InitiateSketchCreationCommand),
}

/// Command series for creating sketch. This command must be sequential to send system.
#[derive(Debug, Clone, Command)]
pub struct InitiateSketchCreationCommand {
    pub id: Im<CommandId>,
    pub body: Im<BodyId>,
}

/// The command to select plane on the body. This command must be after [InitiateSketchCreationCommand]
#[derive(Debug, Clone, Command)]
pub struct SelectSketchPlaneCommand {
    pub id: Im<CommandId>,
    pub plane: Im<PlaneId>,
}

/// Cancellation of creating sketch. This will ignore if the system already created, or
/// already canceled
#[derive(Debug, Clone, Command)]
pub struct CancelCreateSketchCommand {
    pub id: Im<CommandId>,
}

/// Response of [CreateSketchCommand] . this only return when the [CreateSketchCommand::Confirm] was sent.
#[derive(Debug, Clone)]
pub struct CreateSketchCommandReturn {
    /// sketch id created.
    pub sketch_id: Im<SketchId>,
}

/// A command to create body
#[derive(Debug, Clone, Command)]
pub struct CreateBodyCommand {
    pub id: Im<CommandId>,
    pub name: Im<String>,
}

/// Response of [CreateBodyCommand] .
#[derive(Debug, Clone)]
pub struct CreateBodyCommandReturn {
    pub body_id: Im<BodyId>,
}
