use immutable::Im;
use leptos::prelude::{Callback, Get as _, Memo, Set, signal};

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonAttrs {
    /// disabled/enabled the button
    pub disabled: Im<bool>,

    /// Pure role of the button
    pub role: Im<&'static str>,

    _immutable: (),
}

/// Manage Button state.
///
/// Our headless, do not manage element-related state, such as on-click callback.
/// It should be handled by the component layer.
pub struct UseButtonReturn {
    /// Disable the button
    pub disable: Im<Callback<()>>,

    /// Enable the button
    pub enable: Im<Callback<()>>,

    /// Attributes memoized
    pub attrs: Im<Memo<ButtonAttrs>>,

    _immutable: (),
}

pub fn use_button(initial_disabled: bool) -> UseButtonReturn {
    let (disabled, set_disabled) = signal(initial_disabled);
    let disable = Callback::new(move |_| set_disabled.set(true));
    let enable = Callback::new(move |_| set_disabled.set(false));

    let attrs = Memo::new(move |_| ButtonAttrs {
        disabled: disabled.get().into(),
        role: "button".into(),
        _immutable: (),
    });

    UseButtonReturn {
        disable: disable.into(),
        enable: enable.into(),
        attrs: attrs.into(),
        _immutable: (),
    }
}

#[cfg(test)]
mod tests {
    use any_spawner::Executor;
    use leptos::prelude::{Callable as _, Get as _};
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn initial_disabled_false_reflects_in_attrs() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_button(false);

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.disabled, false);
            assert_eq!(*attrs.role, "button");
        })
        .await;
    }

    #[tokio::test]
    async fn initial_disabled_true_reflects_in_attrs() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_button(true);

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.disabled, true);
        })
        .await;
    }

    #[tokio::test]
    async fn disable_callback_sets_disabled_true() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_button(false);

            // Act
            ret.disable.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.disabled, true);
        })
        .await;
    }

    #[tokio::test]
    async fn enable_callback_sets_disabled_false() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_button(true);

            // Act
            ret.enable.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.disabled, false);
        })
        .await;
    }

    #[tokio::test]
    async fn disable_then_enable_toggles_state() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_button(false);

            // Act - disable first
            ret.disable.run(());
            Executor::tick().await;
            let disabled_attrs = ret.attrs.get();

            // Act - then enable
            ret.enable.run(());
            Executor::tick().await;
            let enabled_attrs = ret.attrs.get();

            // Assert
            assert_eq!(*disabled_attrs.disabled, true);
            assert_eq!(*enabled_attrs.disabled, false);
        })
        .await;
    }
}
