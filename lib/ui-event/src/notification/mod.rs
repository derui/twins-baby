use bevy::ecs::message::Message;
use cad_base::id::{BodyId, SketchId};
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Notification;

use crate::CommandId;

/// A notification marker trait.
#[enum_dispatch(Notifications)]
pub trait Notification {
    /// Get ID of the command.
    fn correlation_id(&self) -> &CommandId;

    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Notification + 'static>(&self) -> Option<&T>;
}

/// All notifications of the system. These are occurred from bevy.
#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Notifications {
    SketchCreated(SketchCreatedNotification),
    BodyCreated(BodyCreatedNotification),
}

/// Response of [ConfimSketchCreationCommand]
#[derive(Debug, Clone, Notification)]
pub struct SketchCreatedNotification {
    /// Original Id from the command
    pub correlation_id: Im<CommandId>,
    /// sketch id created.
    pub sketch_id: Im<SketchId>,
    /// name of sketch created
    pub name: Im<String>,
}

/// Response of [CreateBodyCommand] .
#[derive(Debug, Clone, Notification)]
pub struct BodyCreatedNotification {
    /// Original Id from the command
    pub correlation_id: Im<CommandId>,
    pub body_id: Im<BodyId>,
    /// name of body created
    pub name: Im<String>,
}
