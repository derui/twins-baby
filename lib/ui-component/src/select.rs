use leptos::{ev::FocusEvent, portal::Portal, prelude::*};
use ui_headless::select::{SelectItem, UseSelect, use_select, use_select_with_initial};

#[component]
pub fn SelectBox<N, T: SelectItem>(
    items: Vec<T>,
    /// Renders each item in the dropdown list
    item_view: impl Fn(T) -> N + Clone + Send + Sync + 'static,
    /// Renders the currently selected item in the trigger button
    selected_view: impl Fn(Option<T>) -> N + Clone + Send + Sync + 'static,
    #[prop(optional)] initial_selected: Option<T>,
    #[prop(optional)] on_change: Option<Callback<Option<T>>>,
    #[prop(optional)] button_class: Option<&'static str>,
) -> impl IntoView
where
    N: IntoView + 'static,
{
    let UseSelect {
        open,
        close,
        select,
        attrs,
        ..
    } = if let Some(v) = initial_selected {
        use_select_with_initial(&items, v)
    } else {
        use_select(&items)
    };

    let (items, _) = signal(items);
    let select_cb = Callback::new(move |index: usize| {
        let items = items.get();

        if let Some(v) = items.get(index) {
            select.run(v.clone());
        }
    });

    let on_focusout = move |_: FocusEvent| {
        close.run(());
    };

    // Fire on_change when selection changes
    if let Some(cb) = on_change {
        Effect::new(move |_| {
            let selected = attrs.get().selected.clone();
            cb.run((*selected).clone());
        });
    }

    let trigger_ref = NodeRef::<leptos::html::Button>::new();
    let (rect, set_rect) = signal((0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64)); // top, left, width, height

    let toggle = move |_| {
        if *(attrs.get()).opened {
            close.run(());
        } else {
            if let Some(el) = trigger_ref.get() {
                use leptos::wasm_bindgen::JsCast;
                use leptos::web_sys::*;
                let reference_element: &HtmlElement = el.as_ref();
                let ref_rect = reference_element
                    .unchecked_ref::<Element>()
                    .get_bounding_client_rect();
                set_rect.set((
                    ref_rect.top() + ref_rect.height(),
                    ref_rect.left(),
                    ref_rect.width(),
                    0.0,
                ));
            }
            open.run(());
        }
    };

    let is_open = move || *(attrs.get()).opened;
    let selected = move || selected_view((*(attrs.get()).selected).clone());
    let item_view = Callback::new(move |index: usize| {
        let items = items.get();
        let v = items.get(index).expect("Should be get in valid index");

        item_view(v.clone())
    });

    view! {
        <div on:focusout=on_focusout tabindex="-1" class="relative inline-block outline-none">
            <button
                node_ref=trigger_ref
                on:click=toggle
                class=move || {
                    format!(
                        "flex items-center justify-between w-full {}",
                        button_class.unwrap_or(""),
                    )
                }
            >
                {selected}
            </button>
            <Show when=is_open>
                <Portal>
                    <div
                        style=move || {
                            let (top, left, width, height) = rect.get();
                            format!(
                                "position:fixed;top:{}px;left:{}px;min-width:{}px;z-index:9999;",
                                top + height,
                                left,
                                width,
                            )
                        }
                        class="rounded-md shadow-xl overflow-hidden"
                    >
                        <For
                            each=move || items.get().into_iter().enumerate()
                            key=|item| item.0
                            children=move |(index, _)| {
                                let item = item_view.run(index);
                                view! {
                                    <div
                                        on:mousedown=move |_| {
                                            select_cb.run(index);
                                            close.run(());
                                        }
                                        class="flex flex-row cursor-pointer"
                                    >
                                        {item}
                                    </div>
                                }
                            }
                        />
                    </div>
                </Portal>
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
