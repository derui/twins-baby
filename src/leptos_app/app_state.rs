use cad_base::id::{BodyId, SketchId};
use immutable::Im;
use reactive_stores::Store;
use ui_event::{ObjectType, notification::SketchCreatedNotification};

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

/// States of sketch.
#[derive(Debug, Clone, PartialEq)]
pub struct SketchState {
    /// Id of the sketch
    pub id: Im<SketchId>,

    /// Name of the sketch
    pub name: Im<String>,

    /// Id of the body that this sketch belongs to
    pub body_id: Im<BodyId>,

    _immutable: (),
}

impl SketchState {
    /// Create a new sketch state
    pub fn new(id: SketchId, name: &str, body_id: BodyId) -> SketchState {
        SketchState {
            id: id.into(),
            name: name.to_string().into(),
            body_id: body_id.into(),

            _immutable: (),
        }
    }
}

impl From<&SketchCreatedNotification> for SketchState {
    fn from(notification: &SketchCreatedNotification) -> Self {
        SketchState {
            id: notification.sketch_id.clone(),
            name: notification.name.clone(),
            body_id: notification.body_id.clone(),

            _immutable: (),
        }
    }
}

impl From<SketchCreatedNotification> for SketchState {
    fn from(notification: SketchCreatedNotification) -> Self {
        SketchState {
            id: notification.sketch_id,
            name: notification.name,
            body_id: notification.body_id,

            _immutable: (),
        }
    }
}

/// The centralized state of application state. This state is the single source of truth of
/// Application state of **frontend** . This is not the state of beby's 3D engine and CAD data.
#[derive(Debug, Clone, Store, Default)]
pub struct AppStore {
    /// Bodies in this application
    bodies: Vec<BodyState>,

    /// Sketches in this application
    sketches: Vec<SketchState>,

    /// Selections in CAD
    selections: Vec<ObjectType>,
}

impl AppStore {
    /// New [AppStore]
    pub fn new() -> Store<AppStore> {
        Store::new(Default::default())
    }
}
