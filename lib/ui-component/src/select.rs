use leptos::prelude::*;
use ui_headless::select::{SelectItem, use_select, use_select_with_initial};

#[component]
pub fn SelectBox<T: SelectItem>(
    items: Vec<T>,
    /// Renders each item in the dropdown list
    item_view: impl Fn(T) -> AnyView + Clone + Send + Sync + 'static,
    /// Renders the currently selected item in the trigger button
    selected_view: impl Fn(Option<T>) -> AnyView + Clone + Send + Sync + 'static,
    #[prop(optional)] initial_selected: Option<T>,
    #[prop(optional)] on_change: Option<Callback<Option<T>>>,
) -> impl IntoView {
    let hook = if let Some(v) = initial_selected {
        use_select_with_initial(&items, v)
    } else {
        use_select(&items)
    };

    let open = hook.open;
    let close = hook.close;
    let select = hook.select;
    let attrs = hook.attrs;
    let (items, _) = signal(items);
    let item_view = StoredValue::new(item_view);

    let toggle = move |_| {
        let current_attrs = attrs.get();
        if *current_attrs.opened {
            close.run(());
        } else {
            open.run(());
        }
    };

    let on_focusout = move |_: leptos::web_sys::FocusEvent| {
        close.run(());
    };

    // Fire on_change when selection changes
    if let Some(cb) = on_change {
        Effect::new(move |_| {
            let selected = attrs.get().selected.clone();
            cb.run((*selected).clone());
        });
    }

    let is_open = move || *attrs.get().opened;

    view! {
        <div on:focusout=on_focusout tabindex="-1" class="relative inline-block outline-none">
            <button
                on:click=toggle
                class="flex items-center justify-between gap-2 w-full px-3 py-2 rounded-md border border-white/10 bg-black/50 shadow-md backdrop-blur-md hover:bg-black/70 transition-colors"
            >
                {move || {
                    let current = attrs.get().selected.clone();
                    selected_view((*current).clone())
                }}
            </button>
            <Show when=is_open>
                <div class="absolute left-0 top-full mt-1 min-w-full bg-black/80 border border-white/10 rounded-md shadow-xl backdrop-blur-md overflow-hidden z-50">
                    <For
                        each=move || items.get()
                        key=|item| item.to_string()
                        children=move |item| {
                            let item_for_select = item.clone();
                            let rendered = item_view.with_value(|f| f(item));
                            view! {
                                <div
                                    on:mousedown=move |_| {
                                        select.run(item_for_select.clone());
                                        close.run(());
                                    }
                                    class="cursor-pointer hover:bg-white/10 transition-colors"
                                >
                                    {rendered}
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use super::SelectBox;

    fn test_item_view(item: String) -> AnyView {
        view! { <span>{item}</span> }.into_any()
    }

    fn test_selected_view(sel: Option<String>) -> AnyView {
        view! { <span>{sel.unwrap_or("Select...".to_string())}</span> }.into_any()
    }

    #[tokio::test]
    async fn test_select_box_default() {
        with_leptos_owner(async {
            // Arrange
            let items = vec![
                "Apple".to_string(),
                "Banana".to_string(),
                "Cherry".to_string(),
            ];

            // Act
            let view = view! { <SelectBox items=items item_view=test_item_view selected_view=test_selected_view /> };

            // Assert
            assert_view_snapshot!("select_box_default", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_select_box_with_initial_selected() {
        with_leptos_owner(async {
            // Arrange
            let items = vec![
                "Apple".to_string(),
                "Banana".to_string(),
                "Cherry".to_string(),
            ];

            // Act
            let view = view! {
                <SelectBox
                    items=items
                    item_view=test_item_view
                    selected_view=test_selected_view
                    initial_selected="Banana".to_string()
                />
            };

            // Assert
            assert_view_snapshot!("select_box_with_initial_selected", view);
        })
        .await;
    }
}
