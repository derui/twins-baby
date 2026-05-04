use cad_base::id::{BodyId, SketchId};
use immutable::Im;
use leptos::prelude::Signal;
use reactive_stores::Store;
use ui_event::{ObjectType, PerspectiveKind, notification::SketchCreatedNotification};

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

/// Types of body childlen.
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureNode {
    Sketch(SketchState),
}

/// Tree of the features. Root must be body
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureTree {
    pub body_id: Im<BodyId>,
    nodes: Vec<FeatureNode>,
}

impl FeatureTree {
    pub fn new(body_id: &BodyId) -> FeatureTree {
        FeatureTree {
            body_id: body_id.clone().into(),
            nodes: Vec::new().into(),
        }
    }

    /// Add a sketch to tree
    pub fn add_sketch(&mut self, state: &SketchState) {
        self.nodes.push(FeatureNode::Sketch(state.clone()).into());
    }

    /// Remove a sketch from tree
    pub fn remove_sketch(&mut self, sketch_id: &SketchId) {
        self.nodes.retain(|node| match node {
            FeatureNode::Sketch(sketch) => *sketch.id != *sketch_id,
        });
    }

    /// Get reference of nodes
    pub fn nodes(&self) -> &[FeatureNode] {
        &self.nodes
    }
}

/// The centralized state of application state. This state is the single source of truth of
/// Application state of **frontend** . This is not the state of beby's 3D engine and CAD data.
#[derive(Debug, Clone, Store, Default)]
pub struct AppStore {
    /// Current selected perspective, this is only for UI view.
    perspective: PerspectiveKind,

    /// Bodies in this application
    bodies: Vec<BodyState>,

    /// Feature trees
    feature_trees: Vec<FeatureTree>,

    /// state of sketches
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
