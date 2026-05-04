use std::collections::HashMap;

use cad_base::id::BodyId;
use cad_base::id::SketchId;
use immutable::Im;
use leptos::prelude::*;
use reactive_stores::Store;
use ui_event::PerspectiveKind;

use crate::leptos_app::app_state::AppStore;
use crate::leptos_app::app_state::AppStoreStoreFields;
use crate::leptos_app::app_state::FeatureNode;

macro_rules! derive_body_field {
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

macro_rules! derive_sketch_field {
    ($app:expr, $id:expr, $field:ident : $ty:ty) => {{
        Memo::new(move |_| -> $ty {
            $app.sketches()
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
            $app.sketches()
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
            id: derive_body_field!(store, id, id: copy BodyId),
            name: derive_body_field!(store, id, name: String),
            order: derive_body_field!(store, id, order: copy usize),
            active: derive_body_field!(store, id, active: copy bool),
        }
    }
}

/// States of body perspective
#[derive(Debug, Clone, PartialEq)]
pub struct BodyPerspectiveUI {
    pub can_create_sketch: Im<Signal<bool>>,

    _immutable: (),
}

impl BodyPerspectiveUI {
    /// Create body perspective state
    pub fn from_store(store: Store<AppStore>) -> Self {
        Self {
            can_create_sketch: Signal::derive(move || store.selections().read().len() == 1).into(),
            _immutable: (),
        }
    }
}

/// Signal for bodies
#[derive(Debug, Clone, PartialEq)]
pub struct BodiesUI {
    pub bodies: Im<Signal<Vec<BodyId>>>,

    _immutable: (),
}

impl BodiesUI {
    /// Create bodies signal
    pub fn from_store(store: Store<AppStore>) -> Self {
        let body_list: Signal<Vec<_>> = Memo::new(move |_| {
            store.bodies().with(|bodies| {
                let mut bodies = bodies.clone();
                bodies.sort_by_key(|v| *v.order);

                bodies.iter().map(|it| *it.id).collect::<Vec<_>>()
            })
        })
        .into();

        Self {
            bodies: body_list.into(),
            _immutable: (),
        }
    }
}

/// Immutable UI DTO for Body.
#[derive(Debug, Clone, PartialEq)]
pub struct SketchUI {
    pub id: Im<Signal<SketchId>>,
    pub name: Im<Signal<String>>,
    pub active: Im<Signal<bool>>,
}

impl SketchUI {
    /// Conversion method of body.
    pub fn from_store(store: Store<AppStore>, id: SketchId) -> Self {
        let derived_id: Signal<_> = derive_sketch_field!(store, id, id: copy SketchId);
        let name: Signal<_> = derive_sketch_field!(store, id, name: String);

        Self {
            id: derived_id.into(),
            name: name.into(),
            active: Signal::derive(|| false).into(),
        }
    }
}

/// UI type in tree
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FeatureUIType {
    Sketch(SketchId),
}

impl From<FeatureNode> for FeatureUIType {
    fn from(value: FeatureNode) -> Self {
        match value {
            FeatureNode::Sketch(sketch_state) => FeatureUIType::Sketch(*sketch_state.id),
        }
    }
}

impl From<&FeatureNode> for FeatureUIType {
    fn from(value: &FeatureNode) -> Self {
        match value {
            FeatureNode::Sketch(sketch_state) => FeatureUIType::Sketch(*sketch_state.id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureTreeUI {
    /// Target body of the tree
    pub body_id: Im<Signal<BodyId>>,

    /// Features related to the body. this only have id, so each UI needs to fetch
    /// target data by each type of UI
    pub features: Im<Signal<Vec<FeatureUIType>>>,
}

impl FeatureTreeUI {
    /// Create feature tree UI
    pub fn from_store(store: Store<AppStore>, body_id: BodyId) -> Self {
        let features: Signal<_> = Memo::new(move |_| {
            store.feature_trees().with(|trees| {
                trees
                    .iter()
                    .find(|s| *s.body_id == body_id)
                    .map(|s| s.nodes().iter().map(Into::into).collect::<Vec<_>>())
                    .unwrap_or_default()
            })
        })
        .into();

        let id: Signal<_> = derive_body_field!(store, body_id, id: copy BodyId);

        Self {
            body_id: id.into(),
            features: features.into(),
        }
    }
}
