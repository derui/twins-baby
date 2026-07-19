//# Tag module

/// A tag is a unique identifier for an entity in the CAD model. It is used to reference entities in the model and to store metadata about them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceTag(u64);

/// A tag is a unique identifier for a variable in the CAD model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariableTag(u64);

/// A tag is a unique identifier for a solid in the CAD model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SolidTag(u64);
