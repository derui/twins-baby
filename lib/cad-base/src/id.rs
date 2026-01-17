use std::marker::PhantomData;

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

/// Generator trait for creating unique identifiers.
pub trait IdGenerator<T>
where
    T: From<u64>,
{
    /// Generates a new unique identifier.
    ///
    /// # Returns
    /// A new unique identifier of type T.
    fn generate<R: rand::Rng + ?Sized>(rng: &mut R) -> T;
}

/// Default implementation of id generator with rng.
pub struct DefaultIdGenerator<T: From<u64>> {
    _marker: PhantomData<T>,
}

impl<T> IdGenerator<T> for DefaultIdGenerator<T>
where
    T: From<u64>,
{
    fn generate<R: rand::Rng + ?Sized>(rng: &mut R) -> T {
        let id: u64 = rng.next_u64();

        T::from(id)
    }
}
