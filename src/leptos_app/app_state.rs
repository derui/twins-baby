use cad_base::id::BodyId;
use immutable::Im;
use leptos::prelude::Signal;
use reactive_stores::Store;

use crate::leptos_app::ui_state::BodyUI;

#[derive(Debug, Clone, PartialEq)]
pub struct BodyState {
    pub id: Im<BodyId>,
    pub name: Im<String>,
    pub order: Im<usize>,
    pub active: Im<bool>,

    _immutable: (),
}

impl BodyState {
    pub fn new(id: BodyId, name: &str, order: usize) -> BodyState {
        BodyState {
            id: id.into(),
            name: name.to_string().into(),
            order: order.into(),
            active: false.into(),

            _immutable: (),
        }
    }

    /// Marks the body as active.
    pub fn activate(&mut self) {
        self.active = true.into();
    }

    /// Marks the body as inactive.
    pub fn deactivate(&mut self) {
        self.active = false.into();
    }
}

impl From<BodyState> for BodyUI {
    fn from(state: BodyState) -> Self {
        BodyUI {
            id: Signal::derive(move || *state.id),
            name: Signal::derive(move || (*state.name).clone()),
            order: Signal::derive(move || *state.order),
            active: Signal::derive(move || *state.active),
        }
    }
}

/// The centralized state of application state. This state is the single source of truth of
/// Application state of **frontend** . This is not the state of beby's 3D engine and CAD data.
#[derive(Debug, Clone, Store)]
pub struct AppStore {
    /// Bodies in this application
    bodies: Vec<BodyState>,
}

impl AppStore {
    /// New [AppStore]
    pub fn new() -> Store<AppStore> {
        Store::new(AppStore { bodies: Vec::new() })
    }
}
