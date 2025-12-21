/// A representation of size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(usize, usize);

impl Size {
    /// Create a new size
    fn new(first: usize, second: usize) -> Self {
        Size(first, second)
    }
}
