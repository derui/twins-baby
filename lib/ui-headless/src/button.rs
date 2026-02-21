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

impl ButtonState {
    /// Convert to the [ButtonAttrs]
    pub fn to_attrs(&self) -> ButtonAttrs {
        let disabled = *self.disabled;
        ButtonAttrs {
            tabindex: if disabled {(-1).into()} else {0.into()} ,
            disabled: disabled.into(),
            role: "role".into(),
            _immutable: ()
        }
    }
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
    pub role: Im<&'static str>,

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
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    // Helper to create a ButtonState with a specific disabled value
    fn state_with(disabled: bool) -> ButtonState {
        ButtonState {
            disabled: disabled.into(),
            _immutable: (),
        }
    }

    #[rstest]
    #[case(false, ButtonAction::Disable, true)]
    #[case(true, ButtonAction::Disable, true)]
    #[case(false, ButtonAction::Enable, false)]
    #[case(true, ButtonAction::Enable, false)]
    #[case(false, ButtonAction::Toggle, true)]
    #[case(true, ButtonAction::Toggle, false)]
    fn test_reduce_button(
        #[case] initial_disabled: bool,
        #[case] action: ButtonAction,
        #[case] expected_disabled: bool,
    ) {
        // Arrange
        let state = state_with(initial_disabled);

        // Act
        let next = reduce_button(&state, action);

        // Assert
        assert_eq!(*next.disabled, expected_disabled);
    }

    #[test]
    fn test_button_state_default_is_enabled() {
        // Arrange / Act
        let state = ButtonState::default();

        // Assert
        assert_eq!(*state.disabled, false);
    }

    #[rstest]
    #[case(false, false, 0)]
    #[case(true, true, -1)]
    fn test_to_attrs_disabled_and_tabindex(
        #[case] initial_disabled: bool,
        #[case] expected_disabled: bool,
        #[case] expected_tabindex: i32,
    ) {
        // Arrange
        let state = state_with(initial_disabled);

        // Act
        let attrs = state.to_attrs();

        // Assert
        assert_eq!(*attrs.disabled, expected_disabled);
        assert_eq!(*attrs.tabindex, expected_tabindex);
    }

    #[rstest]
    #[case(false)]
    #[case(true)]
    fn test_to_attrs_role_is_always_role(#[case] initial_disabled: bool) {
        // Arrange
        let state = state_with(initial_disabled);

        // Act
        let attrs = state.to_attrs();

        // Assert
        assert_eq!(*attrs.role, "role");
    }
}
