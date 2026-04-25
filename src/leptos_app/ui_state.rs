use cad_base::id::BodyId;
use leptos::prelude::*;
use reactive_stores::Store;
use ui_event::PerspectiveKind;

use crate::leptos_app::app_state::AppStore;
use crate::leptos_app::app_state::AppStoreStoreFields;

macro_rules! derive_field {
    ($app:expr, $id:expr, $field:ident : $ty:ty) => {{
        Memo::new(move |_| -> $ty {
            $app.bodies()
                .read()
                .iter()
                .find(|b| *b.id == $id)
                .map(|b| (*b.$field).clone())
                .expect(&format!("should be found id: {:?}", $id))
        })
        .into()
    }};
    // Copyな型用
    ($app:expr, $id:expr, $field:ident : copy $ty:ty) => {{
        Memo::new(move |_| -> $ty {
            $app.bodies()
                .read()
                .iter()
                .find(|b| *b.id == $id)
                .map(|b| *b.$field)
                .expect(&format!("should be found id: {:?}", $id))
        })
        .into()
    }};
}

/// Immutable UI DTO for Body.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyUI {
    pub id: Signal<BodyId>,
    pub name: Signal<String>,
    pub order: Signal<usize>,
    pub active: Signal<bool>,
}

impl BodyUI {
    /// Conversion method of body.
    pub fn from_store(store: Store<AppStore>, id: BodyId) -> BodyUI {
        BodyUI {
            id: derive_field!(store, id, id: copy BodyId),
            name: derive_field!(store, id, name: String),
            order: derive_field!(store, id, order: copy usize),
            active: derive_field!(store, id, active: copy bool),
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

    _immutable: (),
}

/// Global single signal store.
#[derive(Debug, Clone, PartialEq)]
pub struct UiState {
    /// Current selected perspective, this is only for UI view.
    pub perspective: Signal<PerspectiveKind>,

    /// Bodies in the application
    pub bodies: Signal<Vec<BodyId>>,

    _immutable: (),
}

impl UiStore {
    /// Create new UI state
    pub fn new(store: Store<AppStore>) -> Self {
        let (perspective, set_perspective) = signal(PerspectiveKind::default());

        let body_list: Memo<Vec<_>> = Memo::new(move |_| {
            store.bodies().with(|bodies| {
                let mut bodies = bodies.clone();
                bodies.sort_by_key(|v| *v.order);

                bodies.iter().map(|it| *it.id).collect::<Vec<_>>()
            })
        });

        UiStore {
            perspective: set_perspective,
            ui: UiState {
                perspective: perspective.into(),
                bodies: body_list.into(),
                _immutable: (),
            },
            _immutable: (),
        }
    }
}
