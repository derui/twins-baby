use bevy::ecs::message::Message;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Notification;

use crate::{ButtonState, MouseButton, NotifiedKey};

/// A notification marker trait.
#[enum_dispatch(Notifications)]
pub trait Notification {
    /// Get the ref when the type is for the specified <T>
    fn select_ref<T: Notification + 'static>(&self) -> Option<&T>;
}

#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Notifications {
    CanvasResize(CanvasResizeNotification),
    MouseMovement(MouseMovementNotification),
    MouseButton(MouseButtonNotification),
    MouseWheel(MouseWheelNotification),
    Keyboard(KeyboardNotification),
}

/// An event to notice canvas resize
#[derive(Debug, Clone, Notification)]
pub struct CanvasResizeNotification {
    pub width: Im<u32>,
    pub height: Im<u32>,
}

/// An notification to notice mouse movement.
///
/// This is only for bevy, nad client x/y is client position of the canvas
#[derive(Debug, Clone, Notification)]
pub struct MouseMovementNotification {
    pub delta_x: Im<i32>,
    pub delta_y: Im<i32>,
    /// Last point of moved in canvas
    pub client_x: Im<u32>,
    pub client_y: Im<u32>,
}

/// DOM's mousedown/up event representation
#[derive(Debug, Clone, Notification)]
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
#[derive(Debug, Clone, Notification)]
pub struct MouseWheelNotification {
    /// delta of x. it is only -1 / +1 / 0 only.
    pub delta_x: Im<f32>,
    /// delta of y. it is only -1 / +1 / 0 only.
    pub delta_y: Im<f32>,
}

/// DOM's keyboard event representation
#[derive(Debug, Clone, Notification)]
pub struct KeyboardNotification {
    pub key: Im<NotifiedKey>,
    pub state: Im<ButtonState>,
}
