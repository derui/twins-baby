use leptos::{component, ev::MouseEvent, prelude::*};
use ui_headless::button::{UseButtonReturn, use_button};

use crate::icon::IconType;

/// A indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indicator {
    On,
    Off,
    Disabled,
}

#[component]
fn Indicator(#[prop(into)] indicator: Signal<Indicator>) -> impl IntoView {
    view! {
        <span
            class="flex rounded-full w-full h-1 shadow-2xl absolute bottom-0"
            class=(
                ["bg-green-500", "shadow-green-500/50"],
                move || indicator.get() == Indicator::On,
            )
            class=(["bg-red-500", "shadow-red-500/50"], move || indicator.get() == Indicator::Off)
            class=(
                ["bg-gray-500", "shadow-gray-500/50"],
                move || indicator.get() == Indicator::Disabled,
            )
        ></span>
    }
}

/// Create tool button. This button is icon-based with an aria-label for accessibility.
///
/// # Props
/// - `icon`: The icon to display on the button.
/// - `label`: The aria-label for the button, used for accessibility.
/// - `indicator`: An optional indicator to show the state of the button (e.g., on/off/disabled).
/// - `tabindex`: An optional tabindex for keyboard navigation.
/// - `on_click`: An optional callback that is triggered when the button is clicked.
#[component]
pub fn ToolButton(
    icon: IconType,
    #[prop(into)] label: String,
    #[prop(optional)] indicator: Option<Indicator>,
    #[prop(optional)] tabindex: Option<i32>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let UseButtonReturn { attrs, .. } =
        use_button(indicator.map(|v| v == Indicator::Disabled).unwrap_or(false));

    // need clone to avoid warning
    let a1 = attrs.clone();
    let _a2 = attrs.clone();
    let icon_url = icon.as_url();
    let icon_class = icon.as_size_class();
    let mask_style = format!(
        "mask-image: url({icon_url}); mask-size: contain; mask-repeat: no-repeat; mask-position: center;"
    );

    view! {
        <button
            disabled=move || *attrs.get().disabled
            tabindex=tabindex
            aria-label=label
            on:click=move |ev| {
                let Some(handler) = on_click else {
                    return;
                };
                handler.run(ev)
            }
            class="inline-flex flex-col items-center p-2 rounded-xl border border-white/10 bg-black/50 shadow-lg backdrop-blur-md transition-colors relative overflow-hidden"
            class=("hover:bg-black/70", move || !*a1.get().disabled)
        >
            <span class=format!("{} bg-white", icon_class) style=mask_style />
            <Indicator indicator=indicator.unwrap_or(Indicator::On) />
        </button>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use crate::{
        button::Indicator,
        icon::{IconSize, IconType},
    };

    use super::ToolButton;

    #[tokio::test]
    async fn test_tool_button_default() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <ToolButton icon=IconType::Cube(IconSize::Medium) label="Cube" /> };

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
                <ToolButton
                    icon=IconType::Cube(IconSize::Medium)
                    label="Cube"
                    indicator=Indicator::Disabled
                />
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
            let view = view! { <ToolButton icon=IconType::Cube(IconSize::Small) label="Cube" /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_small_icon", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_large_icon() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <ToolButton icon=IconType::Cube(IconSize::Large) label="Cube" /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_large_icon", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_indicator_off() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <ToolButton icon=IconType::Cube(IconSize::Medium) label="Cube" indicator=Indicator::Off /> };

            // Act & Assert
            assert_view_snapshot!("tool_button_indicator_off", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_indicator_changes_on_signal_update() {
        with_leptos_owner(async {
            // Arrange
            let (indicator, set_indicator) = signal(Indicator::On);
            let view = view! { <Indicator indicator=Signal::derive(move || indicator.get()) /> };

            // Act
            set_indicator.set(Indicator::Off);

            // Assert
            assert_view_snapshot!("indicator_signal_off", view);
        })
        .await;
    }
}
