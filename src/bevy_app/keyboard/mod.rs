use bevy::{
    ecs::{change_detection::DetectChangesMut as _, message::MessageReader, system::ResMut},
    input::{ButtonInput, keyboard::Key},
};
use ui_event::{
    ButtonState, NotifiedKey,
    intent::{Intent, Intents, KeyboardIntent},
};

#[cfg(test)]
mod tests;

/// Map to bevy's key
fn map_dom_key_to_bevy(key: &NotifiedKey) -> Key {
    match key.0.as_str() {
        "Enter" => Key::Enter,
        "Escape" => Key::Escape,
        "Backspace" => Key::Backspace,
        " " => Key::Space,
        "Tab" => Key::Tab,
        "Delete" => Key::Delete,
        "Insert" => Key::Insert,
        "Home" => Key::Home,
        "End" => Key::End,
        "PageUp" => Key::PageUp,
        "PageDown" => Key::PageDown,
        "ArrowUp" => Key::ArrowUp,
        "ArrowDown" => Key::ArrowDown,
        "ArrowLeft" => Key::ArrowLeft,
        "ArrowRight" => Key::ArrowRight,
        "Shift" => Key::Shift,
        "Control" => Key::Control,
        "Alt" => Key::Alt,
        "Meta" => Key::Meta,
        "CapsLock" => Key::CapsLock,
        "NumLock" => Key::NumLock,
        "ScrollLock" => Key::ScrollLock,
        "ContextMenu" => Key::ContextMenu,
        "PrintScreen" => Key::PrintScreen,
        "Pause" => Key::Pause,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        _ => Key::Character(key.0.clone()),
    }
}

/// leptos-connected version of keyboard input system
pub fn keyboard_input_system(
    mut key_input: ResMut<ButtonInput<Key>>,
    mut keyboard_input_reader: MessageReader<Intents>,
) {
    // Avoid clearing if not empty to ensure change detection is not triggered.
    key_input.bypass_change_detection().clear();

    for event in keyboard_input_reader.read() {
        let Some(KeyboardIntent { key, state, .. }) = event.select_ref::<KeyboardIntent>() else {
            continue;
        };

        let key = map_dom_key_to_bevy(key);

        match **state {
            ButtonState::Pressed => {
                key_input.press(key);
            }
            ButtonState::Released => {
                key_input.release(key);
            }
        }
    }
}
