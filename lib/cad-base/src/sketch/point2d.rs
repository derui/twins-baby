#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D(f32, f32);

impl Point2D {
    pub fn from_points(x: f32, y: f32) -> Self {
        Point2D(x, y)
    }

    /// Get a distance between two points
    pub fn distance(&self, other: &Point2D) -> f32 {
        let x = other.0 - self.0;
        let y = other.1 - self.1;

        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn x(&self) -> &f32 {
        &self.0
    }

    pub fn y(&self) -> &f32 {
        &self.1
    }
}

impl From<(f32, f32)> for Point2D {
    fn from(value: (f32, f32)) -> Self {
        Point2D(value.0, value.1)
    }
}

impl From<Point2D> for (f32, f32) {
    fn from(value: Point2D) -> Self {
        (value.0, value.1)
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
        let point = Point2D::from_points(x, y);

        // Assert
        assert_relative_eq!(point.x(), &x);
        assert_relative_eq!(point.y(), &y);
    }

    #[test]
    fn test_from_points_with_negative_coordinates() {
        // Arrange
        let x = -5.0;
        let y = -10.0;

        // Act
        let point = Point2D::from_points(x, y);

        // Assert
        assert_relative_eq!(point.x(), &x);
        assert_relative_eq!(point.y(), &y);
    }

    #[test]
    fn test_from_points_at_origin() {
        // Arrange
        let x = 0.0;
        let y = 0.0;

        // Act
        let point = Point2D::from_points(x, y);

        // Assert
        assert_relative_eq!(point.x(), &x);
        assert_relative_eq!(point.y(), &y);
    }

    #[test]
    fn test_distance_between_same_point() {
        // Arrange
        let point = Point2D::from_points(5.0, 10.0);

        // Act
        let distance = point.distance(&point);

        // Assert
        assert_relative_eq!(distance, 0.0);
    }

    #[test]
    fn test_distance_horizontal() {
        // Arrange
        let point1 = Point2D::from_points(0.0, 5.0);
        let point2 = Point2D::from_points(10.0, 5.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 10.0);
    }

    #[test]
    fn test_distance_vertical() {
        // Arrange
        let point1 = Point2D::from_points(5.0, 0.0);
        let point2 = Point2D::from_points(5.0, 8.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 8.0);
    }

    #[test]
    fn test_distance_diagonal_pythagorean() {
        // Arrange - 3-4-5 Pythagorean triangle
        let point1 = Point2D::from_points(0.0, 0.0);
        let point2 = Point2D::from_points(3.0, 4.0);

        // Act
        let distance = point1.distance(&point2);

        // Assert
        assert_relative_eq!(distance, 5.0);
    }

    #[test]
    fn test_distance_with_negative_coordinates() {
        // Arrange
        let point1 = Point2D::from_points(-3.0, -4.0);
        let point2 = Point2D::from_points(0.0, 0.0);

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
        let point: Point2D = tuple.into();

        // Assert
        assert_relative_eq!(point.x(), &7.5);
        assert_relative_eq!(point.y(), &9.2);
    }

    #[test]
    fn test_from_point2d_to_tuple() {
        // Arrange
        let point = Point2D::from_points(3.3, 6.6);

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
        let point: Point2D = original_tuple.into();
        let result_tuple: (f32, f32) = point.into();

        // Assert
        assert_eq!(result_tuple, original_tuple);
    }
}
