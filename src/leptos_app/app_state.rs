use std::collections::HashMap;

use cad_base::id::BodyId;
use immutable::Im;
use leptos::prelude::*;


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

/// Application State derived by AppStore
#[derive(Debug, Clone, Copy)]
pub struct AppState {
    /// Signal of bodies
    pub bodies: ReadSignal<HashMap<BodyId, BodyUI>>,

    _immutable: (),
}

/// The centralized state of application state. This state is the single source of truth of
/// Application state of **frontend** . This is not the state of beby's 3D engine and CAD data.
#[derive(Debug, Clone, Copy)]
pub struct AppStore {
    /// Bodies in this application
    pub bodies: WriteSignal<HashMap<BodyId, BodyUI>>,

    /// Single state of store
    pub state: AppState,

    _immutable: (),
}

impl AppStore {
    /// New [AppStore]
    pub fn new() -> Self {
        let (bodies, set_bodies) = signal(HashMap::<BodyId, BodyUI>::new());

        AppStore {
            bodies: set_bodies,
            state: AppState {
                bodies,
                _immutable: (),
            },

            _immutable: (),
        }
    }
}
