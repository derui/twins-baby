use bevy::ecs::message::Message;
use bevy::prelude::Event;
use cad_base::id::BodyId;
use immutable::Im;
use ui_event_macros::Command;

use crate::{CommandId, ObjectType};

/// Commands are UI -> Bevy command request
#[derive(Message, Debug, Clone)]
pub enum Commands {
    CreateSketchOnSelected(CreateSketchOnSelectedCommand),
    CreateBody(CreateBodyCommand),
    SwitchActiveBody(SwitchActiveBodyCommand),
}

/// A command to create a sketch to the selected object.
#[derive(Event, Debug, Clone, Command)]
pub struct CreateSketchOnSelectedCommand {
    pub id: Im<CommandId>,
    pub selected: Im<ObjectType>,
}

/// A command to create body
#[derive(Event, Debug, Clone, Command)]
pub struct CreateBodyCommand {
    /// Id of command
    pub id: Im<CommandId>,

    /// Name of the body
    pub name: Im<String>,
}

/// A command to switch active body
#[derive(Event, Debug, Clone, Command)]
pub struct SwitchActiveBodyCommand {
    /// Id of command
    pub id: Im<CommandId>,

    /// Id of body to switch
    pub body_id: Im<BodyId>,
}
