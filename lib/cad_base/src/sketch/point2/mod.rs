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
            Ordering::Equal => {}
            ord => return ord,
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
mod tests;
