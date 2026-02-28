pub mod command;
pub mod notification;
mod types;
use cad_base::id::{BodyId, PlaneId, SketchId};
pub use types::*;

use immutable::Im;

use bevy::ecs::message::Message;

/// Command series for creating sketch. This command must be sequential to send system.
///
/// This initiates mode to create sketch, with attracting in bevy's pickup mechanism.
/// If need to cancel, send `CancelCreateSketchCommand`
#[derive(Message, Debug, Clone)]
pub enum CreateSketchCommand {
    Initiate(Im<String>),
    PickUpPlane(Im<PlaneId>),
    Confirm,
}

/// Cancellation of creating sketch. This will ignore if the system already created, or
/// already canceled
#[derive(Message, Debug, Clone)]
pub struct CancelCreateSketchCommand {}

/// Response of [CreateSketchCommand] . this only return when the [CreateSketchCommand::Confirm] was sent.
#[derive(Message, Debug, Clone)]
pub struct CreateSketchCommandReturn {
    /// sketch id created.
    pub sketch_id: Im<SketchId>,
}

/// A command to create body
#[derive(Message, Debug, Clone)]
pub struct CreateBodyCommand {
    pub name: Im<String>,
}

/// Response of [CreateBodyCommand] .
#[derive(Message, Debug, Clone)]
pub struct CreateBodyCommandReturn {
    pub body_id: Im<BodyId>,
}
