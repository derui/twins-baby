use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use cad_base::id::BodyId;
use enum_dispatch::enum_dispatch;
use immutable::Im;
use leptos::prelude::*;
use ui_event::{CommandId, PerspectiveKind, command::Commands};

use crate::leptos_app::{app_state::AppStore, ui_action::PerspectiveChangedAction};

/// Immutable UI DTO for Body.
#[derive(Debug, Clone)]
pub struct BodyUI {
    pub id: Im<BodyId>,
    pub name: Im<String>,
    pub order: Im<usize>,

    _immutable: (),
}

impl BodyUI {
    /// Make new [BodyUI]
    pub fn new(id: BodyId, name: &str, order: usize) -> BodyUI {
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
#[derive(Debug, Clone)]
pub struct UiStore {
    /// Current selected perspective, this is only for UI view.
    pub perspective: WriteSignal<PerspectiveKind>,

    /// centralized UI state. see this
    pub ui: UiState,

    id_gen: Arc<AtomicU64>,

    _immutable: (),
}

/// Global single signal store.
#[derive(Debug, Clone, PartialEq)]
pub struct UiState {
    /// Current selected perspective, this is only for UI view.
    pub perspective: ReadSignal<PerspectiveKind>,

    /// Bodies in the application
    pub bodies: Signal<Vec<BodyUI>>,

    _immutable: (),
}

impl UiStore {
    /// New UI state.
    pub fn new(app_store: &AppStore) -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());

        let bodies = app_store.state.bodies;
        let body_list = Signal::derive(move || {
            let mut bodies = bodies.read().values().cloned().collect::<Vec<_>>();
            bodies.sort_by_key(|v| *v.order);

            bodies
        });

        UiStore {
            perspective: set_perspective,
            ui: UiState {
                perspective,
                bodies: body_list,
                _immutable: (),
            },
            id_gen: Arc::new(AtomicU64::new(1)),
            _immutable: (),
        }
    }

    /// Dispatch the [event]
    pub fn dispatch(&self, action: &dyn UiAction) {
        let id = self.id_gen.fetch_add(1, Ordering::Relaxed).into();

        action.apply(self, id);
    }
}

pub trait UiAction {
    /// Apply state change from the event.
    ///
    /// UiState can not mutate directly, allow only exposed write signal
    fn apply(&self, state: &UiStore, id: CommandId) -> Option<Commands>;
}
