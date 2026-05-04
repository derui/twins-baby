use bevy::ecs::message::Message;
use bevy::prelude::Event;
use cad_base::id::BodyId;
use immutable::Im;
use ui_event_macros::Command;

/// Commands are UI -> Bevy command request
#[derive(Message, Debug, Clone)]
pub enum Commands {
    CreateSketchOnSelected(CreateSketchOnSelectedCommand),
    CreateBody(CreateBodyCommand),
    SwitchActiveBody(SwitchActiveBodyCommand),
}

/// A command to create a sketch to the selected object in CAD.
///
/// This command do not handle selection that is handled in CAD engine.
#[derive(Event, Debug, Clone, Command)]
pub struct CreateSketchOnSelectedCommand {}

/// A command to create body
#[derive(Event, Debug, Clone, Command)]
pub struct CreateBodyCommand {}

/// A command to switch active body
#[derive(Event, Debug, Clone, Command)]
pub struct SwitchActiveBodyCommand {
    /// Id of body to switch
    pub body_id: Im<BodyId>,
}
