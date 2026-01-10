use std::{
    f32,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use anyhow::Result;

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
    pub fn new(vec: &[f32]) -> Result<Self, anyhow::Error> {
        if vec.is_empty() {
            return Err(anyhow::anyhow!("Can not define 0-dimension vector"));
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
    pub fn zero(size: usize) -> Result<Self, anyhow::Error> {
        if size == 0 {
            return Err(anyhow::anyhow!("Can not define 0-dimension vector"));
        }

        Ok(Vector {
            vec: vec![0.0; size],
        })
    }

    /// Length of this vector
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.vec.is_empty()
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

    /// Compute norm of the vector
    pub fn norm(&self) -> f32 {
        let f = self.vec.iter().map(|f| f * f).sum::<f32>();

        f32::sqrt(f)
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
    type Output = Result<Vector, anyhow::Error>;

    fn add(self, rhs: Vector) -> Self::Output {
        if self.vec.len() != rhs.vec.len() {
            return Err(anyhow::anyhow!(
                "Can not add different dimension, {} <> {}",
                self.vec.len(),
                rhs.vec.len()
            ));
        }
        let mut result = self.vec.clone();

        for (i, v) in rhs.vec.iter().enumerate() {
            result[i] += v;
        }

        Ok(Vector { vec: result })
    }
}

impl Sub<Vector> for Vector {
    type Output = Result<Vector, anyhow::Error>;

    fn sub(self, rhs: Vector) -> Self::Output {
        if self.vec.len() != rhs.vec.len() {
            return Err(anyhow::anyhow!(
                "Can not subtract different dimension, {} <> {}",
                self.vec.len(),
                rhs.vec.len()
            ));
        }
        let mut result = self.vec.clone();

        for (i, v) in rhs.vec.iter().enumerate() {
            result[i] -= v;
        }

        Ok(Vector { vec: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], 3)]
    #[case(&[5.0], 1)]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], 5)]
    fn test_new_creates_vector_with_correct_values(
        #[case] values: &[f32],
        #[case] expected_len: usize,
    ) -> Result<(), anyhow::Error> {
        // Arrange & Act
        let vector = Vector::new(values)?;

        // Assert
        assert_eq!(vector.len(), expected_len);
        for (i, &expected_value) in values.iter().enumerate() {
            assert_eq!(vector[i], expected_value);
        }
        Ok(())
    }

    #[test]
    fn test_new_returns_error_for_empty_slice() {
        // Arrange & Act
        let result = Vector::new(&[]);

        // Assert
        assert!(result.is_err());
    }

    #[rstest]
    #[case(3)]
    #[case(1)]
    #[case(5)]
    fn test_zero_creates_zero_vector(#[case] size: usize) -> Result<(), anyhow::Error> {
        // Arrange & Act
        let vector = Vector::zero(size)?;

        // Assert
        assert_eq!(vector.len(), size);
        for i in 0..size {
            assert_eq!(vector[i], 0.0);
        }
        Ok(())
    }

    #[test]
    fn test_zero_returns_error_for_zero_size() {
        // Arrange & Act
        let result = Vector::zero(0);

        // Assert
        assert!(result.is_err());
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], TransposeMethod::Column, 3, 1, vec![(0, 0, 1.0), (1, 0, 2.0), (2, 0, 3.0)])]
    #[case(&[1.0, 2.0, 3.0], TransposeMethod::Row, 1, 3, vec![(0, 0, 1.0), (0, 1, 2.0), (0, 2, 3.0)])]
    #[case(&[42.0], TransposeMethod::Column, 1, 1, vec![(0, 0, 42.0)])]
    #[case(&[42.0], TransposeMethod::Row, 1, 1, vec![(0, 0, 42.0)])]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], TransposeMethod::Column, 5, 1, vec![(0, 0, 1.0), (1, 0, 2.0), (2, 0, 3.0), (3, 0, 4.0), (4, 0, 5.0)])]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], TransposeMethod::Row, 1, 5, vec![(0, 0, 1.0), (0, 1, 2.0), (0, 2, 3.0), (0, 3, 4.0), (0, 4, 5.0)])]
    fn test_to_matrix_converts_with_correct_dimensions_and_values(
        #[case] values: &[f32],
        #[case] method: TransposeMethod,
        #[case] expected_rows: usize,
        #[case] expected_columns: usize,
        #[case] expected_values: Vec<(usize, usize, f32)>,
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector = Vector::new(values)?;

        // Act
        let matrix = vector.to_matrix(method);

        // Assert
        assert_eq!(matrix.size().rows(), expected_rows);
        assert_eq!(matrix.size().columns(), expected_columns);
        for (row, col, expected_value) in expected_values {
            assert_eq!(matrix.get(row, col)?, Some(expected_value));
        }
        Ok(())
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], 2.0, &[2.0, 4.0, 6.0])]
    #[case(&[1.0, 2.0, 3.0], 0.5, &[0.5, 1.0, 1.5])]
    #[case(&[1.0, 2.0, 3.0], 0.0, &[0.0, 0.0, 0.0])]
    #[case(&[5.0], 3.0, &[15.0])]
    fn test_mul_scalar_multiplies_all_elements(
        #[case] values: &[f32],
        #[case] scalar: f32,
        #[case] expected: &[f32],
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector = Vector::new(values)?;

        // Act
        let result = vector * scalar;

        // Assert
        assert_eq!(result.len(), expected.len());
        for (i, &expected_value) in expected.iter().enumerate() {
            assert_eq!(result[i], expected_value);
        }
        Ok(())
    }

    #[rstest]
    #[case(&[2.0, 4.0, 6.0], 2.0, &[1.0, 2.0, 3.0])]
    #[case(&[1.0, 2.0, 3.0], 0.5, &[2.0, 4.0, 6.0])]
    #[case(&[10.0], 5.0, &[2.0])]
    fn test_div_scalar_divides_all_elements(
        #[case] values: &[f32],
        #[case] scalar: f32,
        #[case] expected: &[f32],
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector = Vector::new(values)?;

        // Act
        let result = vector / scalar;

        // Assert
        assert_eq!(result.len(), expected.len());
        for (i, &expected_value) in expected.iter().enumerate() {
            assert_eq!(result[i], expected_value);
        }
        Ok(())
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[5.0, 7.0, 9.0])]
    #[case(&[1.0, 2.0, 3.0], &[0.0, 0.0, 0.0], &[1.0, 2.0, 3.0])]
    #[case(&[5.0], &[10.0], &[15.0])]
    fn test_add_vectors_adds_elements(
        #[case] values1: &[f32],
        #[case] values2: &[f32],
        #[case] expected: &[f32],
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector1 = Vector::new(values1)?;
        let vector2 = Vector::new(values2)?;

        // Act
        let result = (vector1 + vector2)?;

        // Assert
        assert_eq!(result.len(), expected.len());
        for (i, &expected_value) in expected.iter().enumerate() {
            assert_eq!(result[i], expected_value);
        }
        Ok(())
    }

    #[test]
    fn test_add_returns_error_for_different_dimensions() -> Result<(), anyhow::Error> {
        // Arrange
        let vector1 = Vector::new(&[1.0, 2.0, 3.0])?;
        let vector2 = Vector::new(&[4.0, 5.0])?;

        // Act
        let result = vector1 + vector2;

        // Assert
        assert!(result.is_err());
        Ok(())
    }

    #[rstest]
    #[case(&[5.0, 7.0, 9.0], &[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0])]
    #[case(&[1.0, 2.0, 3.0], &[0.0, 0.0, 0.0], &[1.0, 2.0, 3.0])]
    #[case(&[10.0], &[5.0], &[5.0])]
    fn test_sub_vectors_subtracts_elements(
        #[case] values1: &[f32],
        #[case] values2: &[f32],
        #[case] expected: &[f32],
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector1 = Vector::new(values1)?;
        let vector2 = Vector::new(values2)?;

        // Act
        let result = (vector1 - vector2)?;

        // Assert
        assert_eq!(result.len(), expected.len());
        for (i, &expected_value) in expected.iter().enumerate() {
            assert_eq!(result[i], expected_value);
        }
        Ok(())
    }

    #[test]
    fn test_sub_returns_error_for_different_dimensions() -> Result<(), anyhow::Error> {
        // Arrange
        let vector1 = Vector::new(&[1.0, 2.0, 3.0])?;
        let vector2 = Vector::new(&[4.0, 5.0])?;

        // Act
        let result = vector1 - vector2;

        // Assert
        assert!(result.is_err());
        Ok(())
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], 0, 1.0)]
    #[case(&[1.0, 2.0, 3.0], 1, 2.0)]
    #[case(&[1.0, 2.0, 3.0], 2, 3.0)]
    #[case(&[42.0], 0, 42.0)]
    fn test_index_returns_correct_element(
        #[case] values: &[f32],
        #[case] index: usize,
        #[case] expected: f32,
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let vector = Vector::new(values)?;

        // Act
        let result = vector[index];

        // Assert
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], 0, 10.0)]
    #[case(&[1.0, 2.0, 3.0], 1, 20.0)]
    #[case(&[1.0, 2.0, 3.0], 2, 30.0)]
    #[case(&[42.0], 0, 100.0)]
    fn test_index_mut_sets_correct_element(
        #[case] values: &[f32],
        #[case] index: usize,
        #[case] new_value: f32,
    ) -> Result<(), anyhow::Error> {
        // Arrange
        let mut vector = Vector::new(values)?;

        // Act
        vector[index] = new_value;

        // Assert
        assert_eq!(vector[index], new_value);
        Ok(())
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], f32::sqrt(14.0))]
    #[case(&[42.0], 42.0)]
    #[case(&[-42.0], 42.0)]
    fn test_compute_norm(#[case] values: &[f32], #[case] norm: f32) -> Result<(), anyhow::Error> {
        // Arrange
        let vector = Vector::new(values)?;

        // Act
        let ret = vector.norm();

        // Assert
        assert_eq!(ret, norm);
        Ok(())
    }
}
