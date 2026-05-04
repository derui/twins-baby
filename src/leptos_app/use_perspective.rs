use leptos::prelude::{Read, Signal, use_context};
use reactive_stores::Store;
use ui_event::PerspectiveKind;

use crate::leptos_app::{
    app_state::{AppStore, AppStoreStoreFields as _},
    ui_action::PerspectiveChangedAction,
    use_action::{UseActionReturn, use_action},
};

/// This module provides a hook to manage global **perspective** of the app.
pub struct UsePerspective<ChangeFn>
where
    ChangeFn: Fn(PerspectiveKind) + Clone,
{
    pub perspective: Signal<PerspectiveKind>,

    /// change current perspective via kind
    pub change: ChangeFn,
}

/// Get a hook of perspective. The hook can:
///
/// - get current perspective as reactive
/// - set a perspective in global, including beby
///
/// This hook requires wrapping with `Provider` with [PerspectiveKind] value.
pub fn use_perspective() -> UsePerspective<impl Fn(PerspectiveKind) + Clone + Send + Sync> {
    let context = use_context::<Store<AppStore>>().expect("Should be provided");
    let UseActionReturn { dispatch, .. } = use_action();

    let change = move |v| {
        dispatch(Box::new(PerspectiveChangedAction { next: v }));
    };

    UsePerspective {
        perspective: context.perspective().into(),
        change,
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::{Get as _, provide_context};
    use leptos_bevy_canvas::prelude::message_l2b;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use reactive_stores::Store;
    use ui_event::{Correlation, command::Commands};

    use crate::leptos_app::{app_state::AppStore, command_sender::CommandSender};

    use super::*;

    fn setup_context() -> Store<AppStore> {
        let app_store = AppStore::new();
        let (sender, _receiver) = message_l2b::<Correlation<Commands>>();
        provide_context(app_store);
        provide_context(CommandSender::new(sender));
        app_store
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
            (hook.change)(PerspectiveKind::Sketch);
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
            (hook.change)(PerspectiveKind::Feature);
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

            // Act
            (hook.change)(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;
            (hook.change)(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;
            (hook.change)(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
