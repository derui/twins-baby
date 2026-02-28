use bevy::input::{ButtonInput, keyboard::Key};
use immutable::Im;
use pretty_assertions::assert_eq;
use rstest::rstest;
use smol_str::SmolStr;
use ui_event::{
    ButtonState, NotifiedKey,
    intent::{Intents, KeyboardIntent},
};

use super::map_dom_key_to_bevy;

fn notified_key(s: &str) -> NotifiedKey {
    NotifiedKey(SmolStr::new(s))
}

// --- map_dom_key_to_bevy: named key mappings ---

#[rstest]
#[case("Enter", Key::Enter)]
#[case("Escape", Key::Escape)]
#[case("Backspace", Key::Backspace)]
#[case(" ", Key::Space)]
#[case("Tab", Key::Tab)]
#[case("Delete", Key::Delete)]
#[case("Insert", Key::Insert)]
#[case("Home", Key::Home)]
#[case("End", Key::End)]
#[case("PageUp", Key::PageUp)]
#[case("PageDown", Key::PageDown)]
#[case("ArrowUp", Key::ArrowUp)]
#[case("ArrowDown", Key::ArrowDown)]
#[case("ArrowLeft", Key::ArrowLeft)]
#[case("ArrowRight", Key::ArrowRight)]
#[case("Shift", Key::Shift)]
#[case("Control", Key::Control)]
#[case("Alt", Key::Alt)]
#[case("Meta", Key::Meta)]
#[case("CapsLock", Key::CapsLock)]
#[case("NumLock", Key::NumLock)]
#[case("ScrollLock", Key::ScrollLock)]
#[case("ContextMenu", Key::ContextMenu)]
#[case("PrintScreen", Key::PrintScreen)]
#[case("Pause", Key::Pause)]
#[case("F1", Key::F1)]
#[case("F2", Key::F2)]
#[case("F3", Key::F3)]
#[case("F4", Key::F4)]
#[case("F5", Key::F5)]
#[case("F6", Key::F6)]
#[case("F7", Key::F7)]
#[case("F8", Key::F8)]
#[case("F9", Key::F9)]
#[case("F10", Key::F10)]
#[case("F11", Key::F11)]
#[case("F12", Key::F12)]
fn test_map_named_key(#[case] dom_key: &str, #[case] expected: Key) {
    // Arrange
    let key = notified_key(dom_key);

    // Act
    let result = map_dom_key_to_bevy(&key);

    // Assert
    assert_eq!(result, expected);
}

// --- map_dom_key_to_bevy: character fallback ---

#[rstest]
#[case("a")]
#[case("z")]
#[case("A")]
#[case("0")]
#[case("9")]
#[case("!")]
#[case("@")]
#[case("unknown_key")]
fn test_map_character_fallback(#[case] dom_key: &str) {
    // Arrange
    let key = notified_key(dom_key);

    // Act
    let result = map_dom_key_to_bevy(&key);

    // Assert
    assert_eq!(result, Key::Character(SmolStr::new(dom_key)));
}

// --- keyboard_input_system: behavior via Bevy App ---

fn make_notification(key: &str, state: ButtonState) -> Intents {
    KeyboardIntent {
        key: Im::new(NotifiedKey(SmolStr::new(key))),
        state: Im::new(state),
    }
    .into()
}

#[test]
fn test_keyboard_input_system_press_registers_key() {
    use super::keyboard_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<Key>>();
    app.add_message::<Intents>();
    app.add_systems(Update, keyboard_input_system);

    app.world_mut()
        .write_message(make_notification("Enter", ButtonState::Pressed));

    // Act
    app.update();

    // Assert
    let key_input = app.world().resource::<ButtonInput<Key>>();
    assert!(key_input.pressed(Key::Enter));
}

#[test]
fn test_keyboard_input_system_release_registers_key() {
    use super::keyboard_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<Key>>();
    app.add_message::<Intents>();
    app.add_systems(Update, keyboard_input_system);

    // First update: press
    app.world_mut()
        .write_message(make_notification("Enter", ButtonState::Pressed));
    app.update();

    // Second update: release
    app.world_mut()
        .write_message(make_notification("Enter", ButtonState::Released));

    // Act
    app.update();

    // Assert - key is no longer pressed after release
    let key_input = app.world().resource::<ButtonInput<Key>>();
    assert!(!key_input.pressed(Key::Enter));
}

#[test]
fn test_keyboard_input_system_clears_just_pressed_each_frame() {
    use super::keyboard_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<Key>>();
    app.add_message::<Intents>();
    app.add_systems(Update, keyboard_input_system);

    app.world_mut()
        .write_message(make_notification("ArrowLeft", ButtonState::Pressed));
    app.update();

    // Act - second update with no messages
    app.update();

    // Assert - just_pressed is cleared each frame (clear() only clears just_pressed/just_released),
    // but pressed remains true until a Release event is received
    let key_input = app.world().resource::<ButtonInput<Key>>();
    assert!(
        !key_input.just_pressed(Key::ArrowLeft),
        "just_pressed should be cleared after the frame"
    );
    assert!(
        key_input.pressed(Key::ArrowLeft),
        "pressed persists until an explicit release event"
    );
}

#[test]
fn test_keyboard_input_system_just_pressed_after_a_frame() {
    use super::keyboard_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<Key>>();
    app.add_message::<Intents>();
    app.add_systems(Update, keyboard_input_system);

    app.world_mut()
        .write_message(make_notification("ArrowLeft", ButtonState::Pressed));
    app.update();

    // Assert
    let key_input = app.world().resource::<ButtonInput<Key>>();
    assert!(
        key_input.just_pressed(Key::ArrowLeft),
        "just_pressed should be cleared after the frame"
    );
    assert!(
        key_input.pressed(Key::ArrowLeft),
        "pressed persists until an explicit release event"
    );
}
