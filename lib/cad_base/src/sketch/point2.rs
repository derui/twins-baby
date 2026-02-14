use std::cmp::Ordering;

use epsilon::Epsilon;
use immutable::Im;

/// A 2D point in sketch space with immutable coordinates.
///
/// Once created, the x and y coordinates cannot be modified in place.
/// Use [`Point2::new`] or convert from a `(f32, f32)` tuple to create an instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Point2 {
    pub x: Im<f32>,
    pub y: Im<f32>,
    _immutable: (),
}

impl Point2 {
    /// Create a new [`Point2`] from the given x and y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Point2 {
            x: x.into(),
            y: y.into(),
            _immutable: (),
        }
    }

    /// Compute the Euclidean distance between `self` and `other`.
    pub fn distance(&self, other: &Point2) -> f32 {
        let x = *other.x - *self.x;
        let y = *other.y - *self.y;

        (x.powi(2) + y.powi(2)).sqrt()
    }

    /// Return `true` if `self` and `other` are approximately equal
    /// within the tolerance defined by the [`Epsilon`] type `E`.
    pub fn approx_eq<E: Epsilon>(&self, other: &Point2) -> bool {
        epsilon::approx_eq::<E>(*self.x, *other.x) && epsilon::approx_eq::<E>(*self.y, *other.y)
    }

    /// Return `true` if `self` and `other` are approximately equal
    /// within the tolerance defined by the [`Epsilon`] type `E`.
    pub fn approx_total_cmp<E: Epsilon>(&self, other: &Point2) -> Ordering {
        match epsilon::approx_total_cmp::<E>(*self.x, *other.x) {
            Ordering::Equal => {},
            ord => return ord
        };

        epsilon::approx_total_cmp::<E>(*self.y, *other.y)
    }

    /// Return `true` if the path `self` -> `o1` -> `o2` makes a
    /// counter-clockwise (CCW) turn.
    ///
    /// Uses the cross-product of vectors (`self` -> `o1`) and (`self` -> `o2`).
    /// Returns `false` when the points are collinear or clockwise.
    pub fn detect_ccw(&self, o1: &Point2, o2: &Point2) -> bool {
        let (ax, ay) = (*self.x, *self.y);
        let (bx, by) = (*o1.x, *o1.y);
        let (cx, cy) = (*o2.x, *o2.y);

        (bx - ax) * (cy - ay) > (cx - ax) * (by - ay)
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

impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.y.partial_cmp(&other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

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
    fn test_approx_eq_same_points() {
        // Arrange
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(1.0, 2.0);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_approx_eq_within_epsilon() {
        // Arrange
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(1.0 + 1e-8, 2.0 - 1e-8);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_approx_eq_different_x() {
        // Arrange
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 2.0);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(!result);
    }

    #[test]
    fn test_approx_eq_different_y() {
        // Arrange
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(1.0, 3.0);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(!result);
    }

    #[test]
    fn test_approx_eq_both_at_origin() {
        // Arrange
        let point1 = Point2::new(0.0, 0.0);
        let point2 = Point2::new(0.0, 0.0);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_approx_eq_negative_coordinates() {
        // Arrange
        let point1 = Point2::new(-1.0, -2.0);
        let point2 = Point2::new(-1.0, -2.0);

        // Act
        let result = point1.approx_eq::<epsilon::DefaultEpsilon>(&point2);

        // Assert
        assert!(result);
    }

    // CCW: right turn along x then up  → counter-clockwise
    // CW:  right turn along x then down → clockwise
    // Collinear (horizontal/vertical/diagonal) → not CCW
    // Negative-coordinate equivalents of CCW/CW
    // Degenerate: all three points identical → not CCW
    #[rstest]
    #[case(
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
        true
    )]
    #[case(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0),
        Point2::new(1.0, 0.0),
        false
    )]
    #[case(
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        false
    )]
    #[case(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0),
        Point2::new(0.0, 2.0),
        false
    )]
    #[case(
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(2.0, 2.0),
        false
    )]
    #[case(Point2::new(-2.0, -1.0), Point2::new(-1.0, -1.0), Point2::new(-2.0, 0.0), true)]
    #[case(Point2::new(-2.0, -1.0), Point2::new(-2.0, 0.0), Point2::new(-1.0, -1.0), false)]
    #[case(
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 1.0),
        false
    )]
    fn test_detect_ccw(
        #[case] p: Point2,
        #[case] o1: Point2,
        #[case] o2: Point2,
        #[case] expected: bool,
    ) {
        // Act
        let result = p.detect_ccw(&o1, &o2);

        // Assert
        assert_eq!(result, expected);
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
