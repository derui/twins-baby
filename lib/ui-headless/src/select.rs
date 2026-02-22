use immutable::Im;
use leptos::prelude::*;

/// Marker trait for SelectItem
pub trait SelectItem: ToString + From<String> + Clone + PartialEq + Send + Sync + 'static {}
impl<T: ToString + From<String> + Clone + PartialEq + Send + Sync + 'static> SelectItem for T {}

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
pub fn use_select_with_initial<T: SelectItem>(items: &[T], initial: Option<T>) -> UseSelect<T> {
    use_select_inner(items, initial)
}
