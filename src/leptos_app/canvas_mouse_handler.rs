use std::sync::{Arc, Mutex};

use leptos::web_sys::MouseEvent;
use leptos::{
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};
use ui_event::{
    MouseButton, MouseDownNotification, MouseMovementNotification, MouseUpNotification,
};

/// Accumulated mouse movement within a single animation frame.
#[derive(Debug, Clone, Default, Copy)]
struct AccumulatedMove {
    delta_x: i32,
    delta_y: i32,
    /// Last client position within the canvas
    client_x: u32,
    client_y: u32,
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
}

/// Hook that wires up mouse event handling for the Bevy canvas.
///
/// - `mousemove` events are accumulated per animation frame and sent as a single
///   [`MouseMovementNotification`] once per frame.
/// - `mousedown` and `mouseup` events are converted and sent immediately.
pub fn use_canvas_mouse_handler(
    move_sender: LeptosMessageSender<MouseMovementNotification>,
    down_sender: LeptosMessageSender<MouseDownNotification>,
    up_sender: LeptosMessageSender<MouseUpNotification>,
) -> UseCanvasMouseHandler {
    let accumulated: Arc<Mutex<Option<AccumulatedMove>>> = Arc::new(Mutex::new(None));

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

        let closure = Closure::once(move || {
            if let Ok(mut acc) = accumulated.lock() {
                if let Some(acc) = acc.as_ref() {
                    let _ = move_sender.send(MouseMovementNotification {
                        delta_x: acc.delta_x.into(),
                        delta_y: acc.delta_y.into(),
                        client_x: acc.client_x.into(),
                        client_y: acc.client_y.into(),
                    });
                }

                *acc = None;
            }
        });

        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }

    let on_mouse_down = Callback::new(move |ev: MouseEvent| {
        if let Some(button) = convert_button(ev.button()) {
            let _ = down_sender.send(MouseDownNotification {
                client_x: (ev.offset_x().max(0) as u32).into(),
                client_y: (ev.offset_y().max(0) as u32).into(),
                button: button.into(),
            });
        }
    });

    let on_mouse_up = Callback::new(move |ev: MouseEvent| {
        if let Some(button) = convert_button(ev.button()) {
            let _ = up_sender.send(MouseUpNotification {
                client_x: (ev.offset_x().max(0) as u32).into(),
                client_y: (ev.offset_y().max(0) as u32).into(),
                button: button.into(),
            });
        }
    });

    UseCanvasMouseHandler {
        on_mouse_move,
        on_mouse_down,
        on_mouse_up,
    }
}
