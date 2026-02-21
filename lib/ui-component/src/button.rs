use leptos::{component, ev::MouseEvent, prelude::*};
use ui_headless::button::use_button;

use crate::icon::IconType;

/// Create tool button. This button is icon-based with an aria-label for accessibility.
#[component]
pub fn ToolButton(
    icon: IconType,
    #[prop(into)] label: String,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] tabindex: Option<i32>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let state = use_button(disabled.unwrap_or(false));

    let disabled = move || (*state.attrs).get().disabled;
    let icon_url = icon.to_url();
    let icon_class = icon.size_class();

    view! {
        <button
            disabled=*disabled()
            tabindex=tabindex
            aria-label=label
            on:click=move |ev| {
                let Some(handler) = on_click else {
                    return;
                };
                handler.run(ev)
            }
            class="inline-flex flex-col items-center w-fit rounded-lg border-none shadow focus:outline-none focus:shadow-md focus:shadow-blue-400/50"
        >
            <span class=icon_class>
                <img src=icon_url class="w-full h-full" />
            </span>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use crate::icon::{IconSize, IconType};

    use super::ToolButton;

    #[tokio::test]
    async fn test_tool_button_default() {
        with_leptos_owner(async {
            // Arrange
            let view =
                view! { <ToolButton icon=IconType::Cube(IconSize::Medium) label="Cube" /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_default", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_disabled() {
        with_leptos_owner(async {
            // Arrange
            let view = view! {
                <ToolButton icon=IconType::Cube(IconSize::Medium) label="Cube" disabled=true />
            };

            // Act & Assert
            assert_view_snapshot!("tool_button_disabled", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_small_icon() {
        with_leptos_owner(async {
            // Arrange
            let view =
                view! { <ToolButton icon=IconType::Cube(IconSize::Small) label="Cube" /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_small_icon", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_large_icon() {
        with_leptos_owner(async {
            // Arrange
            let view =
                view! { <ToolButton icon=IconType::Cube(IconSize::Large) label="Cube" /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_large_icon", view);
        })
        .await;
    }
}
