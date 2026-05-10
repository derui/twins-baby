// A hook to handle [ServerIntents]

use leptos::prelude::*;
use leptos_bevy_canvas::prelude::LeptosMessageReceiver;
use reactive_stores::Store;
use ui_event::{Correlation, PerspectiveKind, notification::Notifications};

use crate::leptos_app::{
    app_state::{AppStore, AppStoreStoreFields as _, BodyState, FeatureTree, SketchState},
    ui_action::{PerspectiveChangedAction, SketchActivatedAction},
    use_action::{UseActionReturn, use_action},
};

#[derive(Debug, Clone)]
pub struct UseNotificationReturn;

/// Make handler set of [Notifications]
pub(crate) fn use_notificarions(
    receiver: LeptosMessageReceiver<Correlation<Notifications>>,
) -> UseNotificationReturn {
    let store = use_context::<Store<AppStore>>().expect("Must initialized");
    let UseActionReturn { dispatch, .. } = use_action();

    Effect::new(move || {
        let Some(intent) = receiver.get() else {
            return;
        };

        match &*intent {
            Notifications::BodyCreated(n) => {
                store.bodies().update(|bodies| {
                    let order = bodies.len();
                    bodies.push(BodyState::new(*n.body_id, &n.name, order));
                });
                store.feature_trees().update(|trees| {
                    trees.push(FeatureTree::new(&n.body_id));
                });
            }
            Notifications::SketchCreated(n) => {
                let state: SketchState = n.into();

                store.feature_trees().update(|trees| {
                    let Some(tree) = trees.iter_mut().find(|t| *t.body_id == *state.body_id) else {
                        return;
                    };

                    tree.add_sketch(&state)
                });

                store.sketches().update(|v| {
                    v.push(state);
                });

                // created sketch should be activated ASAP
                dispatch(
                    SketchActivatedAction {
                        sketch_id: *n.sketch_id,
                    }
                    .into(),
                );
            }
            Notifications::BodyActivated(n) => {
                store.bodies().update(|bodies| {
                    let Some(index) = bodies.iter().position(|v| *v.id == *n.body_id) else {
                        return;
                    };

                    for body in bodies.iter_mut() {
                        body.deactivate();
                    }

                    bodies[index].activate();
                });
            }
            Notifications::SketchCreationFailed(n) => {
                tracing::warn!("Got error on sketch creation: {:?}", *n.reason)
            }
            Notifications::SketchActivated(n) => {
                store.sketches().update(|sketches| {
                    let Some(index) = sketches.iter().position(|v| *v.id == *n.sketch_id) else {
                        return;
                    };

                    for sketch in sketches.iter_mut() {
                        sketch.deactivate();
                    }

                    sketches[index].activate();
                });

                store.perspective().set(PerspectiveKind::Sketch);
            }
        }
    });

    UseNotificationReturn
}

#[cfg(test)]
mod tests {
    use any_spawner::Executor;
    use cad_base::id::{BodyId, SketchId};
    use crossbeam_channel::bounded;
    use leptos::prelude::*;
    use leptos_bevy_canvas::prelude::{LeptosMessageReceiver, message_l2b};
    use leptos_test::with_leptos_owner;
    use pretty_assertions::assert_eq;
    use reactive_stores::Store;
    use ui_event::{
        CommandId, Correlation, PerspectiveKind,
        command::Commands,
        notification::{
            BodyActivatedNotification, BodyCreatedNotification, SketchActivatedNotification,
            SketchCreatedNotification, SketchCreationFailedNotification,
        },
    };

    use crate::leptos_app::{
        app_state::{AppStore, AppStoreStoreFields as _, FeatureNode},
        command_sender::CommandSender,
    };

    use super::*;

    fn make_receiver() -> (
        LeptosMessageReceiver<Correlation<Notifications>>,
        RwSignal<Option<Correlation<Notifications>>>,
    ) {
        let (_, rx) = bounded::<Correlation<Notifications>>(50);
        let signal = RwSignal::new(None);
        let receiver = LeptosMessageReceiver::new(rx, signal);
        (receiver, signal)
    }

    fn setup_context() -> Store<AppStore> {
        let store = AppStore::new();
        let (sender, _) = message_l2b::<Correlation<Commands>>();
        provide_context(store);
        provide_context(CommandSender::new(sender));
        store
    }

    fn make_correlation(notification: Notifications) -> Correlation<Notifications> {
        Correlation::new(CommandId::new(1), notification)
    }

