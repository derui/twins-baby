use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use cad_base::id::BodyId;
use leptos::prelude::*;
use ui_event::CommandId;

use crate::leptos_app::ui_state::BodyUI;

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
    pub gen_id: Callback<(), CommandId>,

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
        let id_gen = Arc::new(AtomicU64::new(0));

        let gen_id = Callback::new(move |_| {
            let id = id_gen.fetch_add(1, Ordering::Relaxed);

            id.into()
        });

        AppStore {
            bodies: set_bodies,
            gen_id,
            state: AppState {
                bodies,
                _immutable: (),
            },

            _immutable: (),
        }
    }
}
