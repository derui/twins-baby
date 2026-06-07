use bevy::ecs::message::Message;
use bevy::prelude::Event;
use cad_base::id::{BodyId, SketchId};
use immutable::Im;
use ui_event_macros::Command;

use crate::SketchGeometryOperation;

/// Commands are UI -> Bevy command request
#[derive(Message, Debug, Clone)]
pub enum Commands {
    CreateSketchOnSelected(CreateSketchOnSelectedCommand),
    CreateBody(CreateBodyCommand),
    SwitchActiveBody(SwitchActiveBodyCommand),
    ActivateSketch(ActivateSketchCommand),
    RequestGeometryCreation(RequestGeometryCreationCommand),
    CancelCurrentGeometryCreation(CancelCurrentGeometryCreationCommand),
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

/// A command to activate the sketch.
/// This command is used to move sketch perspective in CAD view
#[derive(Event, Debug, Clone, Command)]
pub struct ActivateSketchCommand {
    /// Id of the sketch to activate
    pub sketch_id: Im<SketchId>,
}

/// A command to request geometry to create on the activated sketch
///
/// This command will not send any notification for UI
#[derive(Event, Debug, Clone, Command)]
pub struct RequestGeometryCreationCommand {
    /// a geometry creating on the activated sketch
    pub geometry: Im<SketchGeometryOperation>,
}

/// A command to cancel current geometry creation process.
///
/// This command will not send any notification for UI
#[derive(Event, Debug, Clone, Command)]
pub struct CancelCurrentGeometryCreationCommand {}
