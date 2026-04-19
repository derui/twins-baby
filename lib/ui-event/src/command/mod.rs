use bevy::ecs::message::Message;
use bevy::prelude::Event;
use cad_base::id::{BodyId, PlaneId};
use immutable::Im;
use ui_event_macros::Command;

use crate::CommandId;

#[derive(Message, Debug, Clone)]
pub enum Commands {
    InitiateSketchCreation(InitiateSketchCreationCommand),
    SelectSketchPlane(SelectSketchPlaneCommand),
    CancelSketchCreation(CancelSketchCreationCommand),
    ConfirmSketchCreation(ConfirmSketchCreationCommand),
    CreateBody(CreateBodyCommand),
}

/// Command series for creating sketch. This command must be sequential to send system.
#[derive(Event, Debug, Clone, Command)]
pub struct InitiateSketchCreationCommand {
    pub id: Im<CommandId>,
    pub body: Im<BodyId>,
}

/// The command to select plane on the body. This command must be after [InitiateSketchCreationCommand]
#[derive(Event, Debug, Clone, Command)]
pub struct SelectSketchPlaneCommand {
    pub id: Im<CommandId>,
    pub plane: Im<PlaneId>,
}

/// The command to select plane on the body. This command must be after [InitiateSketchCreationCommand]
#[derive(Event, Debug, Clone, Command)]
pub struct ConfirmSketchCreationCommand {
    pub id: Im<CommandId>,
}

/// Cancellation of creating sketch. This will ignore if the system already created, or
/// already canceled
#[derive(Event, Debug, Clone, Command)]
pub struct CancelSketchCreationCommand {
    pub id: Im<CommandId>,
}

/// A command to create body
#[derive(Event, Debug, Clone, Command)]
pub struct CreateBodyCommand {
    /// Id of command
    pub id: Im<CommandId>,

    /// Name of the body
    pub name: Im<String>,
}
