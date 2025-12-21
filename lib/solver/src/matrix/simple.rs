use std::{cmp::min, error::Error};

use crate::matrix::{FloatingMatrix, Matrix, size::Size};

/// implement simple matrix.

pub struct SimpleMatrix<M> {
    size: Size,
    /// A simple 2D matrix
    values: Vec<Vec<Option<M>>>,
}

impl<M: Clone> SimpleMatrix<M> {
    /// Create a new empty simple matrix
    ///
    /// # Arguments
    /// * `row` - number of rows
    /// * `column` - number of columns
    ///
    /// # Returns
    /// * A new simple matrix with specified size
    pub fn new(row: usize, column: usize) -> Result<Self, Box<dyn Error>> {
        if row == 0 || column == 0 {
            return Err("Row and column must be greater than zero".into());
        }

        let values: Vec<Vec<Option<M>>> = vec![vec![None; column]; row];

        Ok(SimpleMatrix {
            size: Size::new(row, column),
            values,
        })
    }
}

impl<M: Clone> Matrix<M> for SimpleMatrix<M> {
    fn size(&self) -> Size {
        self.size
    }

    fn get(&self, row: usize, col: usize) -> Result<Option<M>, Box<dyn Error>> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err("Index out of bounds".into());
        }
        Ok(self.values[row][col].clone())
    }

    fn set(
        &mut self,
        row: usize,
        col: usize,
        element: M,
    ) -> Result<Option<M>, Box<dyn std::error::Error>> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err("Index out of bounds".into());
        }
        let old_value = self.values[row][col].clone();
        self.values[row][col] = Some(element);
        Ok(old_value)
    }

    fn extract<T>(&self, extract: T) -> impl Matrix<f32> + FloatingMatrix
    where
        T: Fn(&M) -> f32,
    {
        let mut new_matrix = SimpleMatrix::<f32>::new(self.size.rows(), self.size.columns())
            .expect("Must be valid matrix in this");
        for r in 0..self.size.rows() {
            for c in 0..self.size.columns() {
                if let Some(ref value) = self.values[r][c] {
                    let extracted_value = extract(value);
                    new_matrix.set(r, c, extracted_value).unwrap();
                }
            }
        }

        new_matrix
    }

    fn diagonal_components(&self) -> Vec<Option<M>> {
        let pos = min(self.size.rows(), self.size.columns());

        let mut ret: Vec<Option<M>> = vec![None; pos];
        for p in 0..pos {
            ret[p] = self.values[p][p].clone();
        }

        ret
    }
}

impl FloatingMatrix for SimpleMatrix<f32> {
    fn determinant(&self) -> Option<f32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_new_creates_matrix_with_correct_size() -> Result<(), Box<dyn Error>> {
        // Arrange
        let rows = 3;
        let cols = 4;

        // Act
        let matrix = SimpleMatrix::<i32>::new(rows, cols)?;

        // Assert
        assert_eq!(matrix.size(), Size::new(rows, cols));
        assert_eq!(matrix.size().rows(), rows);
        assert_eq!(matrix.size().columns(), cols);
        Ok(())
    }

    #[rstest]
    #[case(0, 5, "zero row")]
    #[case(5, 0, "zero column")]
    #[case(0, 0, "both zero")]
    fn test_new_returns_error_for_invalid_dimensions(
        #[case] row: usize,
        #[case] col: usize,
        #[case] description: &str,
    ) {
        // Arrange & Act
        let result = SimpleMatrix::<i32>::new(row, col);

        // Assert
        assert!(
            result.is_err(),
            "Expected error for {}, but got Ok",
            description
        );
    }

    #[test]
    fn test_get_returns_none_for_empty_matrix() -> Result<(), Box<dyn Error>> {
        // Arrange
        let matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let result = matrix.get(0, 0)?;

        // Assert
        assert_eq!(result, None);
        Ok(())
    }

    #[test]
    fn test_set_stores_value_and_returns_none_for_empty_cell() -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let old_value = matrix.set(1, 2, 42)?;

