use crate::{events::PerspectiveKind, leptos_app::ui_events::PerspectiveChangedEvent};
use enum_dispatch::enum_dispatch;
use leptos::prelude::*;

/// The centralized state of UI. This state is the single source of truth in UI,
/// but some states which bevy has are do not inclued this, exclude ID or metadata.
#[derive(Debug, Clone, Copy)]
pub struct UiState {
    /// Current selected perspective, this is only for UI view.
    pub perspective: WriteSignal<PerspectiveKind>,

    /// centralized UI state. see this
    pub ui: Signal<UiSignal>,

    _immutable: (),
}

/// Global single signal store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiSignal {
    /// Current selected perspective, this is only for UI view.
    pub perspective: ReadSignal<PerspectiveKind>,
}

impl UiState {
    /// New UI state.
    pub fn new() -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());

        let ui = Signal::derive(move || UiSignal { perspective });

        UiState {
            perspective: set_perspective,
            ui,
            _immutable: (),
        }
    }

    /// Dispatch the [event]
    pub fn dispatch(&self, event: UiEvents) {
        event.apply(self);
    }
}

#[enum_dispatch(UiEvents)]
pub trait UiReducer {
    /// Apply state change from the event.
    ///
    /// UiState can not mutate directly, allow only exposed write signal
    fn apply(&self, state: &UiState);
}

/// Events enum of UI.
#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum UiEvents {
    /// Occurance of changes
    PerspectiveChanged(PerspectiveChangedEvent),
}
