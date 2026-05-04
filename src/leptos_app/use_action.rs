use leptos::prelude::{Callable, Callback, use_context};
use reactive_stores::Store;

use crate::leptos_app::{app_state::AppStore, command_sender::CommandSender, ui_action::UiAction};

pub struct UseActionReturn<DispatchFn>
where
    DispatchFn: Fn(Box<dyn UiAction>) + Clone,
{
    /// Dispatch the action
    pub dispatch: DispatchFn,

    _immutable: (),
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub store: Store<AppStore>,
}

pub fn use_action() -> UseActionReturn<impl Fn(Box<dyn UiAction>) + Clone + Send + Sync> {
    let store = use_context::<Store<AppStore>>().expect("Must set UiStore before");
    let sender = use_context::<CommandSender>().expect("Must set CommandSender before");

    let context = ActionContext { store };

    let dispatch = Callback::new(move |action: Box<dyn UiAction>| {
        if let Some(command) = action.apply(&context) {
            sender.send(command);
        }
    });

    let do_dispatch = { move |action: Box<dyn UiAction>| dispatch.run(action) };

    UseActionReturn {
        dispatch: do_dispatch,
        _immutable: (),
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::provide_context;
    use leptos_bevy_canvas::prelude::{BevyMessageReceiver, message_l2b};
    use leptos_bevy_canvas::traits::HasReceiver;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use reactive_stores::Store;
    use ui_event::{
        Correlation, PerspectiveKind,
        command::{Commands, CreateBodyCommand},
    };

    use crate::leptos_app::{
        app_state::{AppStore, AppStoreStoreFields as _},
        command_sender::CommandSender,
    };

    use super::*;

    fn setup_context() -> (Store<AppStore>, BevyMessageReceiver<Correlation<Commands>>) {
        let app_store = AppStore::new();
        let (sender, receiver) = message_l2b::<Correlation<Commands>>();
        provide_context(app_store);
        provide_context(CommandSender::new(sender));
        (app_store, receiver)
    }

    struct NoOpAction;

    impl UiAction for NoOpAction {
        fn apply(&self, _context: &ActionContext) -> Option<Commands> {
            None
        }
    }

    struct SendCommandAction;

    impl UiAction for SendCommandAction {
        fn apply(&self, _context: &ActionContext) -> Option<Commands> {
            Some(CreateBodyCommand {}.into())
        }
    }

    #[tokio::test]
    async fn dispatch_action_returning_none_sends_no_command() {
        with_leptos_owner(async {
            // Arrange
            let (_store, receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            // Act
            dispatch(Box::new(NoOpAction));
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
            let (_store, receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            // Act
            dispatch(Box::new(SendCommandAction));
            any_spawner::Executor::tick().await;

            // Assert
            let received = receiver.rx().try_recv();
            assert!(received.is_ok());
            assert!(matches!(*received.unwrap().data, Commands::CreateBody(_)));
        })
        .await;
    }

    #[tokio::test]
    async fn dispatch_applies_store_changes() {
        with_leptos_owner(async {
            use leptos::prelude::{Get as _, Set};

            // Arrange
            let (app_store, _receiver) = setup_context();
            let UseActionReturn { dispatch, .. } = use_action();

            struct SetPerspectiveAction;
            impl UiAction for SetPerspectiveAction {
                fn apply(&self, context: &ActionContext) -> Option<Commands> {
                    context.store.perspective().set(PerspectiveKind::Sketch);
                    None
                }
            }

            // Act
            dispatch(Box::new(SetPerspectiveAction));
            any_spawner::Executor::tick().await;

            // Assert
            assert_eq!(app_store.perspective().get(), PerspectiveKind::Sketch);
        })
        .await;
    }
}
