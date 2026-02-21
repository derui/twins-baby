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
