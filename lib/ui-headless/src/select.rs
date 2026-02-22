use std::str::FromStr;

use immutable::Im;
use leptos::prelude::*;

/// Marker trait for SelectItem
pub trait SelectItem: ToString + FromStr + Clone + PartialEq + Send + Sync + 'static {}
impl<T: ToString + FromStr + Clone + PartialEq + Send + Sync + 'static> SelectItem for T {}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectAttrs<T: SelectItem> {
    /// current selected item.
    pub selected: Im<Option<T>>,

    /// items in list
    pub items: Im<Vec<T>>,

    /// Select list opened.
    pub opened: Im<bool>,

    _immutable: (),
}

/// Operations of select logic
#[derive(Debug, Clone)]
pub struct UseSelect<T: SelectItem> {
    /// Select an item. This can call on opened, unless no-op.
    pub select: Callback<T>,
    /// Unselect an item. This can call on opened, unless no-op.
    pub unselect: Callback<()>,

    /// Open the select.
    pub open: Callback<()>,
    /// Close the select.
    pub close: Callback<()>,

    /// Memoized attributes
    pub attrs: Memo<SelectAttrs<T>>,

    _immutable: (),
}

fn use_select_inner<T: SelectItem>(items: &[T], initial: Option<T>) -> UseSelect<T> {
    let (items, _) = signal(Vec::from(items));
    let (opened, set_opened) = signal(false);
    let (selected, set_selected) = signal(initial);

    let select = Callback::new(move |v| {
        if !opened.get() {
            return;
        }

        if !items.get().contains(&v) {
            return;
        }

        set_selected.set(Some(v));
    });

    let unselect = Callback::new(move |_| {
        if !opened.get() {
            return;
        }

        set_selected.set(None);
    });

    let open = Callback::new(move |_| {
        set_opened.set(true);
    });

    let close = Callback::new(move |_| {
        set_opened.set(false);
    });

    let attrs: Memo<SelectAttrs<T>> = Memo::new(move |_| SelectAttrs {
        selected: selected.get().into(),
        items: items.get().into(),
        opened: opened.get().into(),
        _immutable: (),
    });

    UseSelect {
        select,
        unselect,
        open,
        close,
        attrs,
        _immutable: (),
    }
}

/// Create a select logic with given items. The initial selected item is None.
pub fn use_select<T: SelectItem>(items: &[T]) -> UseSelect<T> {
    use_select_inner(items, None)
}

/// Create a select logic with given items and initial selected item.
pub fn use_select_with_initial<T: SelectItem>(items: &[T], initial: T) -> UseSelect<T> {
    use_select_inner(items, Some(initial))
}

#[cfg(test)]
mod tests {
    use any_spawner::Executor;
    use leptos::prelude::{Callable as _, Get as _};
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;

    use super::*;

    fn items() -> Vec<String> {
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Color {
        name: String,
    }

    impl Color {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl std::str::FromStr for Color {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s))
        }
    }

    fn color_items() -> Vec<Color> {
        vec![Color::new("red"), Color::new("green"), Color::new("blue")]
    }

    #[tokio::test]
    async fn user_defined_struct_select_when_opened() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&color_items());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.select.run(Color::new("green"));
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, Some(Color::new("green")));
        })
        .await;
    }

    #[tokio::test]
    async fn user_defined_struct_select_is_noop_for_unknown_item() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&color_items());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.select.run(Color::new("yellow"));
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn user_defined_struct_with_initial_and_unselect() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select_with_initial(&color_items(), Color::new("red"));
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.unselect.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn initial_selected_is_none() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn initial_opened_is_false() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.opened, false);
        })
        .await;
    }

    #[tokio::test]
    async fn initial_items_reflect_given_slice() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.items, items());
        })
        .await;
    }

    #[tokio::test]
    async fn open_sets_opened_true() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());

            // Act
            ret.open.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.opened, true);
        })
        .await;
    }

    #[tokio::test]
    async fn close_sets_opened_false() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.close.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.opened, false);
        })
        .await;
    }

    #[tokio::test]
    async fn select_sets_selected_when_opened() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.select.run("b".to_string());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, Some("b".to_string()));
        })
        .await;
    }

    #[tokio::test]
    async fn select_is_noop_when_closed() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());

            // Act
            ret.select.run("b".to_string());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn select_is_noop_for_item_not_in_list() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select(&items());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.select.run("z".to_string());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn unselect_clears_selected_when_opened() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select_with_initial(&items(), "a".to_string());
            ret.open.run(());
            Executor::tick().await;

            // Act
            ret.unselect.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, None);
        })
        .await;
    }

    #[tokio::test]
    async fn unselect_is_noop_when_closed() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select_with_initial(&items(), "a".to_string());

            // Act
            ret.unselect.run(());
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, Some("a".to_string()));
        })
        .await;
    }

    #[tokio::test]
    async fn use_select_with_initial_sets_selected() {
        with_leptos_owner(async {
            // Arrange
            let ret = use_select_with_initial(&items(), "a".to_string());

            // Act
            Executor::tick().await;
            let attrs = ret.attrs.get();

            // Assert
            assert_eq!(*attrs.selected, Some("a".to_string()));
        })
        .await;
    }
}
