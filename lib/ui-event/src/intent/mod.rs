use bevy::ecs::message::Message;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use ui_event_macros::Intent;

use crate::{ButtonState, MouseButton, NotifiedKey};

/// Intents for UI -> Bevy notification.
#[enum_dispatch]
#[derive(Message, Debug, Clone)]
pub enum Intents {
    CanvasResize(CanvasResizeIntent),
    MouseMovement(MouseMovementIntent),
    MouseButton(MouseButtonIntent),
    MouseWheel(MouseWheelIntent),
    Keyboard(KeyboardIntent),
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

/// An notification to notice mouse movement.
///
/// This is only for bevy, nad client x/y is client position of the canvas
#[derive(Debug, Clone, Intent)]
pub struct MouseMovementIntent {
    pub delta_x: Im<i32>,
    pub delta_y: Im<i32>,
    /// Last point of moved in canvas
    pub client_x: Im<u32>,
    pub client_y: Im<u32>,
}

/// DOM's mousedown/up event representation
#[derive(Debug, Clone, Intent)]
pub struct MouseButtonIntent {
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
#[derive(Debug, Clone, Intent)]
pub struct MouseWheelIntent {
    /// delta of x. it is only -1 / +1 / 0 only.
    pub delta_x: Im<f32>,
    /// delta of y. it is only -1 / +1 / 0 only.
    pub delta_y: Im<f32>,
}

/// DOM's keyboard event representation
#[derive(Debug, Clone, Intent)]
pub struct KeyboardIntent {
    pub key: Im<NotifiedKey>,
    pub state: Im<ButtonState>,
}
