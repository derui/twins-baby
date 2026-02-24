mod types;
use cad_base::id::{BodyId, PlaneId, SketchId};
pub use types::*;

use immutable::Im;

use bevy::ecs::message::Message;

/// An event to change the active sketch tool
///
/// All event must be read only forcibly.
#[derive(Message, Debug, Clone)]
pub struct SketchToolChangeNotification {
    pub tool: Im<SketchTool>,
}

/// Command series for creating sketch. This command must be sequential to send system.
///
/// This initiates mode to create sketch, with attracting in bevy's pickup mechanism.
/// If need to cancel, send `CancelCreateSketchNotification`
#[derive(Message, Debug, Clone)]
pub enum CreateSketchCommand {
    Initiate(Im<String>),
    PickUpPlane(Im<PlaneId>),
    Confirm,
}

/// Cancellation of creating sketch. This will ignore if the system already created, or
/// already canceled
#[derive(Message, Debug, Clone)]
pub struct CancelCreateSketchNotification {}

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

/// An event to notice canvas resize
#[derive(Message, Debug, Clone)]
pub struct CanvasResizeNotification {
    pub width: Im<u32>,
    pub height: Im<u32>,
}

/// An notification to notice mouse movement.
///
/// This is only for bevy, nad client x/y is client position of the canvas
#[derive(Message, Debug, Clone)]
pub struct MouseMovementNotification {
    pub delta_x: Im<i32>,
    pub delta_y: Im<i32>,
    /// Last point of moved in canvas
    pub client_x: Im<u32>,
    pub client_y: Im<u32>,
}

/// DOM's mousedown/up event representation
#[derive(Message, Debug, Clone)]
pub struct MouseButtonNotification {
    /// Last point of moved in canvas
    pub client_x: Im<u32>,
    pub client_y: Im<u32>,

    /// pressed button on event.
    pub button: Im<MouseButton>,
    pub state: Im<ButtonState>,
}

/// DOM's wheel event representation
///
/// Currently, browser's wheel event gives value and some types we can not control,
/// but we can control per-delta value, which is only -1 / +1 / 0 only. So this event is designed to be simple.
#[derive(Message, Debug, Clone)]
pub struct MouseWheelNotification {
    /// delta of x. it is only -1 / +1 / 0 only.
    pub delta_x: Im<f32>,
    /// delta of y. it is only -1 / +1 / 0 only.
    pub delta_y: Im<f32>,
}

/// DOM's keyboard event representation
#[derive(Message, Debug, Clone)]
pub struct KeyboardNotification {
    pub key: Im<NotifiedKey>,
    pub state: Im<ButtonState>,
}
