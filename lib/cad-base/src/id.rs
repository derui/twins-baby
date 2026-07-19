use std::fmt::Debug;

use cad_base_macros::MakeId;

/// A unique identifier for an edge in solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct EdgeId(u64);

/// A unique identifier for a vertex in solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct VertexId(u64);

/// A unique identifier for a sketch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct SketchId(u64);

/// Internal id for manage shape in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct GeometryId(u64);

/// Internal id for constraint management in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct ConstraintId(u64);

/// id for Body
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct BodyId(u64);

/// id for solid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct SolidId(u64);

/// id of surface in the solid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct FaceId(u64);

/// id of surface in the feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, MakeId)]
pub struct FeatureId(u64);

pub trait Id: Clone + Copy + From<u64> + Debug {}
impl<T: Clone + Copy + From<u64> + Debug> Id for T {}

/// An ID store for each concrete Id types.
#[derive(Debug, Clone)]
pub struct IdStore {
    current: u64,

    _immutable: (),
}

impl IdStore {
    pub fn of() -> IdStore {
        IdStore {
            current: 1,
            _immutable: (),
        }
    }

    /// Generates a new unique identifier.
    ///
    /// # Returns
    /// A new unique identifier of type T.
    pub fn generate<T: Id>(&mut self) -> T {
        let current = self.current;
        self.current += 1;
        T::from(current)
    }
}
