use bevy::ecs::message::Message;
use enum_dispatch::enum_dispatch;
use ui_event_macros::ServerIntent;

use crate::ObjectType;

/// Intents for UI -> Bevy notification.
#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum ServerIntents {
    ObjectSelectionChange(ObjectSelectionChangeServerIntent),
}

/// A notification marker trait.
#[enum_dispatch(ServerIntents)]
pub trait ServerIntent {
    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: ServerIntent + 'static>(&self) -> Option<&T>;
}

/// An event to notice canvas resize
#[derive(Debug, Clone, ServerIntent)]
pub struct ObjectSelectionChangeServerIntent {
    pub selections: Vec<ObjectType>,
}
