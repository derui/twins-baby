use std::collections::HashMap;

use cad_base::id::BodyId;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use leptos::prelude::*;
use ui_event::PerspectiveKind;

use crate::leptos_app::ui_action::PerspectiveChangedAction;

/// Immutable UI DTO for Body.
#[derive(Debug, Clone)]
pub struct BodyUI {
    pub id: Im<BodyId>,
    pub name: Im<String>,
    pub order: Im<u32>,

    _immutable: (),
}

impl BodyUI {
    /// Make new [BodyUI]
    pub fn new(id: BodyId, name: &str, order: u32) -> BodyUI {
        BodyUI {
            id: id.into(),
            name: name.to_string().into(),
            order: order.into(),
            _immutable: (),
        }
    }
}

/// The centralized state of UI. This state is the single source of truth in UI,
/// but some states which bevy has are do not inclued this, exclude ID or metadata.
#[derive(Debug, Clone, Copy)]
pub struct UiStore {
    /// Current selected perspective, this is only for UI view.
    pub perspective: WriteSignal<PerspectiveKind>,

    pub bodies: WriteSignal<HashMap<BodyId, BodyUI>>,

    /// centralized UI state. see this
    pub ui: UiState,

    _immutable: (),
}

/// Global single signal store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiState {
    /// Current selected perspective, this is only for UI view.
    pub perspective: ReadSignal<PerspectiveKind>,

    /// Bodies in the application
    pub bodies: Signal<Vec<BodyUI>>,

    _immutable: (),
}

impl UiStore {
    /// New UI state.
    pub fn new() -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());
        let (bodies, set_bodies) = signal(HashMap::<BodyId, BodyUI>::new());

        let body_list = Signal::derive(move || {
            let mut bodies = bodies.get().into_values().collect::<Vec<_>>();
            bodies.sort_by_key(|v| *v.order);

            bodies
        });

        UiStore {
            perspective: set_perspective,
            bodies: set_bodies,
            ui: UiState {
                perspective,
                bodies: body_list,
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
