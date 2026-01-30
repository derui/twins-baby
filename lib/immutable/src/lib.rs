use std::ops::Deref;

pub struct Immutable<T> {
    value: T,
    _immutable: (),
}

impl<T> Immutable<T> {
    /// Get a new immutable
    pub fn new(initial: T) -> Self {
        Immutable {
            value: initial,
            _immutable: (),
        }
    }
}

/// Only deref, without mut.
impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
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
        let immutable = Immutable::new(value);

        // Assert
        assert_eq!(*immutable, value);
    }

    #[test]
    fn test_deref_provides_read_access_to_value() {
        // Arrange
        let immutable = Immutable::new(String::from("Hello, World!"));

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
        let immutable = Immutable::new(vec![1, 2, 3, 4, 5]);

        // Act
        let sum: i32 = immutable.iter().sum();
        let length = immutable.len();
        let first = immutable.first();

        // Assert
        assert_eq!(sum, 15);
        assert_eq!(length, 5);
        assert_eq!(first, Some(&1));
    }

    #[derive(Debug, PartialEq)]
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
        let immutable = Immutable::new(custom);

        // Assert
        assert_eq!(immutable.id, 1);
        assert_eq!(immutable.name, "test");
    }

    #[test]
    fn test_immutable_with_empty_collection() {
        // Arrange
        let empty_vec: Vec<i32> = vec![];

        // Act
        let immutable = Immutable::new(empty_vec);

        // Assert
        assert_eq!(immutable.len(), 0);
        assert_eq!(immutable.is_empty(), true);
    }

    #[test]
    fn test_immutable_with_option_types() {
        // Arrange
        let some_value = Immutable::new(Some(42));
        let none_value: Immutable<Option<i32>> = Immutable::new(None);

        // Act & Assert
        assert_eq!(some_value.is_some(), true);
        assert_eq!(*some_value, Some(42));
        assert_eq!(none_value.is_none(), true);
    }
}
