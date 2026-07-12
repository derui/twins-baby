mod face_ref;
mod plane_ref;

pub use face_ref::*;
use immutable::Im;
pub use plane_ref::*;

use crate::id::{BodyId, FeatureId};

/// A id-like reference of a feature in a body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureRef {
    /// The ID of the body that contains the feature.
    pub body_id: Im<BodyId>,

    /// The ID of the feature.
    pub feature_id: Im<FeatureId>,
}
