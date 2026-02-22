use immutable::Im;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AccordionAttrs {
    /// extracted or not
    pub extracted: Im<bool>,

    /// Role of the accordion
    pub role: Im<&'static str>,

    _immutable: (),
}

pub struct UseAccordion {
    pub toggle: Im<Callback<()>>,
    pub open: Im<Callback<()>>,
    pub close: Im<Callback<()>>,

    /// Attributes memoized
    pub attrs: Im<Memo<AccordionAttrs>>,

    _immutable: (),
}

/// Create accordion logic
pub fn use_accordion(initial_extracted: bool) -> UseAccordion {
    let (extracted, set_extracted) = signal(initial_extracted);
    let open = Callback::new(move |_| set_extracted.set(true)).into();
    let close = Callback::new(move |_| set_extracted.set(false)).into();
    let toggle = Callback::new(move |_| set_extracted.update(|v| *v = !*v)).into();

    let attrs = Memo::new(move |_| AccordionAttrs {
        extracted: extracted.get().into(),
        role: "button".into(),
        _immutable: (),
    })
    .into();

    UseAccordion {
        toggle,
        open,
        close,
        attrs,
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
    async fn initial_extracted_false_reflects_in_attrs() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(false);

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, false);
            assert_eq!(*attrs.role, "button");
        })
        .await;
    }

    #[tokio::test]
    async fn initial_extracted_true_reflects_in_attrs() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(true);

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, true);
        })
        .await;
    }

    #[tokio::test]
    async fn open_callback_sets_extracted_true() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(false);

            // Act
            ret.open.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, true);
        })
        .await;
    }

    #[tokio::test]
    async fn close_callback_sets_extracted_false() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(true);

            // Act
            ret.close.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, false);
        })
        .await;
    }

    #[tokio::test]
    async fn toggle_callback_flips_extracted_from_false_to_true() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(false);

            // Act
            ret.toggle.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, true);
        })
        .await;
    }

    #[tokio::test]
    async fn toggle_callback_flips_extracted_from_true_to_false() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(true);

            // Act
            ret.toggle.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, false);
        })
        .await;
    }

    #[tokio::test]
    async fn open_then_close_toggles_state() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(false);

            // Act - open first
            ret.open.run(());
            Executor::tick().await;
            let opened_attrs = ret.attrs.get();

            // Act - then close
            ret.close.run(());
            Executor::tick().await;
            let closed_attrs = ret.attrs.get();

            // Assert
            assert_eq!(*opened_attrs.extracted, true);
            assert_eq!(*closed_attrs.extracted, false);
        })
        .await;
    }

    #[tokio::test]
    async fn toggle_twice_returns_to_initial_state() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_accordion(false);

            // Act
            ret.toggle.run(());
            Executor::tick().await;
            ret.toggle.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.extracted, false);
        })
        .await;
    }
}
