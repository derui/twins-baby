use reactive_stores::Store;

use crate::leptos_app::ui_state::BodyUI;

/// The centralized state of application state. This state is the single source of truth of
/// Application state of **frontend** . This is not the state of beby's 3D engine and CAD data.
#[derive(Debug, Clone, Store)]
pub struct AppStore {
    /// Bodies in this application
    pub bodies: Vec<BodyUI>,

    _immutable: (),
}

impl AppStore {
    /// New [AppStore]
    pub fn new() -> Store<AppStore> {
        Store::new(AppStore {
            bodies: Vec::new(),

            _immutable: (),
        })
    }
}