        // Assert
        assert_eq!(old_value, None);
        assert_eq!(matrix.get(1, 2)?, Some(42));
        Ok(())
    }

    #[test]
    fn test_set_overwrites_existing_value_and_returns_old_value() -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(1, 1, 10)?;

        // Act
        let old_value = matrix.set(1, 1, 20)?;

        // Assert
        assert_eq!(old_value, Some(10));
        assert_eq!(matrix.get(1, 1)?, Some(20));
        Ok(())
    }

    #[test]
    fn test_extract_creates_f32_matrix_from_complex_type() -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(2, 2)?;
        matrix.set(0, 0, 10)?;
        matrix.set(0, 1, 20)?;
        matrix.set(1, 0, 30)?;

        // Act
        let extracted = matrix.extract(|&val| val as f32 * 2.0);

        // Assert
        assert_eq!(extracted.size(), Size::new(2, 2));
        assert_eq!(extracted.get(0, 0)?, Some(20.0));
        assert_eq!(extracted.get(0, 1)?, Some(40.0));
        assert_eq!(extracted.get(1, 0)?, Some(60.0));
        assert_eq!(extracted.get(1, 1)?, None);
        Ok(())
    }

    #[test]
    fn test_diagonal_components_returns_diagonal_elements_for_square_matrix()
    -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(0, 0, 1)?;
        matrix.set(1, 1, 5)?;
        matrix.set(2, 2, 9)?;
        matrix.set(0, 1, 2)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    #[test]
    fn test_diagonal_components_returns_min_size_for_rectangular_matrix()
    -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(4, 2)?;
        matrix.set(0, 0, 10)?;
        matrix.set(1, 1, 20)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 2);
        assert_eq!(diagonal[0], Some(10));
        assert_eq!(diagonal[1], Some(20));
        Ok(())
    }

    #[test]
    fn test_diagonal_components_with_some_none_values() -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(0, 0, 1)?;
        // (1, 1) is intentionally left as None
        matrix.set(2, 2, 9)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], None);
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    #[test]
    fn test_extract_preserves_none_values() -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(2, 3)?;
        matrix.set(0, 0, 5)?;
        // (0, 1) and (0, 2) are None
        matrix.set(1, 1, 10)?;

        // Act
        let extracted = matrix.extract(|&val| val as f32);

        // Assert
        assert_eq!(extracted.get(0, 0)?, Some(5.0));
        assert_eq!(extracted.get(0, 1)?, None);
        assert_eq!(extracted.get(0, 2)?, None);
        assert_eq!(extracted.get(1, 0)?, None);
        assert_eq!(extracted.get(1, 1)?, Some(10.0));
        assert_eq!(extracted.get(1, 2)?, None);
        Ok(())
    }

    #[rstest]
    #[case(3, 0, "row at boundary")]
    #[case(4, 0, "row beyond boundary")]
    #[case(0, 3, "column at boundary")]
    #[case(0, 4, "column beyond boundary")]
    #[case(3, 3, "both at boundary")]
    #[case(5, 5, "both beyond boundary")]
    fn test_get_returns_error_for_invalid_indices(
        #[case] row: usize,
        #[case] col: usize,
        #[case] description: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let result = matrix.get(row, col);

        // Assert
        assert!(
            result.is_err(),
            "Expected error for {}, but got Ok",
            description
        );
        Ok(())
    }

    #[rstest]
    #[case(3, 0, "row at boundary")]
    #[case(4, 0, "row beyond boundary")]
    #[case(0, 3, "column at boundary")]
    #[case(0, 4, "column beyond boundary")]
    #[case(3, 3, "both at boundary")]
    #[case(5, 5, "both beyond boundary")]
    fn test_set_returns_error_for_invalid_indices(
        #[case] row: usize,
        #[case] col: usize,
        #[case] description: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let result = matrix.set(row, col, 42);

        // Assert
        assert!(
            result.is_err(),
            "Expected error for {}, but got Ok",
            description
        );
        Ok(())
    }
}
