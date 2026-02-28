use bevy::ecs::message::Message;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Notification;

/// A command marker trait.
//#[enum_dispatch(Commands)]
pub trait Command {
    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Command + 'static>(&self) -> Option<&T>;
}

//#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Commands {}
