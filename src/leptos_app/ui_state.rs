use enum_dispatch::enum_dispatch;
use leptos::prelude::*;
use ui_event::PerspectiveKind;

use crate::leptos_app::ui_action::PerspectiveChangedAction;

/// The centralized state of UI. This state is the single source of truth in UI,
/// but some states which bevy has are do not inclued this, exclude ID or metadata.
#[derive(Debug, Clone, Copy)]
pub struct UiStore {
    /// Current selected perspective, this is only for UI view.
    pub perspective: WriteSignal<PerspectiveKind>,

    /// centralized UI state. see this
    pub ui: UiState,

    _immutable: (),
}

/// Global single signal store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiState {
    /// Current selected perspective, this is only for UI view.
    pub perspective: ReadSignal<PerspectiveKind>,

    _immutable: (),
}

impl UiStore {
    /// New UI state.
    pub fn new() -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());

        UiStore {
            perspective: set_perspective,
            ui: UiState {
                perspective,
                _immutable: (),
            },
            _immutable: (),
        }
    }

    /// Dispatch the [event]
    pub fn dispatch(&self, event: UiActions) {
        event.apply(self);
    }
}

#[enum_dispatch(UiActions)]
pub trait UiReducer {
    /// Apply state change from the event.
    ///
    /// UiState can not mutate directly, allow only exposed write signal
    fn apply(&self, state: &UiStore);
}

/// Events enum of UI.
#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum UiActions {
    /// Occurance of changes
    PerspectiveChanged(PerspectiveChangedAction),
}
