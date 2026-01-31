use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// A unique identifier for a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaneId(u64);

impl PlaneId {
    /// Creates a new PlaneId.
    pub fn new(id: u64) -> Self {
        PlaneId(id)
    }
}

impl From<u64> for PlaneId {
    fn from(id: u64) -> Self {
        PlaneId(id)
    }
}

impl Display for PlaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plane{}", self.0)
    }
}

/// A unique identifier for an edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(u64);

impl EdgeId {
    /// Creates a new EdgeId.
    pub fn new(id: u64) -> Self {
        EdgeId(id)
    }

    #[inline]
    pub fn id(&self) -> &u64 {
        &self.0
    }
}

impl From<u64> for EdgeId {
    fn from(id: u64) -> Self {
        EdgeId(id)
    }
}

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge{}", self.0)
    }
}

/// A unique identifier for a point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointId(u64);

impl PointId {
    /// Creates a new PointId.
    pub fn new(id: u64) -> Self {
        PointId(id)
    }

    #[inline]
    pub fn id(&self) -> &u64 {
        &self.0
    }
}

impl From<u64> for PointId {
    fn from(id: u64) -> Self {
        PointId(id)
    }
}

impl Display for PointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point{}", self.0)
    }
}

/// A unique identifier for a sketch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SketchId(u64);

impl SketchId {
    /// Creates a new SketchId.
    pub fn new(id: u64) -> Self {
        SketchId(id)
    }
}

impl From<u64> for SketchId {
    fn from(id: u64) -> Self {
        SketchId(id)
    }
}

impl Display for SketchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sketch{}", self.0)
    }
}

/// Internal id for manage variable in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VariableId(u64);

impl Display for VariableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Var{}", self.0)
    }
}

impl From<u64> for VariableId {
    fn from(value: u64) -> Self {
        VariableId(value)
    }
}

impl From<VariableId> for u64 {
    fn from(value: VariableId) -> Self {
        value.0
    }
}

/// Internal id for manage shape in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryId(u64);

impl Display for GeometryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geo{}", self.0)
    }
}

impl From<u64> for GeometryId {
    fn from(value: u64) -> Self {
        GeometryId(value)
    }
}

impl From<GeometryId> for u64 {
    fn from(value: GeometryId) -> Self {
        value.0
    }
}

/// Internal id for constraint management in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConstraintId(u64);

impl Display for ConstraintId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Constraint{}", self.0)
    }
}

impl From<u64> for ConstraintId {
    fn from(value: u64) -> Self {
        ConstraintId(value)
    }
}

impl From<ConstraintId> for u64 {
    fn from(value: ConstraintId) -> Self {
        value.0
    }
}

pub trait Id: Clone + Copy + From<u64> + Debug {}
impl<T: Clone + Copy + From<u64> + Debug> Id for T {}

/// An ID store for each concrete Id types.
#[derive(Debug, Clone)]
pub struct IdStore<T: Id> {
    current: u64,
    _marker: PhantomData<T>,
}

impl<T: Id> IdStore<T> {
    pub fn of() -> IdStore<T> {
        IdStore {
            current: 1,
            _marker: PhantomData,
        }
    }

    /// Generates a new unique identifier.
    ///
    /// # Returns
    /// A new unique identifier of type T.
    pub fn generate(&mut self) -> T {
        let current = self.current;
        self.current += 1;
        T::from(current)
    }
}
