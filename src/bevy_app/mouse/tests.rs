use bevy::input::{ButtonInput, mouse::MouseButton};
use immutable::Im;
use pretty_assertions::assert_eq;
use rstest::rstest;
use ui_event::{
    ButtonState, MouseButton as MB,
    intent::{Intents, MouseButtonIntent},
};

use super::to_bevy_mouse_button;

fn make_notification(button: MB, state: ButtonState) -> Intents {
    MouseButtonIntent {
        client_x: Im::new(0),
        client_y: Im::new(0),
        button: Im::new(button),
        state: Im::new(state),
    }
    .into()
}

// --- to_bevy_mouse_button: button mappings ---

#[rstest]
#[case(MB::Left, MouseButton::Left)]
#[case(MB::Right, MouseButton::Right)]
#[case(MB::Center, MouseButton::Middle)]
fn test_to_bevy_mouse_button(#[case] mb: MB, #[case] expected: MouseButton) {
    // Arrange / Act
    let result = to_bevy_mouse_button(&mb);

    // Assert
    assert_eq!(result, expected);
}

// --- mouse_input_system: behavior via Bevy App ---

#[test]
fn test_mouse_input_system_press_registers_button() {
    use super::mouse_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<MouseButton>>();
    app.add_message::<Intents>();
    app.add_systems(Update, mouse_input_system);

    app.world_mut()
        .write_message(make_notification(MB::Left, ButtonState::Pressed));

    // Act
    app.update();

    // Assert
    let mouse_input = app.world().resource::<ButtonInput<MouseButton>>();
    assert!(mouse_input.pressed(MouseButton::Left));
}

#[test]
fn test_mouse_input_system_release_registers_button() {
    use super::mouse_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<MouseButton>>();
    app.add_message::<Intents>();
    app.add_systems(Update, mouse_input_system);

    // First update: press
    app.world_mut()
        .write_message(make_notification(MB::Left, ButtonState::Pressed));
    app.update();

    // Second update: release
    app.world_mut()
        .write_message(make_notification(MB::Left, ButtonState::Released));

    // Act
    app.update();

    // Assert
    let mouse_input = app.world().resource::<ButtonInput<MouseButton>>();
    assert!(!mouse_input.pressed(MouseButton::Left));
}

#[test]
fn test_mouse_input_system_clears_just_pressed_each_frame() {
    use super::mouse_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<MouseButton>>();
    app.add_message::<Intents>();
    app.add_systems(Update, mouse_input_system);

    app.world_mut()
        .write_message(make_notification(MB::Right, ButtonState::Pressed));
    app.update();

    // Act - second update with no messages
    app.update();

    // Assert
    let mouse_input = app.world().resource::<ButtonInput<MouseButton>>();
    assert!(
        !mouse_input.just_pressed(MouseButton::Right),
        "just_pressed should be cleared after the frame"
    );
    assert!(
        mouse_input.pressed(MouseButton::Right),
        "pressed persists until an explicit release event"
    );
}

#[test]
fn test_mouse_input_system_just_pressed_after_a_frame() {
    use super::mouse_input_system;
    use bevy::app::{App, Update};

    // Arrange
    let mut app = App::new();
    app.init_resource::<ButtonInput<MouseButton>>();
    app.add_message::<Intents>();
    app.add_systems(Update, mouse_input_system);

    app.world_mut()
        .write_message(make_notification(MB::Center, ButtonState::Pressed));

    // Act
    app.update();

    // Assert
    let mouse_input = app.world().resource::<ButtonInput<MouseButton>>();
    assert!(
        mouse_input.just_pressed(MouseButton::Middle),
        "just_pressed should be true immediately after the press frame"
    );
    assert!(
        mouse_input.pressed(MouseButton::Middle),
        "pressed should be true after press"
    );
}
