use immutable::Im;
use leptos::prelude::Callback;

/// A combination of button on clicked.
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Main,
    Auxiliary,
    Secondary
}

/// Button behavior. This struct makes contract of button behavior, with accecibility works.
pub struct UseButton {
    /// A callback to click. This must be wired to `on_click`
    pub on_click: Im<Callback<MouseButton>>,

    /// tabindex of the button
    pub tabindex: Im<i32>,

    /// Role of the button. This must be aria-role
    pub role: Im<&'static str>,

    _immutable: (),
}

impl UseButton {
    /// Make a new behavior of button.
    pub fn new<T: 'static + Fn(MouseButton) + Sync + Send>(on_click: T, tabindex: i32) -> UseButton {
        let on_click = Callback::new(on_click);

        UseButton {
            on_click: on_click.into(),
            tabindex: tabindex.into(),
            role: "button".into(),
            _immutable: (),
        }
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
        let button = UseButton::new(|_| {}, tabindex);

        // Assert
        assert_eq!(*button.tabindex, tabindex);
    }

    #[test]
    fn test_new_sets_role_as_button() {
        // Arrange & Act
        let button = UseButton::new(|_| {}, 0);

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
        let button = UseButton::new(move |b| *received_clone.lock().unwrap() = Some(b), 0);

        // Act
        button.on_click.run(variant);

        // Assert
        assert!(matches!(*received.lock().unwrap(), Some(_)));
    }
}
