use std::{
    error::Error,
    f32,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use crate::matrix::{Matrix, simple::SimpleMatrix};

/// Offer simple multi-dimension vector. This works with `matrix` module in this library.

/// A simple vector type
#[derive(Debug, Clone)]
pub struct Vector {
    // A simple element holder
    vec: Vec<f32>,
}

/// Method to convert a vector to a [FloatingMatrix], column or row direction
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransposeMethod {
    Column,
    Row,
}

impl Vector {
    /// Make new vector from slice
    ///
    /// # Arguments
    /// * `vec` : base slice
    ///
    /// # Returns
    /// * new vector. Return `Err` when `vec` is 0-sized slice
    pub fn new(vec: &[f32]) -> Result<Self, Box<dyn Error>> {
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
    pub fn zero(size: usize) -> Result<Self, Box<dyn Error>> {
        if size <= 0 {
            return Err("Can not define 0-dimension vector".into());
        }

        Ok(Vector {
            vec: vec![0.0; size],
        })
    }

    /// Length of this vector
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    /// Change to the matrix.
    ///
    /// # Parameters
    /// * `method` : the method to determine the shape of the converted matrix
    ///
    /// # Return
    /// * New `SimpleMatrix`
    pub fn to_matrix(&self, method: TransposeMethod) -> SimpleMatrix<f32> {
        let rows = match method {
            TransposeMethod::Column => self.len(),
            TransposeMethod::Row => 1,
        };
        let columns = match method {
            TransposeMethod::Column => 1,
            TransposeMethod::Row => self.len(),
        };

        let mut mat = SimpleMatrix::new(rows, columns).expect("Must succeeded");

        let mut update: Box<dyn FnMut(usize, &mut SimpleMatrix<f32>)> = match method {
            TransposeMethod::Column => Box::new(move |idx, mat| -> () {
                mat.set(idx, 0, self[idx]).expect("should success to set");
            }),
            TransposeMethod::Row => Box::new(move |idx, mat| -> () {
                mat.set(0, idx, self[idx]).expect("should success to set");
            }),
        };

        for idx in 0..self.len() {
            update(idx, &mut mat);
        }

        mat
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
