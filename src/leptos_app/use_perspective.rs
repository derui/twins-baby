use leptos::prelude::{Effect, Get, Set, Signal, WriteSignal, provide_context, signal, use_context};
use leptos_bevy_canvas::prelude::{LeptosChannelMessageSender, LeptosMessageSender};

use crate::events::{PerspectiveChangeEvent, PerspectiveKind};

/// This module provides a hook to manage global **perspective** of the app.
pub struct UsePerspective(Signal<PerspectiveKind>, WriteSignal<PerspectiveKind>);

impl UsePerspective {
    /// Get the current perspective value
    fn get(&self) -> PerspectiveKind {
        self.0.get()
    }

    /// Set a new perspective value
    fn set(&self, value: PerspectiveKind) {
        self.1.set(value);
    }
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
        let _ = sender.send(PerspectiveChangeEvent {next: value
        });
    });

    Effect::new(move || {
        let Some(v) = use_context() else {return};

        if v != value.get() {
            set_value.set(v);
        }
    });

    UsePerspective(value.into(), set_value)
}

#[cfg(test)]
mod tests {
    use leptos_bevy_canvas::prelude::message_l2b;
    use pretty_assertions::assert_eq;

    use crate::leptos_app::test_leptos::with_leptos_owner;

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
            assert_eq!(hook.get(), PerspectiveKind::default());
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
            assert_eq!(hook.get(), PerspectiveKind::Feature);
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
            hook.set(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.get(), PerspectiveKind::Sketch);
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
            hook.set(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.get(), PerspectiveKind::Feature);
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
            hook.set(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;
            hook.set(PerspectiveKind::Feature);
            any_spawner::Executor::tick().await;
            hook.set(PerspectiveKind::Sketch);
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(hook.get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
