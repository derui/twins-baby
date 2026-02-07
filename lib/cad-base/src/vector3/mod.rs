#[cfg(test)]
mod tests;

use std::ops::{Add, Div, Mul, Sub};

use crate::{edge::Edge, point::Point};

/// f32-specialized 3D vector
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _immutable: (),
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x,
            y,
            z,
            _immutable: (),
        }
    }

    /// Get a new X unit vector
    pub fn new_x_unit() -> Self {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            _immutable: (),
        }
    }

    /// Get a new Y unit vector
    pub fn new_y_unit() -> Self {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            _immutable: (),
        }
    }

    /// Get a new Z unit vector
    pub fn new_z_unit() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            _immutable: (),
        }
    }

    /// Get dot product with another vector
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Get cross product with another vector
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Return squared norm of the vector
    pub fn norm2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// From edge to a new [Vector3d]
    pub fn from_edge(edge: &Edge) -> Self {
        let start: Vector3 = edge.start().into();
        let end: Vector3 = edge.end().into();

        end - start
    }

    /// Convert to a new unit vector
    pub fn unit(&self) -> Vector3 {
        let norm = self.norm2().sqrt();

        self / norm
    }

    pub fn add(&self, rhs: &Vector3) -> Self {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn subtract(&self, rhs: &Vector3) -> Self {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    pub fn multiply(&self, rhs: f32) -> Self {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
    pub fn divide(&self, rhs: f32) -> Self {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Vector3::new(value.0, value.1, value.2)
    }
}

impl From<Vector3> for (f32, f32, f32) {
    fn from(value: Vector3) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<Point> for Vector3 {
    fn from(value: Point) -> Self {
        Vector3::new(*value.x(), *value.y(), *value.z())
    }
}

impl From<&Point> for Vector3 {
    fn from(value: &Point) -> Self {
        Vector3::new(*value.x(), *value.y(), *value.z())
    }
}

/// operations

// Add
impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        self.add(rhs)
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::add(&self, rhs)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        &self + &rhs
    }
}

// subtracts
impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self.subtract(&rhs)
    }
}
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        self.subtract(rhs)
    }
}

// scalar operations
impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f32)
    }
}

impl Mul<i32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f32)
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide(rhs)
    }
}

/// Default implementation
impl Default for Vector3 {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide(rhs)
    }
}

impl Div<i32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f32)
    }
}

impl Div<i32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f32)
    }
}
