mod plane_ref;
mod resolve;

use immutable::Im;
pub use plane_ref::*;
pub use resolve::*;

use crate::{id::FeatureId, tag::FaceTag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceRef {
    /// The ID of the body that contains the face.
    pub feature: Im<FeatureId>,

    /// The ID of the face from the feature.
    pub face: Im<FaceTag>,
}

impl FaceRef {
    /// Create a new FaceRef with the given feature ID and face tag.
    pub fn new(feature: FeatureId, face: FaceTag) -> Self {
        FaceRef {
            feature: feature.into(),
            face: face.into(),
        }
    }
}
