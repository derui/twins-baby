#[cfg(test)]
mod tests;

use std::ops::{Add, Div, Mul, Sub};

use crate::{edge::Edge, point::Point};

/// f32-specialized 3D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d(f32, f32, f32);

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d(x, y, z)
    }

    /// Get a new X unit vector
    pub fn new_x_unit() -> Self {
        Vector3d(1.0, 0.0, 0.0)
    }

    /// Get a new Y unit vector
    pub fn new_y_unit() -> Self {
        Vector3d(0.0, 1.0, 0.0)
    }

    /// Get a new Z unit vector
    pub fn new_z_unit() -> Self {
        Vector3d(0.0, 0.0, 1.0)
    }

    /// Get reference of X
    pub fn x(&self) -> &f32 {
        &self.0
    }

    /// Get reference of Y
    pub fn y(&self) -> &f32 {
        &self.1
    }

    /// Get reference of Z
    pub fn z(&self) -> &f32 {
        &self.2
    }

    /// Get dot product with another vector
    pub fn dot(&self, other: &Vector3d) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Get cross product with another vector
    pub fn cross(&self, other: &Vector3d) -> Vector3d {
        Vector3d::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    /// Return squared norm of the vector
    pub fn norm2(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// From edge to a new [Vector3d]
    pub fn from_edge(edge: &Edge) -> Self {
        let start: Vector3d = edge.start().into();
        let end: Vector3d = edge.end().into();

        end - start
    }

    /// Convert to a new unit vector
    pub fn unit(&self) -> Vector3d {
        let norm = self.norm2().sqrt();

        self / norm
    }

    pub fn add(&self, rhs: &Vector3d) -> Self {
        Vector3d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }

    pub fn subtract(&self, rhs: &Vector3d) -> Self {
        Vector3d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }

    pub fn multiply(&self, rhs: f32) -> Self {
        Vector3d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
    pub fn divide(&self, rhs: f32) -> Self {
        Vector3d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl From<(f32, f32, f32)> for Vector3d {
    fn from(value: (f32, f32, f32)) -> Self {
        Vector3d::new(value.0, value.1, value.2)
    }
}

impl From<Vector3d> for (f32, f32, f32) {
    fn from(value: Vector3d) -> Self {
        (value.0, value.1, value.2)
    }
}

impl From<Point> for Vector3d {
    fn from(value: Point) -> Self {
        Vector3d(*value.x(), *value.y(), *value.z())
    }
}

impl From<&Point> for Vector3d {
    fn from(value: &Point) -> Self {
        Vector3d(*value.x(), *value.y(), *value.z())
    }
}

/// operations

// Add
impl Add<&Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &Vector3d) -> Self::Output {
        self.add(rhs)
    }
}

impl Add<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &Vector3d) -> Self::Output {
        Vector3d::add(&self, rhs)
    }
}

impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        &self + &rhs
    }
}

// subtracts
impl Sub<&Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: &Vector3d) -> Self::Output {
        self.subtract(rhs)
    }
}

impl Sub<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        self.subtract(&rhs)
    }
}
impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: &Vector3d) -> Self::Output {
        self.subtract(rhs)
    }
}

// scalar operations
impl Mul<f32> for &Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<i32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f32)
    }
}

impl Mul<i32> for &Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f32)
    }
}

impl Div<f32> for &Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide(rhs)
    }
}

/// Default implementation
impl Default for Vector3d {
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide(rhs)
    }
}

impl Div<i32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f32)
    }
}

impl Div<i32> for &Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f32)
    }
}
