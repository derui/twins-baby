use anyhow::{Result, anyhow};

use crate::matrix::size::Size;

/// A representation of Rect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

pub struct RectBuilder {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

impl RectBuilder {
    pub fn left(mut self, v: usize) -> Self {
        self.left = v;
        self
    }

    pub fn top(mut self, v: usize) -> Self {
        self.top = v;
        self
    }
    pub fn right(mut self, v: usize) -> Self {
        self.right = v;
        self
    }
    pub fn bottom(mut self, v: usize) -> Self {
        self.bottom = v;
        self
    }

    pub fn build(&self) -> Result<Rect> {
        if self.left > self.right {
            return Err(anyhow!(
                "Invalid X-axiz: (left, right) = ({}, {})",
                self.left,
                self.right
            ));
        }

        if self.top > self.bottom {
            return Err(anyhow!(
                "Invalid X-axiz: (top, bottom) = ({}, {})",
                self.top,
                self.bottom
            ));
        }

        Ok(Rect {
            top: self.top,
            left: self.left,
            right: self.right,
            bottom: self.bottom,
        })
    }
}

impl Rect {
    /// Create a new size
    pub fn builder() -> RectBuilder {
        RectBuilder {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        }
    }

    /// Get x of position
    pub fn as_size(&self) -> Size {
        Size::new(self.bottom - self.top, self.right - self.left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0, 0, 0, 0, "all zeros")]
    #[case(0, 0, 5, 5, 5, 5, "5x5 from origin")]
    #[case(1, 2, 3, 4, 2, 2, "offset rect")]
    #[case(10, 20, 30, 40, 20, 20, "large offset rect")]
    fn test_builder_creates_valid_rect(
        #[case] top: usize,
        #[case] left: usize,
        #[case] bottom: usize,
        #[case] right: usize,
        #[case] expected_height: usize,
        #[case] expected_width: usize,
        #[case] description: &str,
    ) {
        // Arrange & Act
        let result = Rect::builder()
            .top(top)
            .left(left)
            .bottom(bottom)
            .right(right)
            .build();

        // Assert
        assert!(result.is_ok(), "{}: expected Ok but got Err", description);
        let rect = result.unwrap();
        let size = rect.as_size();
        assert_eq!(
            size.rows(),
            expected_height,
            "{}: height mismatch",
            description
        );
        assert_eq!(
            size.columns(),
            expected_width,
            "{}: width mismatch",
            description
        );
    }

    #[rstest]
    #[case(0, 5, 0, 0, "left > right")]
    #[case(0, 10, 0, 5, "left > right with larger values")]
    #[case(5, 0, 0, 0, "top > bottom")]
    #[case(10, 0, 0, 5, "top > bottom with larger values")]
    #[case(5, 10, 0, 5, "both invalid")]
    fn test_builder_returns_error_for_invalid_rect(
        #[case] top: usize,
        #[case] left: usize,
        #[case] bottom: usize,
        #[case] right: usize,
        #[case] description: &str,
    ) {
        // Arrange & Act
        let result = Rect::builder()
            .top(top)
            .left(left)
            .bottom(bottom)
            .right(right)
            .build();

        // Assert
        assert!(
            result.is_err(),
            "{}: expected error for invalid rect",
            description
        );
    }

    #[rstest]
    #[case(0, 0, 10, 10, Size::new(10, 10), "square from origin")]
    #[case(5, 5, 15, 15, Size::new(10, 10), "square with offset")]
    #[case(0, 0, 5, 10, Size::new(5, 10), "horizontal rectangle")]
    #[case(0, 0, 10, 5, Size::new(10, 5), "vertical rectangle")]
    #[case(2, 3, 7, 8, Size::new(5, 5), "offset square")]
    fn test_as_size_returns_correct_size(
        #[case] top: usize,
        #[case] left: usize,
        #[case] bottom: usize,
        #[case] right: usize,
        #[case] expected: Size,
        #[case] description: &str,
    ) {
        // Arrange
        let rect = Rect::builder()
            .top(top)
            .left(left)
            .bottom(bottom)
            .right(right)
            .build()
            .unwrap();

        // Act
        let size = rect.as_size();

        // Assert
        assert_eq!(size, expected, "{}: size mismatch", description);
    }

    #[test]
    fn test_builder_methods_are_chainable() {
        // Arrange & Act
        let result = Rect::builder().left(1).top(2).right(3).bottom(4).build();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_rect_default_values() {
        // Arrange & Act
        let rect = Rect::builder().build();

        // Assert
        assert!(rect.is_ok());
        let rect = rect.unwrap();
        assert_eq!(rect.as_size(), Size::new(0, 0));
    }

    #[test]
    fn test_rect_clone_and_equality() {
        // Arrange
        let rect1 = Rect::builder()
            .top(1)
            .left(2)
            .bottom(5)
            .right(6)
            .build()
            .unwrap();

        // Act
        let rect2 = rect1.clone();

        // Assert
        assert_eq!(rect1, rect2);
    }
}
