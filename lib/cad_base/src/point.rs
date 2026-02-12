use std::fmt::{Display, Formatter};

use immutable::Im;

/// definition of point
///
/// This type is totally immutable
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: Im<f32>,
    pub y: Im<f32>,
    pub z: Im<f32>,
    _immutable: (),
}

impl Point {
    /// Get a new [Point]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            _immutable: (),
        }
    }

    /// Get a new zero point
    pub fn zero() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(value: (f32, f32, f32)) -> Self {
        Point::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl From<Point> for (f32, f32, f32) {
    fn from(value: Point) -> Self {
        (*value.x, *value.y, *value.z)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", *self.x, *self.y, *self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_creates_point_with_coordinates() {
        // Arrange
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;

        // Act
        let point = Point::new(x, y, z);

        // Assert
        assert_relative_eq!(*point.x, 1.0);
        assert_relative_eq!(*point.y, 2.0);
        assert_relative_eq!(*point.z, 3.0);
    }

    #[test]
    fn from_tuple_creates_point() {
        // Arrange
        let tuple = (1.0, 2.0, 3.0);

        // Act
        let point: Point = tuple.into();

        // Assert
        assert_relative_eq!(*point.x, 1.0);
        assert_relative_eq!(*point.y, 2.0);
        assert_relative_eq!(*point.z, 3.0);
    }

    #[test]
    fn into_tuple_converts_point() {
        // Arrange
        let point = Point::new(1.0, 2.0, 3.0);

        // Act
        let tuple: (f32, f32, f32) = point.into();

        // Assert
        assert_eq!(tuple, (1.0, 2.0, 3.0));
    }

    #[test]
    fn points_with_same_coordinates_are_equal() {
        // Arrange
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);

        // Act & Assert
        assert_eq!(p1, p2);
    }

    #[test]
    fn display_formats_as_tuple() {
        // Arrange
        let point = Point::new(1.0, 2.0, 3.0);

        // Act
        let result = format!("{}", point);

        // Assert
        assert_eq!(result, "(1, 2, 3)");
    }
}
