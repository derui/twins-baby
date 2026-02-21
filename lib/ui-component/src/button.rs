use leptos::{component, ev::MouseEvent, prelude::*};
use ui_headless::button::use_button;

/// Create button. This button is `icon-based` button, the label is bottom of the button, with fits icon size.
#[component]
pub fn Button(
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] tabindex: Option<i32>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(optional, into)] icon: ViewFn,
    children: Children,
) -> impl IntoView {
    let state = use_button(disabled.unwrap_or(false));

    let disabled = move || (*state.attrs).get().disabled;

    view! {
        <button
            disabled=*disabled()
            tabindex=tabindex
            on:click=move |ev| {
                let Some(handler) = on_click else {
                    return;
                };
                handler.run(ev)
            }
            class="inline-flex flex-col items-center w-fit"
        >
            {icon.run()}
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_test::{assert_view_snapshot, with_leptos_owner};

    use super::Button;

    #[tokio::test]
    async fn test_button_default() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <Button>"label"</Button> };

            // Act & Assert
            assert_view_snapshot!("button_default", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_button_disabled() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <Button disabled=true>"label"</Button> };

            // Act & Assert
            assert_view_snapshot!("button_disabled", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_button_with_icon() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <Button icon=|| view! { <span class="icon" /> }>"label"</Button> };

            // Act & Assert
            assert_view_snapshot!("button_with_icon", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_button_with_icon_and_label() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <Button icon=|| view! { <span class="icon" /> }>"Save"</Button> };

            // Act & Assert
            assert_view_snapshot!("button_with_icon_and_label", view);
        })
        .await;
    }

    #[tokio::test]
    async fn test_button_disabled_with_label() {
        with_leptos_owner(async {
            // Arrange
            let view = view! { <Button disabled=true>"Disabled"</Button> };

            // Act & Assert
            assert_view_snapshot!("button_disabled_with_label", view);
        })
        .await;
    }
}
