/// A representation of position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos(usize, usize);

impl Pos {
    /// Create a new size
    pub fn new(first: usize, second: usize) -> Self {
        Pos(first, second)
    }

    /// Get x of position
    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }

    /// Get y of position
    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos(value.0, value.1)
    }
}

impl From<Pos> for (usize, usize) {
    fn from(value: Pos) -> Self {
        (value.0, value.1)
    }
}
