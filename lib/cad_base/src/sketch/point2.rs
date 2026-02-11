use immutable::Im;

#[derive(Debug, Clone, PartialEq)]
pub struct Point2 {
    pub x: Im<f32>,
    pub y: Im<f32>,
    _immutable: (),
}

impl Point2 {
    /// Get a new [Point2]
    pub fn new(x: f32, y: f32) -> Self {
        Point2 {
            x: x.into(),
            y: y.into(),
            _immutable: (),
        }
    }

    /// Get a distance between two points
    pub fn distance(&self, other: &Point2) -> f32 {
        let x = *other.x - *self.x;
        let y = *other.y - *self.y;

        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl From<(f32, f32)> for Point2 {
    fn from(value: (f32, f32)) -> Self {
        Point2 {
            x: value.0.into(),
            y: value.1.into(),
            _immutable: (),
        }
    }
}

impl From<Point2> for (f32, f32) {
    fn from(value: Point2) -> Self {
        (*value.x, *value.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_points_with_positive_coordinates() {
        // Arrange
        let x = 3.0;
        let y = 4.0;

        // Act
        let point = Point2::new(x, y);

        // Assert
        assert_relative_eq!(*point.x, x);
        assert_relative_eq!(*point.y, y);
    }

    #[test]
    fn test_from_points_with_negative_coordinates() {
        // Arrange
        let x = -5.0;
        let y = -10.0;

        // Act
        let point = Point2::new(x, y);

        // Assert
        assert_relative_eq!(*point.x, x);
        assert_relative_eq!(*point.y, y);
    }

    #[test]
    fn test_from_points_at_origin() {
        // Arrange
        let x = 0.0;
        let y = 0.0;

        // Act
        let point = Point2::new(x, y);

        // Assert
        assert_relative_eq!(*point.x, x);
        assert_relative_eq!(*point.y, y);
    }

    #[test]
    fn test_distance_between_same_point() {
        // Arrange
        let point = Point2::new(5.0, 10.0);

        // Act
        let distance = point.distance(&point);

        // Assert
        assert_relative_eq!(distance, 0.0);
    }

    #[test]
    fn test_distance_horizontal() {
        // Arrange
        let point1 = Point2::new(0.0, 5.0);
        let point2 = Point2::new(10.0, 5.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 10.0);
    }

    #[test]
    fn test_distance_vertical() {
        // Arrange
        let point1 = Point2::new(5.0, 0.0);
        let point2 = Point2::new(5.0, 8.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 8.0);
    }

    #[test]
    fn test_distance_diagonal_pythagorean() {
        // Arrange - 3-4-5 Pythagorean triangle
        let point1 = Point2::new(0.0, 0.0);
        let point2 = Point2::new(3.0, 4.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 5.0);
    }

    #[test]
    fn test_distance_with_negative_coordinates() {
        // Arrange
        let point1 = Point2::new(-3.0, -4.0);
        let point2 = Point2::new(0.0, 0.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 5.0);
    }

    #[test]
    fn test_from_tuple_to_point2d() {
        // Arrange
        let tuple = (7.5, 9.2);

        // Act
        let point: Point2 = tuple.into();

        // Assert
        assert_relative_eq!(*point.x, 7.5);
        assert_relative_eq!(*point.y, 9.2);
    }

    #[test]
    fn test_from_point2d_to_tuple() {
        // Arrange
        let point = Point2::new(3.3, 6.6);

        // Act
        let tuple: (f32, f32) = point.into();

        // Assert
        assert_relative_eq!(tuple.0, 3.3);
        assert_relative_eq!(tuple.1, 6.6);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Arrange
        let original_tuple = (12.5, 25.0);

        // Act
        let point: Point2 = original_tuple.into();
        let result_tuple: (f32, f32) = point.into();

        // Assert
        assert_eq!(result_tuple, original_tuple);
    }
}
