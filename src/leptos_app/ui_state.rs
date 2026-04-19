use std::sync::{Arc, atomic::AtomicU64};

use cad_base::id::BodyId;
use leptos::prelude::*;
use reactive_stores::Store;
use ui_event::PerspectiveKind;

use crate::leptos_app::app_state::AppStore;
use crate::leptos_app::app_state::AppStoreStoreFields;

/// Immutable UI DTO for Body.
#[derive(Debug, Clone)]
pub struct BodyUI {
    pub id: ReadSignal<BodyId>,
    pub name: ReadSignal<String>,
    pub order: ReadSignal<usize>,
    pub active: ReadSignal<bool>,

    set_active: WriteSignal<bool>,
}

impl BodyUI {
    /// Make new [BodyUI]
    pub fn new(id: BodyId, name: &str, order: usize) -> BodyUI {
        let (active, set_active) = signal(false);

        BodyUI {
            id: signal(id).0,
            name: signal(name.to_string()).0,
            order: signal(order).0,
            active,

            set_active,
        }
    }

    /// Marks the body as active.
    pub fn active(&mut self) {
        self.set_active.set(true)
    }

    /// Marks the body as inactive.
    pub fn inactive(&mut self) {
        self.set_active.set(false)
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
    pub perspective: Signal<PerspectiveKind>,

    /// Bodies in the application
    pub bodies: Signal<Vec<BodyUI>>,

    _immutable: (),
}

impl UiStore {
    /// New UI state.
    pub fn new(app_store: &Store<AppStore>) -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());

        let bodies = app_store.bodies();
        let body_list = Signal::derive(move || {
            let mut bodies = bodies.read().iter().cloned().collect::<Vec<_>>();
            bodies.sort_by_key(|v| v.order.get_untracked());

            bodies
        });

        UiStore {
            perspective: set_perspective,
            ui: UiState {
                perspective: perspective.into(),
                bodies: body_list,
                _immutable: (),
            },
            id_gen: Arc::new(AtomicU64::new(1)),
            _immutable: (),
        }
    }
}
