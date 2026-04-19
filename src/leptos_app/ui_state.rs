use std::sync::{Arc, atomic::AtomicU64};

use cad_base::id::BodyId;
use immutable::Im;
use leptos::prelude::*;
use ui_event::PerspectiveKind;

use crate::leptos_app::app_state::AppStore;

/// Immutable UI DTO for Body.
#[derive(Debug, Clone)]
pub struct BodyUI {
    pub id: Im<BodyId>,
    pub name: Im<String>,
    pub order: Im<usize>,
    pub active: Im<bool>,

    _immutable: (),
}

impl BodyUI {
    /// Make new [BodyUI]
    pub fn new(id: BodyId, name: &str, order: usize, active: bool) -> BodyUI {
        BodyUI {
            id: id.into(),
            name: name.to_string().into(),
            order: order.into(),
            active: active.into(),
            _immutable: (),
        }
    }

    /// Make active the body
    pub fn active(&mut self) {
        self.active = true.into()
    }

    /// Make inactive the body
    pub fn inactive(&mut self) {
        self.active = false.into()
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
                perspective: perspective.into(),
                bodies: body_list,
                _immutable: (),
            },
            id_gen: Arc::new(AtomicU64::new(1)),
            _immutable: (),
        }
    }
}
