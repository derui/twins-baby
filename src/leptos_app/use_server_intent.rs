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
