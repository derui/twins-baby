use std::{clone::Clone, ops::Deref};

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct Im<T: Clone>(T);

impl<T: Clone> Im<T> {
    /// Get a new immutable
    pub fn new(initial: T) -> Self {
        Im(initial)
    }
}

/// Only deref, without mut.
impl<T: Clone> Deref for Im<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> From<T> for Im<T> {
    fn from(value: T) -> Self {
        Im::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(42)]
    fn test_new_creates_immutable_with_value(#[case] value: i32) {
        // Arrange & Act
        let immutable = Im::new(value);

        // Assert
        assert_eq!(*immutable, value);
    }

    #[rstest]
    #[case(42)]
    fn test_new_creates_immutable_from_value(#[case] value: i32) {
        // Arrange & Act
        let immutable: Im<_> = value.into();

        // Assert
        assert_eq!(*immutable, value);
    }

    #[test]
    fn test_deref_provides_read_access_to_value() {
        // Arrange
        let immutable = Im::new(String::from("Hello, World!"));

        // Act
        let lowercase = immutable.to_lowercase();
        let length = immutable.len();

        // Assert
        assert_eq!(lowercase, "hello, world!");
        assert_eq!(length, 13);
    }

    #[test]
    fn test_deref_allows_multiple_reads() {
        // Arrange
        let immutable = Im::new(vec![1, 2, 3, 4, 5]);

        // Act
        let sum: i32 = immutable.iter().sum();
        let length = immutable.len();
        let first = immutable.first();

        // Assert
        assert_eq!(sum, 15);
        assert_eq!(length, 5);
        assert_eq!(first, Some(&1));
    }

    #[derive(Debug, PartialEq, Clone)]
    struct CustomStruct {
        id: u32,
        name: String,
    }

    #[test]
    fn test_immutable_with_custom_type() {
        // Arrange
        let custom = CustomStruct {
            id: 1,
            name: "test".to_string(),
        };

        // Act
        let immutable = Im::new(custom);

        // Assert
        assert_eq!(immutable.id, 1);
        assert_eq!(immutable.name, "test");
    }

    #[test]
    fn test_immutable_with_empty_collection() {
        // Arrange
        let empty_vec: Vec<i32> = vec![];

        // Act
        let immutable = Im::new(empty_vec);

        // Assert
        assert_eq!(immutable.len(), 0);
        assert_eq!(immutable.is_empty(), true);
    }

    #[test]
    fn test_immutable_with_option_types() {
        // Arrange
        let some_value = Im::new(Some(42));
        let none_value: Im<Option<i32>> = Im::new(None);

        // Act & Assert
        assert_eq!(some_value.is_some(), true);
        assert_eq!(*some_value, Some(42));
        assert_eq!(none_value.is_none(), true);
    }
}
