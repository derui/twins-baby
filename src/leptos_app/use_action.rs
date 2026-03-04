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
