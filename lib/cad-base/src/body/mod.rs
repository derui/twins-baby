use std::collections::HashMap;

use immutable::Im;

use crate::id::{BodyId, IdStore};

pub struct BodyPerspective {
    /// All bodies in application
    bodies: HashMap<BodyId, Body>,

    /// body id generator
    body_id_gen: IdStore<BodyId>,
}

impl BodyPerspective {
    /// Create a new perspective
    pub fn new() -> Self {
        BodyPerspective {
            bodies: Default::default(),
            body_id_gen: IdStore::of(),
        }
    }
}

/// The body for the rendering target.
#[derive(Debug, Clone)]
pub struct Body {
    /// Name of body. It will generate automatically when it is created
    pub name: Im<String>,
}
