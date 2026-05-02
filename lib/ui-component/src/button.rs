use leptos::{component, ev::MouseEvent, prelude::*};
use ui_headless::button::{UseButtonReturn, use_button};

use crate::icon::IconType;

/// A indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorState {
    On,
    Off,
    Disabled,
}

#[component]
fn Indicator(#[prop(into)] indicator: Signal<IndicatorState>) -> impl IntoView {
    view! {
        <span
            class="flex rounded-full w-full h-1 shadow-2xl absolute bottom-0"
            class=(
                ["bg-green-500", "shadow-green-500/50"],
                move || indicator.get() == IndicatorState::On,
            )
            class=(
                ["bg-red-500", "shadow-red-500/50"],
                move || indicator.get() == IndicatorState::Off,
            )
            class=(
                ["bg-gray-500", "shadow-gray-500/50"],
                move || indicator.get() == IndicatorState::Disabled,
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
    #[prop(optional, into)] indicator: MaybeProp<IndicatorState>,
    #[prop(optional)] tabindex: Option<i32>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let UseButtonReturn {
        disable,
        enable,
        attrs,
        ..
    } = use_button(
        indicator
            .get_untracked()
            .map(|v| v == IndicatorState::Disabled)
            .unwrap_or(false),
    );

    Effect::new(move |_| {
        if indicator
            .get()
            .map(|v| v == IndicatorState::Disabled)
            .unwrap_or(false)
        {
            (*disable).run(());
        } else {
            (*enable).run(());
        }
    });

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
            <Indicator indicator=Signal::derive(move || {
                indicator.get().unwrap_or(IndicatorState::On)
            }) />
        </button>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use crate::icon::{IconSize, IconType};

    use super::{Indicator, IndicatorState, ToolButton};

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
                    indicator=IndicatorState::Disabled
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
            let view = view! {
                <ToolButton
                    icon=IconType::Cube(IconSize::Medium)
                    label="Cube"
                    indicator=IndicatorState::Off
                />
            };

            // Act & Assert
            assert_view_snapshot!("tool_button_indicator_off", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_indicator_changes_on_signal_update() {
        with_leptos_owner(async {
            // Arrange
            let (indicator, set_indicator) = signal(IndicatorState::On);
            let view_on = view! { <Indicator indicator=Signal::derive(move || indicator.get()) /> };
            assert_view_snapshot!("indicator_signal_on", view_on);

            // Act
            set_indicator.set(IndicatorState::Off);

            // Assert
            let view_off =
                view! { <Indicator indicator=Signal::derive(move || indicator.get()) /> };
            assert_view_snapshot!("indicator_signal_off", view_off);
        })
        .await;
    }

    #[tokio::test]
    async fn test_tool_button_indicator_changes_through_prop() {
        with_leptos_owner(async {
            // Arrange
            let indicator = RwSignal::new(IndicatorState::On);
            let view_on = view! {
                <ToolButton
                    icon=IconType::Cube(IconSize::Medium)
                    label="Cube"
                    indicator=Signal::derive(move || indicator.get())
                />
            };
            assert_view_snapshot!("tool_button_indicator_prop_on", view_on);

            // Act
            indicator.set(IndicatorState::Off);

            // Assert
            let view_off = view! {
                <ToolButton
                    icon=IconType::Cube(IconSize::Medium)
                    label="Cube"
                    indicator=Signal::derive(move || indicator.get())
                />
            };
            assert_view_snapshot!("tool_button_indicator_prop_off", view_off);
        })
        .await;
    }
}
