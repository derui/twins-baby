use bevy::ecs::message::Message;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Intent;

/// Intents for UI -> Bevy notification.
#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Intents {
    CanvasResize(CanvasResizeIntent),
}

/// A notification marker trait.
#[enum_dispatch(Intents)]
pub trait Intent {
    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Intent + 'static>(&self) -> Option<&T>;
}

/// An event to notice canvas resize
#[derive(Debug, Clone, Intent)]
pub struct CanvasResizeIntent {
    pub width: Im<u32>,
    pub height: Im<u32>,
}
