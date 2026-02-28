use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use leptos::web_sys::{KeyboardEvent, MouseEvent, WheelEvent};
use leptos::{prelude::*, wasm_bindgen::prelude::*};
use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::intent::{
    Intents, KeyboardIntent, MouseButtonIntent, MouseMovementIntent, MouseWheelIntent,
};
use ui_event::{ButtonState, MouseButton, NotifiedKey};

/// Accumulated mouse movement within a single animation frame.
#[derive(Debug, Clone, Default, Copy)]
struct AccumulatedMove {
    delta_x: i32,
    delta_y: i32,
    /// Last client position within the canvas
    client_x: u32,
    client_y: u32,
}

/// Normalizes a wheel delta value to -1.0, 0.0, or +1.0.
fn normalize_delta(delta: f64) -> f32 {
    if delta > 0.0 {
        1.0
    } else if delta < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn convert_button(button: i16) -> Option<MouseButton> {
    match button {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Center),
        2 => Some(MouseButton::Right),
        _ => None,
    }
}

/// Hook return value containing event handler closures for the Bevy canvas.
pub struct UseCanvasMouseHandler {
    pub on_mouse_move: Callback<MouseEvent>,
    pub on_mouse_down: Callback<MouseEvent>,
    pub on_mouse_up: Callback<MouseEvent>,
    pub on_wheel: Callback<WheelEvent>,
    pub on_key_down: Callback<KeyboardEvent>,
    pub on_key_up: Callback<KeyboardEvent>,
}

// Helper function to register an animation frame callback.
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    leptos::web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Convert a DOM keyboard event into a [KeyboardNotification].
fn keyboard_event_to_notification(event: &KeyboardEvent) -> KeyboardIntent {
    let state = match event.type_().as_str() {
        "keydown" => ButtonState::Pressed,
        _ => ButtonState::Released,
    };

    KeyboardIntent {
        key: NotifiedKey(event.key().into()).into(),
        state: state.into(),
    }
}

/// Hook that wires up mouse event handling for the Bevy canvas.
///
/// - `mousemove` events are accumulated per animation frame and sent as a single
///   [`MouseMovementNotification`] once per frame.
/// - `mousedown` and `mouseup` events are converted and sent immediately.
pub fn use_canvas_mouse_handler(
    notification_sender: LeptosMessageSender<Intents>,
) -> UseCanvasMouseHandler {
    let accumulated = Arc::new(Mutex::new(None::<AccumulatedMove>));

    let accumulated_move = accumulated.clone();
    let on_mouse_move = Callback::new(move |ev: MouseEvent| {
        let Ok(mut slot) = accumulated_move.lock() else {
            return;
        };

        match *slot {
            None => {
                *slot = Some(AccumulatedMove {
                    delta_x: ev.movement_x(),
                    delta_y: ev.movement_y(),
                    client_x: ev.offset_x().max(0) as u32,
                    client_y: ev.offset_y().max(0) as u32,
                });
            }
            Some(acc) => {
                *slot = Some(AccumulatedMove {
                    delta_x: acc.delta_x + ev.movement_x(),
                    delta_y: acc.delta_y + ev.movement_y(),
                    client_x: ev.offset_x().max(0) as u32,
                    client_y: ev.offset_y().max(0) as u32,
                });
            }
        };
    });

    {
        let accumulated = accumulated.clone();

        let closure = Rc::new(RefCell::new(None::<ScopedClosure<_>>));
        let closure_g = closure.clone();
        let notification_sender = notification_sender.clone();

        *(*closure_g).borrow_mut() = Some(Closure::new(move || {
            if let Ok(mut acc) = accumulated.lock() {
                if let Some(acc) = *acc {
                    let _ = notification_sender.send(
                        MouseMovementIntent {
                            delta_x: acc.delta_x.into(),
                            delta_y: acc.delta_y.into(),
                            client_x: acc.client_x.into(),
                            client_y: acc.client_y.into(),
                        }
                        .into(),
                    );
                }

                *acc = None;
            }

            request_animation_frame(closure.borrow().as_ref().unwrap())
        }));

        request_animation_frame(closure_g.borrow().as_ref().unwrap())
    }

    let sender_in_down = notification_sender.clone();
    let on_mouse_down = Callback::new(move |ev: MouseEvent| {
        if let Some(button) = convert_button(ev.button()) {
            let _ = sender_in_down.send(
                MouseButtonIntent {
                    client_x: (ev.offset_x().max(0) as u32).into(),
                    client_y: (ev.offset_y().max(0) as u32).into(),
                    button: button.into(),
                    state: ButtonState::Pressed.into(),
                }
                .into(),
            );
        }
    });

    let sender_in_up = notification_sender.clone();
    let on_mouse_up = Callback::new(move |ev: MouseEvent| {
        if let Some(button) = convert_button(ev.button()) {
            let _ = sender_in_up.send(
                MouseButtonIntent {
                    client_x: (ev.offset_x().max(0) as u32).into(),
                    client_y: (ev.offset_y().max(0) as u32).into(),
                    button: button.into(),
                    state: ButtonState::Released.into(),
                }
                .into(),
            );
        }
    });

    let wheel_sender = notification_sender.clone();
    let on_wheel = Callback::new(move |ev: WheelEvent| {
        let _ = wheel_sender.send(
            MouseWheelIntent {
                delta_x: normalize_delta(ev.delta_x()).into(),
                delta_y: normalize_delta(ev.delta_y()).into(),
            }
            .into(),
        );
    });

    let keyboard_sender_down = notification_sender.clone();
    let on_key_down = Callback::new(move |ev: KeyboardEvent| {
        let _ = keyboard_sender_down.send(keyboard_event_to_notification(&ev).into());
    });
    let keyboard_sender_up = notification_sender.clone();
    let on_key_up = Callback::new(move |ev: KeyboardEvent| {
        let _ = keyboard_sender_up.send(keyboard_event_to_notification(&ev).into());
    });

    UseCanvasMouseHandler {
        on_mouse_move,
        on_mouse_down,
        on_mouse_up,
        on_wheel,
        on_key_down,
        on_key_up,
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_delta;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(100.0, 1.0)]
    #[case(0.001, 1.0)]
    #[case(-1.0, -1.0)]
    #[case(-100.0, -1.0)]
    #[case(-0.001, -1.0)]
    #[case(0.0, 0.0)]
    fn test_normalize_delta(#[case] input: f64, #[case] expected: f32) {
        // Arrange
        // (inputs provided via rstest)

        // Act
        let result = normalize_delta(input);

        // Assert
        assert_eq!(result, expected);
    }
}
