use std::{
    error::Error,
    f32,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
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

    /// Make new zero vector
    ///
    /// # Arguments
    /// * `size` : size of the new vector
    ///
    /// # Returns
    /// * new vector unless `size` is lesser than 1
    fn zero(size: usize) -> Result<Self, Box<dyn Error>> {
        if size <= 0 {
            return Err("Can not define 0-dimension vector".into());
        }

        Ok(Vector {
            vec: vec![0.0; size],
        })
    }
}

impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.vec.index(index)
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.vec.index_mut(index)
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

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            vec: self.vec.iter().map(|f| f / rhs).collect(),
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
