use immutable::Im;

/// A combination of button on clicked.
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Main,
    Auxiliary,
    Secondary,
}

/// Action of the button
#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Disable,
    Enable,
    Toggle,
}

/// State of the button.
pub struct ButtonState {
    pub disabled: Im<bool>,
    _immutable: (),
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            disabled: false.into(),
            _immutable: (),
        }
    }
}

pub struct ButtonAttrs {
    /// tabindex of the button
    pub tabindex: Im<i32>,
    /// disabled/enabled the button
    pub disabled: Im<bool>,

    /// Pure role of the button
    role: Im<&'static str>,

    _immutable: (),
}

/// Reduce button states.
pub fn reduce_button(state: &ButtonState, action: ButtonAction) -> ButtonState {
    match action {
        ButtonAction::Disable => ButtonState {
            disabled: true.into(),
            ..*state
        },
        ButtonAction::Enable => ButtonState {
            disabled: false.into(),
            ..*state
        },
        ButtonAction::Toggle => ButtonState {
            disabled: (!*state.disabled).into(),
            ..*state
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Callable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::sync::{Arc, Mutex};

    #[rstest]
    #[case(3)]
    #[case(0)]
    #[case(-1)]
    fn test_new_sets_tabindex(#[case] tabindex: i32) {
        // Arrange & Act
        let button = use_button(|_| {}, tabindex);

        // Assert
        assert_eq!(*button.tabindex, tabindex);
    }

    #[test]
    fn test_new_sets_role_as_button() {
        // Arrange & Act
        let button = use_button(|_| {}, 0);

        // Assert
        assert_eq!(*button.role, "button");
    }

    #[rstest]
    #[case(MouseButton::Main)]
    #[case(MouseButton::Secondary)]
    #[case(MouseButton::Auxiliary)]
    fn test_on_click_called_with_button(#[case] variant: MouseButton) {
        // Arrange
        let received = Arc::new(Mutex::new(None::<MouseButton>));
        let received_clone = received.clone();
        let button = use_button(move |b| *received_clone.lock().unwrap() = Some(b), 0);

        // Act
        button.on_click.run(variant);

        // Assert
        assert!(matches!(*received.lock().unwrap(), Some(_)));
    }
}
