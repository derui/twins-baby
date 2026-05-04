// A hook to handle [ServerIntents]

use leptos::prelude::*;
use leptos_bevy_canvas::prelude::LeptosMessageReceiver;
use reactive_stores::Store;
use ui_event::{
    Correlation,
    notification::{Notification, Notifications},
    server::ServerIntents,
};

use crate::leptos_app::app_state::{
    AppStore, AppStoreStoreFields as _, BodyState, FeatureTree, SketchState,
};

#[derive(Debug, Clone)]
pub struct UseNotificationReturn;

/// Make handler set of [Notifications]
pub(crate) fn use_notificarions(
    receiver: LeptosMessageReceiver<Correlation<Notifications>>,
) -> UseNotificationReturn {
    let store = use_context::<Store<AppStore>>().expect("Must initialized");

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
        }
    });

    UseNotificationReturn
}
