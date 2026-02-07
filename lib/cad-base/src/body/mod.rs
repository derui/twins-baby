use std::collections::HashMap;

use immutable::Im;

use crate::id::BodyId;

pub struct BodyPerspective {
    /// All bodies in application
    bodies: HashMap<BodyId, Body>,
}

/// The body for the rendering target.
#[derive(Debug, Clone)]
pub struct Body {
    /// Name of body. It will generate automatically when it is created
    pub name: Im<String>,
}
