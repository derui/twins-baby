use leptos::prelude::{Callable as _, Callback, ReadSignal, use_context};
use ui_event::PerspectiveKind;

use crate::leptos_app::{
    ui_action::PerspectiveChangedAction,
    ui_state::UiStore,
    use_action::{UseActionReturn, use_action},
};

/// This module provides a hook to manage global **perspective** of the app.
pub struct UsePerspective {
    pub perspective: ReadSignal<PerspectiveKind>,
    pub set_perspective: Callback<PerspectiveKind>,
}

/// Get a hook of perspective. The hook can:
///
/// - get current perspective as reactive
/// - set a perspective in global, including beby
///
/// This hook requires wrapping with `Provider` with [PerspectiveKind] value.
pub fn use_perspective() -> UsePerspective {
    let context = use_context::<UiStore>().expect("Should be provided");
    let UseActionReturn { dispatch, .. } = use_action();

    let set_perspective = Callback::new(move |v| {
        dispatch.run(Box::new(PerspectiveChangedAction { next: v }));
    });

    UsePerspective {
        perspective: context.ui.perspective,
        set_perspective,
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::{Callable as _, Get as _, provide_context};
    use leptos_bevy_canvas::prelude::message_l2b;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use ui_event::command::Commands;

    use crate::leptos_app::{
        app_state::AppStore, command_sender::CommandSender, ui_state::UiStore,
    };

    use super::*;

    fn setup_context() -> UiStore {
        let app_store = AppStore::new();
        let state = UiStore::new(&app_store);
        let (sender, _receiver) = message_l2b::<Commands>();
        provide_context(app_store);
        provide_context(state.clone());
        provide_context(CommandSender::new(sender));
        state
    }

    #[tokio::test]
    async fn hook_initializes_with_default_perspective() {
        with_leptos_owner(async {
            // Arrange
            setup_context();

            // Act
            let hook = use_perspective();
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::default());
        })
        .await;
    }

    #[tokio::test]
    async fn hook_updates_value_to_sketch() {
        with_leptos_owner(async {
            // Arrange
            setup_context();
            let hook = use_perspective();
            any_spawner::Executor::tick().await;

            // Act
            hook.set_perspective.run(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_updates_value_to_feature() {
        with_leptos_owner(async {
            // Arrange
            setup_context();
            let hook = use_perspective();
            any_spawner::Executor::tick().await;

            // Act
            hook.set_perspective.run(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Feature);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_multiple_updates_reflect_latest_value() {
        with_leptos_owner(async {
            // Arrange
            setup_context();
            let hook = use_perspective();
            any_spawner::Executor::tick().await;

            // Act - multiple updates
            hook.set_perspective.run(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;
            hook.set_perspective.run(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;
            hook.set_perspective.run(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
