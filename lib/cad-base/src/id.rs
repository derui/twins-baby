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
        write!(f, "Point{}", self.0)
    }
}

/// Generator trait for creating unique identifiers.
pub trait GenerateId<T>: std::fmt::Debug + GenerateIdClone<T>
where
    T: From<u64> + Clone,
{
    /// Generates a new unique identifier.
    ///
    /// # Returns
    /// A new unique identifier of type T.
    fn generate(&mut self) -> T;
}

pub trait GenerateIdClone<T>
where
    T: From<u64>,
{
    fn clone_box(&self) -> Box<dyn GenerateId<T>>;
}

impl<T: From<u64> + Clone> Clone for Box<dyn GenerateIdClone<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Default implementation of id generator with rng.
#[derive(Debug, Clone)]
pub struct DefaultIdGenerator<T: From<u64>> {
    current: u64,
    _marker: PhantomData<T>,
}

impl<T: From<u64>> Default for DefaultIdGenerator<T> {
    fn default() -> Self {
        Self {
            current: 1,
            _marker: PhantomData,
        }
    }
}

impl<T, V> GenerateIdClone<T> for V
where
    T: From<u64> + Debug + Clone,
    V: 'static + Clone + GenerateId<T>,
{
    fn clone_box(&self) -> Box<dyn GenerateId<T>> {
        Box::new(self.clone())
    }
}

impl<T: From<u64> + Debug> Clone for Box<dyn GenerateId<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<T> GenerateId<T> for DefaultIdGenerator<T>
where
    T: From<u64> + std::fmt::Debug + Clone + 'static,
{
    fn generate(&mut self) -> T {
        let id: u64 = self.current;
        self.current += 1;

        T::from(id)
    }
}
