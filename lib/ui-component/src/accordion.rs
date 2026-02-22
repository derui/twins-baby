use leptos::prelude::*;
use ui_headless::accordion::use_accordion;

#[component]
pub fn TreeAccordion(
    #[prop(into)] trigger: ViewFn,
    children: Children,
    #[prop(optional)] initial_open: Option<bool>,
) -> impl IntoView {
    let state = use_accordion(initial_open.unwrap_or(false));
    let toggle = *state.toggle;
    let attrs = *state.attrs;

    let is_open = move || *attrs.get().extracted;

    view! {
        <div class="flex flex-col w-full">
            <button
                role=move || *attrs.get().role
                on:click=move |_| toggle.run(())
                class="flex flex-row items-center gap-2 w-full rounded-md border border-white/10 transition-colors text-left"
            >
                <img
                    src="/assets/icons/chevron-right.svg"
                    class="w-4 h-4 transition-transform duration-200 opacity-60"
                    class:rotate-90=is_open
                />
               {trigger.run()}
            </button>
            <div class=move || {
                if is_open() {
                    "flex flex-col pl-4 border-l border-white/10 mt-1"
                } else {
                    "hidden"
                }
            }>
                {children()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use super::TreeAccordion;

    #[tokio::test]
    async fn test_tree_accordion_default() {
        with_leptos_owner(async {
            // Arrange
            let view = view! {
                <TreeAccordion trigger=|| view! { "Section" }>
                    <div>"Content"</div>
                </TreeAccordion>
            };

            // Act & Assert
            assert_view_snapshot!("tree_accordion_default", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tree_accordion_initial_open() {
        with_leptos_owner(async {
            // Arrange
            let view = view! {
                <TreeAccordion trigger=|| view! { "Section" } initial_open=true>
                    <div>"Content"</div>
                </TreeAccordion>
            };

            // Act & Assert
            assert_view_snapshot!("tree_accordion_initial_open", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tree_accordion_nested() {
        with_leptos_owner(async {
            // Arrange
            let view = view! {
                <TreeAccordion trigger=|| view! { "Parent" } initial_open=true>
                    <TreeAccordion trigger=|| view! { "Child" }>
                        <div>"Nested content"</div>
                    </TreeAccordion>
                </TreeAccordion>
            };

            // Act & Assert
            assert_view_snapshot!("tree_accordion_nested", view);
        })
        .await;
    }
}
