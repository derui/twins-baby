// A hook to handle [ServerIntents]

use leptos::prelude::*;
use leptos_bevy_canvas::prelude::LeptosMessageReceiver;
use reactive_stores::Store;
use ui_event::server::ServerIntents;

use crate::leptos_app::app_state::{AppStore, AppStoreStoreFields as _};

#[derive(Debug, Clone)]
pub struct UseServerIntentReturn;

/// Make infinite-receivelr
pub(crate) fn use_server_intent(
    receiver: LeptosMessageReceiver<ServerIntents>,
) -> UseServerIntentReturn {
    let store = use_context::<Store<AppStore>>().expect("Must initialized");

    Effect::new(move || {
        let Some(intent) = receiver.get() else {
            return;
        };

        match intent {
            ServerIntents::ObjectSelectionChange(intent) => {
                store.selections().update(|obj| {
                    obj.splice(0..obj.len(), intent.selections);
                });
            }
        }
    });

    UseServerIntentReturn
}

#[cfg(test)]
mod tests {
    use any_spawner::Executor;

    use crossbeam_channel::bounded;
    use leptos::prelude::*;
    use leptos_bevy_canvas::prelude::LeptosMessageReceiver;
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use reactive_stores::Store;
    use ui_event::{
        ObjectType,
        server::{ObjectSelectionChangeServerIntent, ServerIntents},
    };

    use crate::leptos_app::app_state::AppStore;

    use super::*;

    fn make_receiver() -> (
        LeptosMessageReceiver<ServerIntents>,
        RwSignal<Option<ServerIntents>>,
    ) {
        let (_, rx) = bounded::<ServerIntents>(50);
        let signal = RwSignal::new(None);
        let receiver = LeptosMessageReceiver::new(rx, signal);
        (receiver, signal)
    }

    fn setup_store() -> Store<AppStore> {
        let store = AppStore::new();
        provide_context(store);
        store
    }

    #[tokio::test]
    async fn selections_remain_empty_when_no_intent_received() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_store();
            let (receiver, _signal) = make_receiver();
            let _ = use_server_intent(receiver);

            // Act
            Executor::tick().await;

            // Assert
            assert_eq!(store.selections().get(), Vec::<ObjectType>::new());
        })
        .await;
    }

    #[tokio::test]
    async fn updates_selections_when_object_selection_change_received() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_store();
            let (receiver, signal) = make_receiver();
            let _ = use_server_intent(receiver);

            // Act
            signal.set(Some(ServerIntents::ObjectSelectionChange(
                ObjectSelectionChangeServerIntent {
                    selections: vec![ObjectType::Point],
                },
            )));
            Executor::tick().await;

            // Assert
            assert_eq!(store.selections().get(), vec![ObjectType::Point]);
        })
        .await;
    }

    #[tokio::test]
    async fn replaces_single_selection_with_multiple_on_subsequent_intent() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_store();
            let (receiver, signal) = make_receiver();
            let _ = use_server_intent(receiver);

            signal.set(Some(ServerIntents::ObjectSelectionChange(
                ObjectSelectionChangeServerIntent {
                    selections: vec![ObjectType::Point],
                },
            )));
            Executor::tick().await;

            // Act
            signal.set(Some(ServerIntents::ObjectSelectionChange(
                ObjectSelectionChangeServerIntent {
                    selections: vec![
                        ObjectType::Face(From::from(1)),
                        ObjectType::Point,
                        ObjectType::Point,
                    ],
                },
            )));
            Executor::tick().await;

            // Assert
            assert_eq!(
                store.selections().get(),
                vec![
                    ObjectType::Face(From::from(1)),
                    ObjectType::Point,
                    ObjectType::Point
                ]
            );
        })
        .await;
    }

    #[tokio::test]
    async fn replaces_previous_selections_on_subsequent_intent() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_store();
            let (receiver, signal) = make_receiver();
            let _ = use_server_intent(receiver);

            signal.set(Some(ServerIntents::ObjectSelectionChange(
                ObjectSelectionChangeServerIntent {
                    selections: vec![ObjectType::Point],
                },
            )));
            Executor::tick().await;

            // Act
            signal.set(Some(ServerIntents::ObjectSelectionChange(
                ObjectSelectionChangeServerIntent { selections: vec![] },
            )));
            Executor::tick().await;

            // Assert
            assert_eq!(store.selections().get(), Vec::<ObjectType>::new());
        })
        .await;
    }
}
