use bevy::ecs::message::Message;
use cad_base::id::{BodyId, SketchId};
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Notification;

use crate::SketchCreationFailure;

/// A notification marker trait.
#[enum_dispatch(Notifications)]
pub trait Notification {
    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Notification + 'static>(&self) -> Option<&T>;
}

/// All notifications of the system. These are occurred from bevy.
#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Notifications {
    SketchCreated(SketchCreatedNotification),
    SketchCreationFailed(SketchCreationFailedNotification),
    BodyCreated(BodyCreatedNotification),
    BodyActivated(BodyActivatedNotification),
    SketchActivated(SketchActivatedNotification),
}

/// Response of [ConfimSketchCreationCommand]
#[derive(Debug, Clone, Notification)]
pub struct SketchCreatedNotification {
    /// sketch id created.
    pub sketch_id: Im<SketchId>,
    /// name of sketch created
    pub name: Im<String>,
    /// id of the body that new sketch belongs to
    pub body_id: Im<BodyId>,
}

/// Response of [ConfimSketchCreationCommand]
#[derive(Debug, Clone, Notification)]
pub struct SketchCreationFailedNotification {
    /// failure reason
    pub reason: Im<SketchCreationFailure>,
}

/// Response of [CreateBodyCommand] .
#[derive(Debug, Clone, Notification)]
pub struct BodyCreatedNotification {
    pub body_id: Im<BodyId>,
    /// name of body created
    pub name: Im<String>,
}

/// Response of [SwitchActiveBodyCommand] .
#[derive(Debug, Clone, Notification)]
pub struct BodyActivatedNotification {
    /// Activated body id
    pub body_id: Im<BodyId>,
}

/// Response of [ActivateSketchCommand] .
#[derive(Debug, Clone, Notification)]
pub struct SketchActivatedNotification {
    /// Activated sketch id
    pub sketch_id: Im<SketchId>,
}