    #[tokio::test]
    async fn store_unchanged_when_no_notification_received() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, _signal) = make_receiver();
            let _ = use_notificarions(receiver);

            // Act
            Executor::tick().await;

            // Assert
            assert_eq!(store.bodies().get(), vec![]);
            assert_eq!(store.sketches().get(), vec![]);
            assert_eq!(store.feature_trees().get(), vec![]);
        })
        .await;
    }

    #[tokio::test]
    async fn body_created_adds_body_and_feature_tree() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            // Act
            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let bodies = store.bodies().get();
            assert_eq!(bodies.len(), 1);
            assert_eq!(*bodies[0].id, BodyId::new(1));
            assert_eq!(*bodies[0].name, "Body1");
            assert_eq!(*bodies[0].order, 0);

            let trees = store.feature_trees().get();
            assert_eq!(trees.len(), 1);
            assert_eq!(*trees[0].body_id, BodyId::new(1));
        })
        .await;
    }

    #[tokio::test]
    async fn body_created_assigns_incremental_order_for_multiple_bodies() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            // Act
            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(2).into(),
                    name: "Body2".to_string().into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let bodies = store.bodies().get();
            assert_eq!(bodies.len(), 2);
            assert_eq!(*bodies[0].order, 0);
            assert_eq!(*bodies[1].order, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn body_activated_marks_only_that_body_active() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(2).into(),
                    name: "Body2".to_string().into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::BodyActivated(
                BodyActivatedNotification {
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let bodies = store.bodies().get();
            assert_eq!(*bodies[0].active, true);
            assert_eq!(*bodies[1].active, false);
        })
        .await;
    }

    #[tokio::test]
    async fn body_activated_deactivates_previously_active_body() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(2).into(),
                    name: "Body2".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::BodyActivated(
                BodyActivatedNotification {
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::BodyActivated(
                BodyActivatedNotification {
                    body_id: BodyId::new(2).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let bodies = store.bodies().get();
            assert_eq!(*bodies[0].active, false);
            assert_eq!(*bodies[1].active, true);
        })
        .await;
    }

    #[tokio::test]
    async fn body_activated_for_unknown_id_does_not_change_state() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::BodyActivated(
                BodyActivatedNotification {
                    body_id: BodyId::new(999).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let bodies = store.bodies().get();
            assert_eq!(bodies.len(), 1);
            assert_eq!(*bodies[0].active, false);
        })
        .await;
    }

    #[tokio::test]
    async fn sketch_created_adds_sketch_and_registers_in_feature_tree() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::SketchCreated(
                SketchCreatedNotification {
                    sketch_id: SketchId::new(10).into(),
                    name: "Sketch1".to_string().into(),
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let sketches = store.sketches().get();
            assert_eq!(sketches.len(), 1);
            assert_eq!(*sketches[0].id, SketchId::new(10));
            assert_eq!(*sketches[0].name, "Sketch1");
            assert_eq!(*sketches[0].body_id, BodyId::new(1));

            let trees = store.feature_trees().get();
            let nodes = trees[0].nodes();
            assert_eq!(nodes.len(), 1);
            assert!(matches!(&nodes[0], FeatureNode::Sketch(s) if *s.id == SketchId::new(10)));
        })
        .await;
    }

    #[tokio::test]
    async fn sketch_activated_marks_only_that_sketch_active_and_switches_perspective() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::SketchCreated(
                SketchCreatedNotification {
                    sketch_id: SketchId::new(10).into(),
                    name: "Sketch1".to_string().into(),
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::SketchCreated(
                SketchCreatedNotification {
                    sketch_id: SketchId::new(11).into(),
                    name: "Sketch2".to_string().into(),
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::SketchActivated(
                SketchActivatedNotification {
                    sketch_id: SketchId::new(10).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let sketches = store.sketches().get();
            assert_eq!(*sketches[0].active, true);
            assert_eq!(*sketches[1].active, false);
            assert_eq!(store.perspective().get(), PerspectiveKind::Sketch);
        })
        .await;
    }

    #[tokio::test]
    async fn sketch_activated_deactivates_previously_active_sketch() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            signal.set(Some(make_correlation(Notifications::BodyCreated(
                BodyCreatedNotification {
                    body_id: BodyId::new(1).into(),
                    name: "Body1".to_string().into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::SketchCreated(
                SketchCreatedNotification {
                    sketch_id: SketchId::new(10).into(),
                    name: "Sketch1".to_string().into(),
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::SketchCreated(
                SketchCreatedNotification {
                    sketch_id: SketchId::new(11).into(),
                    name: "Sketch2".to_string().into(),
                    body_id: BodyId::new(1).into(),
                },
            ))));
            Executor::tick().await;
            signal.set(Some(make_correlation(Notifications::SketchActivated(
                SketchActivatedNotification {
                    sketch_id: SketchId::new(10).into(),
                },
            ))));
            Executor::tick().await;

            // Act
            signal.set(Some(make_correlation(Notifications::SketchActivated(
                SketchActivatedNotification {
                    sketch_id: SketchId::new(11).into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            let sketches = store.sketches().get();
            assert_eq!(*sketches[0].active, false);
            assert_eq!(*sketches[1].active, true);
        })
        .await;
    }

    #[tokio::test]
    async fn sketch_creation_failed_does_not_change_store() {
        with_leptos_owner(async {
            // Arrange
            let store = setup_context();
            let (receiver, signal) = make_receiver();
            let _ = use_notificarions(receiver);

            // Act
            signal.set(Some(make_correlation(Notifications::SketchCreationFailed(
                SketchCreationFailedNotification {
                    reason: ui_event::SketchCreationFailure::TargetIsNotValid.into(),
                },
            ))));
            Executor::tick().await;

            // Assert
            assert_eq!(store.sketches().get(), vec![]);
            assert_eq!(store.bodies().get(), vec![]);
        })
        .await;
    }
}
