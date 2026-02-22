use leptos::prelude::{
    Effect, Get, Set, Signal, WriteSignal, provide_context, signal, use_context,
};
use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};

use crate::events::{PerspectiveChangeEvent, PerspectiveKind};

/// This module provides a hook to manage global **perspective** of the app.
pub struct UsePerspective {
    pub perspective: Signal<PerspectiveKind>,
    pub set_perspective: WriteSignal<PerspectiveKind>,
}

/// Get a hook of perspective. The hook can:
///
/// - get current perspective as reactive
/// - set a perspective in global, including beby
///
/// This hook requires wrapping with `Provider` with [PerspectiveKind] value.
pub fn use_perspective(sender: LeptosMessageSender<PerspectiveChangeEvent>) -> UsePerspective {
    let (value, set_value) = signal::<PerspectiveKind>(PerspectiveKind::default());

    Effect::new(move || {
        let value = value.get();
        provide_context(value);
        let _ = sender.send(PerspectiveChangeEvent { next: value });
    });

    Effect::new(move || {
        let Some(v) = use_context() else { return };

        if v != value.get() {
            set_value.set(v);
        }
    });

    UsePerspective {
        perspective: value.into(),
        set_perspective: set_value,
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::provide_context;
    use leptos_bevy_canvas::prelude::message_l2b;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn hook_initializes_with_default_perspective() {
        with_leptos_owner(async {
            // Arrange
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();

            // Act
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::default());
        })
        .await;
    }

    #[tokio::test]
    async fn hook_initializes_with_feature_perspective() {
        with_leptos_owner(async {
            // Arrange
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();

            // Act
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Feature);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_updates_value_to_sketch() {
        with_leptos_owner(async {
            // Arrange
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Act
            hook.set_perspective.set(PerspectiveKind::Sketch);
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
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Act
            hook.set_perspective.set(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Feature);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_syncs_from_pre_existing_context() {
        with_leptos_owner(async {
            // Arrange
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();
            provide_context(PerspectiveKind::Sketch);

            // Act
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }

    #[tokio::test]
    async fn hook_multiple_updates_reflect_latest_value() {
        with_leptos_owner(async {
            // Arrange
            let (sender, _receiver) = message_l2b::<PerspectiveChangeEvent>();
            let hook = use_perspective(sender);
            any_spawner::Executor::tick().await;

            // Act - multiple updates
            hook.set_perspective.set(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;
            hook.set_perspective.set(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;
            hook.set_perspective.set(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
