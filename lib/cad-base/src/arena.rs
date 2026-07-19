use std::collections::HashMap;
use std::fmt::Debug;

/// internal tag alias
type Tag = u64;

/// A trait for index types that can be used as indices in data structures.
pub trait Index: Clone + Copy + From<u64> + Debug {}
impl<T: Clone + Copy + From<u64> + Debug> Index for T {}

/// A macro to add From<u64> for index
///
/// use like:
/// ```index_impl(SampleIndex)```
#[macro_export]
macro_rules! index_impl {
    ($ty:ident) => {
        impl $ty {
            pub fn new(id: u64) -> Self {
                $ty(id)
            }
        }

        impl From<u64> for $ty {
            fn from(id: u64) -> Self {
                $ty(id)
            }
        }

        impl From<$ty> for u64 {
            fn from(id: $ty) -> Self {
                id.0
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", stringify!($ty), self.0)
            }
        }
    };
}

/// A generation counter for tags. This is used to track the current generation of tags in the system.
#[derive(Debug, Clone)]
pub struct Gen {
    /// mapped internal id of current tags.
    current_generation: HashMap<Tag, u64>,

    /// current index of generation. This will be reset when reset call.
    current: u64,

    /// The generation of arena
    generation: u64,
}

impl Gen {
    /// Create a new generation counter.
    pub fn new() -> Self {
        Self {
            current_generation: HashMap::new(),
            current: 0,
            generation: 1,
        }
    }

    /// no-tag generation.
    pub fn next<I: Index>(&mut self) -> I {
        let next = self.current + 1;
        self.current = next;
        I::from(next)
    }

    /// Re-generate an index for the given tag. This will increment the current index and return a new index.
    ///
    /// This keeps same index while in the same generation by a tag
    pub fn regen<I: Index>(&mut self, tag: Tag) -> I {
        if let Some(v) = self.current_generation.get(&tag) {
            return I::from(*v);
        }

        let next = self.current + 1;
        self.current = next;
        self.current_generation.insert(tag, next);
        I::from(next)
    }

    /// Reset generation.
    pub fn next_generation(&mut self) {
        self.current_generation.clear();
        self.current = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SampleIndex(u64);
    index_impl!(SampleIndex);

    #[test]
    fn regen_returns_same_index_for_same_tag_in_same_generation() {
        // Arrange
        let mut generator = Gen::new();

        // Act
        let first: SampleIndex = generator.regen(1);
        let second: SampleIndex = generator.regen(1);

        // Assert
        assert_eq!(first, second);
    }

    #[test]
    fn regen_returns_different_indices_for_different_tags() {
        // Arrange
        let mut generator = Gen::new();

        // Act
        let first: SampleIndex = generator.regen(1);
        let second: SampleIndex = generator.regen(2);

        // Assert
        assert_ne!(first, second);
    }

    #[test]
    fn next_generation_resets_the_counter() {
        // Arrange
        let mut generator = Gen::new();
        let first_ever: SampleIndex = generator.regen(1);
        let _: SampleIndex = generator.regen(2);

        // Act
        generator.next_generation();
        let first_of_new_generation: SampleIndex = generator.regen(3);

        // Assert
        assert_eq!(first_ever, first_of_new_generation);
    }

    #[test]
    fn next_generation_forgets_previously_registered_tags() {
        // Arrange
        let mut generator = Gen::new();
        let before_reset: SampleIndex = generator.regen(1);
        let _: SampleIndex = generator.regen(2);

        // Act
        generator.next_generation();
        let after_reset: SampleIndex = generator.regen(1);

        // Assert
        assert_eq!(before_reset, after_reset);
    }
}
