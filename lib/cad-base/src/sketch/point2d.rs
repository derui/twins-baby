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
