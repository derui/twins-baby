/// A representation of size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(usize, usize);

impl Size {
    /// Create a new size
    pub fn new(first: usize, second: usize) -> Self {
        Size(first, second)
    }

    /// Get number of rows
    #[inline]
    pub fn rows(&self) -> usize {
        self.0
    }

    /// Get number of columns
    #[inline]
    pub fn columns(&self) -> usize {
        self.1
    }

    /// Get minimum value of the size
    #[inline]
    pub fn min(&self) -> usize {
        if self.0 >= self.1 { self.1 } else { self.0 }
    }
}
