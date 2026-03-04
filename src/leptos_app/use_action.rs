use leptos::prelude::{Callable, Callback, use_context};
use ui_event::{CommandId, command::Commands};

use crate::leptos_app::{app_state::AppStore, command_sender::CommandSender, ui_state::UiStore};

pub struct UseActionReturn {
    /// Dispatch the action
    pub dispatch: Callback<Box<dyn UiAction>>,

    _immutable: (),
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub ui_store: UiStore,
    pub app_store: AppStore,
}

pub fn use_action() -> UseActionReturn {
    let ui_store = use_context::<UiStore>().expect("Must set UiStore before");
    let app_store = use_context::<AppStore>().expect("Must set AppStore before");
    let sender = use_context::<CommandSender>().expect("Must set CommandSender before");

    let context = ActionContext {
        ui_store,
        app_store,
    };

    let dispatch = Callback::new(move |action: Box<dyn UiAction>| {
        let id = app_store.gen_id.run(());

        if let Some(command) = action.apply(id, &context) {
            sender.send(command);
        }
    });

    UseActionReturn {
        dispatch,
        _immutable: (),
    }
}

pub trait UiAction {
    /// Apply state change from the event.
    ///
    /// UiState can not mutate directly, allow only exposed write signal
    fn apply(&self, id: CommandId, context: &ActionContext) -> Option<Commands>;
}

#[cfg(test)]
mod tests {
    use leptos::prelude::{Callable as _, provide_context};
    use leptos_bevy_canvas::prelude::{BevyMessageReceiver, message_l2b};
    use leptos_bevy_canvas::traits::HasReceiver;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use ui_event::{
        PerspectiveKind,
        command::{Commands, CreateBodyCommand},
    };

    use crate::leptos_app::{
        app_state::AppStore, command_sender::CommandSender, ui_state::UiStore,
    };

    use super::*;

    fn setup_context() -> (UiStore, BevyMessageReceiver<Commands>) {
        let app_store = AppStore::new();
        let ui_store = UiStore::new(&app_store);
        let (sender, receiver) = message_l2b::<Commands>();
        provide_context(app_store);
        provide_context(ui_store.clone());
        provide_context(CommandSender::new(sender));
        (ui_store, receiver)
    }

    struct NoOpAction;

    impl UiAction for NoOpAction {
        fn apply(&self, _id: CommandId, _context: &ActionContext) -> Option<Commands> {
            None
        }
    }

    struct SendCommandAction;

    impl UiAction for SendCommandAction {
        fn apply(&self, id: CommandId, _context: &ActionContext) -> Option<Commands> {
            Some(
                CreateBodyCommand {
                    id: id.into(),
                    name: "test".to_string().into(),
                }
                .into(),
            )
        }
    }

    #[tokio::test]
    async fn dispatch_action_returning_none_sends_no_command() {
        with_leptos_owner(async {
            // Arrange
            let (_ui_store, receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            // Act
            dispatch.run(Box::new(NoOpAction));
            any_spawner::Executor::tick().await;

            // Assert
            assert!(receiver.rx().try_recv().is_err());
        })
        .await;
    }

    #[tokio::test]
    async fn dispatch_action_returning_some_sends_command() {
        with_leptos_owner(async {
            // Arrange
            let (_ui_store, receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            // Act
            dispatch.run(Box::new(SendCommandAction));
            any_spawner::Executor::tick().await;

            // Assert
            let received = receiver.rx().try_recv();
            assert!(received.is_ok());
            assert!(matches!(received.unwrap(), Commands::CreateBody(_)));
        })
        .await;
    }

    #[tokio::test]
    async fn dispatch_applies_ui_state_changes() {
        with_leptos_owner(async {
            use leptos::prelude::{Get as _, Set};

            // Arrange
            let (ui_store, _receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            struct SetPerspectiveAction;
            impl UiAction for SetPerspectiveAction {
                fn apply(&self, _id: CommandId, context: &ActionContext) -> Option<Commands> {
                    context.ui_store.perspective.set(PerspectiveKind::Sketch);
                    None
                }
            }

            // Act
            dispatch.run(Box::new(SetPerspectiveAction));
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(ui_store.ui.perspective.get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
