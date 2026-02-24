#[cfg(test)]
mod tests;

use bevy::{
    ecs::{change_detection::DetectChangesMut as _, message::MessageReader, system::ResMut},
    input::{ButtonInput, mouse::MouseButton},
};
use ui_event::{ButtonState, MouseButton as MB, MouseButtonNotification};

/// leptos-connected version of mouse input system
pub fn mouse_input_system(
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
    mut mouse_input_reader: MessageReader<MouseButtonNotification>,
) {
    // Avoid clearing if not empty to ensure change detection is not triggered.
    mouse_input.bypass_change_detection().clear();

    for event in mouse_input_reader.read() {
        let MouseButtonNotification { button, state, .. } = event;

        let button = to_bevy_mouse_button(button);

        match **state {
            ButtonState::Pressed => {
                mouse_input.press(button);
            }
            ButtonState::Released => {
                mouse_input.release(button);
            }
        }
    }
}

/// Convert leptos-connected mouse button to bevy's MouseButton enum.
fn to_bevy_mouse_button(button: &MB) -> MouseButton {
    match button {
        MB::Left => MouseButton::Left,
        MB::Right => MouseButton::Right,
        MB::Center => MouseButton::Middle,
    }
}
