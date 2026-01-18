use std::{fmt::Display, marker::PhantomData};

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

/// Generator trait for creating unique identifiers.
pub trait GenerateId<T>
where
    T: From<u64>,
{
    /// Generates a new unique identifier.
    ///
    /// # Returns
    /// A new unique identifier of type T.
    fn generate(&mut self) -> T;
}

/// Default implementation of id generator with rng.
pub struct DefaultIdGenerator<T: From<u64>> {
    current: u64,
    _marker: PhantomData<T>,
}

impl<T: From<u64>> Default for DefaultIdGenerator<T> {
    fn default() -> Self {
        Self {
            current: Default::default(),
            _marker: PhantomData,
        }
    }
}

impl<T> GenerateId<T> for DefaultIdGenerator<T>
where
    T: From<u64>,
{
    fn generate(&mut self) -> T {
        let id: u64 = self.current + 1;
        self.current += 1;

        T::from(id)
    }
}
