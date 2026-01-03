use std::{
    error::Error,
    ops::{Add, Mul, Sub},
};

/// Offer simple multi-dimension vector. This works with `matrix` module in this library.

/// A simple vector type
#[derive(Debug, Clone)]
pub struct Vector {
    // A simple element holder
    vec: Vec<f32>,
}

impl Vector {
    /// Make new vector from slice
    ///
    /// # Arguments
    /// * `vec` : base slice
    ///
    /// # Returns
    /// * new vector. Return `Err` when `vec` is 0-sized slice
    fn new(vec: &[f32]) -> Result<Self, Box<dyn Error>> {
        if vec.is_empty() {
            return Err("Can not define 0-dimension vector".into());
        }

        Ok(Vector { vec: vec.to_vec() })
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            vec: self.vec.iter().map(|f| f * rhs).collect(),
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Result<Vector, Box<dyn Error>>;

    fn add(self, rhs: Vector) -> Self::Output {
        if self.vec.len() != rhs.vec.len() {
            return Err(format!(
                "Can not add different dimension, {} <> {}",
                self.vec.len(),
                rhs.vec.len()
            )
            .into());
        }
        let mut result = self.vec.clone();

        for (i, v) in rhs.vec.iter().enumerate() {
            result[i] += v;
        }

        Ok(Vector { vec: result })
    }
}

impl Sub<Vector> for Vector {
    type Output = Result<Vector, Box<dyn Error>>;

    fn sub(self, rhs: Vector) -> Self::Output {
        if self.vec.len() != rhs.vec.len() {
            return Err(format!(
                "Can not subtract different dimension, {} <> {}",
                self.vec.len(),
                rhs.vec.len()
            )
            .into());
        }
        let mut result = self.vec.clone();

        for (i, v) in rhs.vec.iter().enumerate() {
            result[i] -= v;
        }

        Ok(Vector { vec: result })
    }
}
