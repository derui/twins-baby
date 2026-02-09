use std::cmp::min;

use eyre::{Result, eyre};

use crate::matrix::{Matrix, MatrixExtract, size::Size};

/// implement simple matrix.

#[derive(Debug, Clone)]
pub struct SimpleMatrix<M>
where
    M: std::fmt::Debug,
{
    size: Size,
    /// A simple 2D matrix
    values: Vec<Vec<Option<M>>>,
}

impl<M: Clone + std::fmt::Debug> SimpleMatrix<M> {
    /// Create a new empty simple matrix
    ///
    /// # Arguments
    /// * `row` - number of rows
    /// * `column` - number of columns
    ///
    /// # Returns
    /// * A new simple matrix with specified size
    pub fn new(row: usize, column: usize) -> Result<Self, eyre::Error> {
        if row == 0 || column == 0 {
            return Err(eyre::eyre!("Row and column must be greater than zero"));
        }

        let values: Vec<Vec<Option<M>>> = vec![vec![None; column]; row];

        Ok(SimpleMatrix {
            size: Size::new(row, column),
            values,
        })
    }

    /// Clone from other matrix
    ///
    /// # Parameters
    /// * `other`: other matrix
    ///
    /// # Return
    /// * New simple matrix
    pub fn from_matrix(other: &impl Matrix<M>) -> Self {
        let mut mat =
            Self::new(other.size().rows(), other.size().columns()).expect("Must be valid");

        for i in 0..(other.size().rows()) {
            for j in 0..(other.size().columns()) {
                let Some(m) = other.get(i, j).expect("must be valid") else {
                    continue;
                };
                mat.set(i, j, m.clone()).expect("must be valid");
            }
        }

        mat
    }
}

impl<M: Clone + std::fmt::Debug> Matrix<M> for SimpleMatrix<M> {
    fn size(&self) -> Size {
        self.size
    }

    fn get(&self, row: usize, col: usize) -> Result<Option<&M>, eyre::Error> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err(eyre::eyre!("Index out of bounds"));
        }
        Ok(self.values[row][col].as_ref())
    }

    fn set(&mut self, row: usize, col: usize, element: M) -> Result<Option<M>, eyre::Error> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err(eyre::eyre!("Index out of bounds"));
        }
        let old_value = self.values[row][col].clone();
        self.values[row][col] = Some(element);
        Ok(old_value)
    }

    fn diagonal_components(&self) -> Option<Vec<Option<M>>> {
        if self.size.rows() != self.size.columns() {
            return None;
        }

        let pos = min(self.size.rows(), self.size.columns());

        let mut ret: Vec<Option<M>> = vec![None; pos];
        for p in 0..pos {
            ret[p] = self.values[p][p].clone();
        }

        Some(ret)
    }

    fn get_row(&self, row: usize) -> Result<Vec<Option<M>>> {
        if row >= self.size.rows() {
            return Err(eyre!("Can not get row : {}", &row));
        }

        Ok(self.values[row].clone())
    }

    fn set_row(&mut self, row: usize, elements: &[Option<M>]) -> Result<()> {
        if row >= self.size.rows() {
            return Err(eyre!("Can not get row : {}", &row));
        }

        for (i, e) in elements.iter().enumerate() {
            self.values[row][i] = e.clone();
        }

        Ok(())
    }
}

impl<M: Clone + std::fmt::Debug> MatrixExtract<M> for SimpleMatrix<M> {
    fn extract<T>(&self, extract: T) -> impl Matrix<f32>
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_new_creates_matrix_with_correct_size() -> Result<(), eyre::Error> {
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
    fn test_get_returns_none_for_empty_matrix() -> Result<(), eyre::Error> {
        // Arrange
        let matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let result = matrix.get(0, 0)?;

        // Assert
        assert_eq!(result, None);
        Ok(())
    }

    #[test]
    fn test_set_stores_value_and_returns_none_for_empty_cell() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;

        // Act
        let old_value = matrix.set(1, 2, 42)?;

        // Assert
        assert_eq!(old_value, None);
        assert_eq!(matrix.get(1, 2)?, Some(&42));
        Ok(())
    }

    #[test]
    fn test_set_overwrites_existing_value_and_returns_old_value() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(1, 1, 10)?;

        // Act
        let old_value = matrix.set(1, 1, 20)?;

        // Assert
        assert_eq!(old_value, Some(10));
        assert_eq!(matrix.get(1, 1)?.map(|v| *v), Some(20));
        Ok(())
    }

    #[test]
    fn test_extract_creates_f32_matrix_from_complex_type() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(2, 2)?;
        matrix.set(0, 0, 10)?;
        matrix.set(0, 1, 20)?;
        matrix.set(1, 0, 30)?;

        // Act
        let extracted = matrix.extract(|&val| val as f32 * 2.0);

        // Assert
        assert_eq!(extracted.size(), Size::new(2, 2));
        assert_eq!(extracted.get(0, 0)?.map(|v| *v), Some(20.0));
        assert_eq!(extracted.get(0, 1)?.map(|v| *v), Some(40.0));
        assert_eq!(extracted.get(1, 0)?.map(|v| *v), Some(60.0));
        assert_eq!(extracted.get(1, 1)?, None);
        Ok(())
    }

    #[test]
    fn test_diagonal_components_returns_diagonal_elements_for_square_matrix()
    -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(0, 0, 1)?;
        matrix.set(1, 1, 5)?;
        matrix.set(2, 2, 9)?;
        matrix.set(0, 1, 2)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        let diagonal = diagonal.expect("Should return Some for square matrix");
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    #[test]
    fn test_diagonal_components_returns_none_for_rectangular_matrix() -> Result<(), eyre::Error> {
        // Arrange
        let matrix = SimpleMatrix::<i32>::new(4, 2)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        assert!(
            diagonal.is_none(),
            "Should return None for non-square matrix"
        );
        Ok(())
    }

    #[test]
    fn test_diagonal_components_with_some_none_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(0, 0, 1)?;
        // (1, 1) is intentionally left as None
        matrix.set(2, 2, 9)?;

        // Act
        let diagonal = matrix.diagonal_components();

        // Assert
        let diagonal = diagonal.expect("Should return Some for square matrix");
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], None);
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    #[test]
    fn test_extract_preserves_none_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(2, 3)?;
        matrix.set(0, 0, 5)?;
        // (0, 1) and (0, 2) are None
        matrix.set(1, 1, 10)?;

        // Act
        let extracted = matrix.extract(|&val| val as f32);

        // Assert
        assert_eq!(extracted.get(0, 0)?.map(|v| *v), Some(5.0));
        assert_eq!(extracted.get(0, 1)?, None);
        assert_eq!(extracted.get(0, 2)?, None);
        assert_eq!(extracted.get(1, 0)?, None);
        assert_eq!(extracted.get(1, 1)?.map(|v| *v), Some(10.0));
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
    ) -> Result<(), eyre::Error> {
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
    ) -> Result<(), eyre::Error> {
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

    #[test]
    fn test_from_mat_creates_copy_with_all_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 2)?;
        source.set(0, 0, 10)?;
        source.set(0, 1, 20)?;
        source.set(1, 0, 30)?;
        source.set(1, 1, 40)?;
        source.set(2, 0, 50)?;
        source.set(2, 1, 60)?;

        // Act
        let copied = SimpleMatrix::from_matrix(&source);

        // Assert
        assert_eq!(copied.size(), Size::new(3, 2));
        assert_eq!(copied.get(0, 0)?.map(|v| *v), Some(10));
        assert_eq!(copied.get(0, 1)?.map(|v| *v), Some(20));
        assert_eq!(copied.get(1, 0)?.map(|v| *v), Some(30));
        assert_eq!(copied.get(1, 1)?.map(|v| *v), Some(40));
        assert_eq!(copied.get(2, 0)?.map(|v| *v), Some(50));
        assert_eq!(copied.get(2, 1)?.map(|v| *v), Some(60));
        Ok(())
    }

    #[test]
    fn test_from_mat_preserves_none_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 3)?;
        source.set(0, 0, 5)?;
        source.set(1, 2, 15)?;
        // (0, 1), (0, 2), (1, 0), (1, 1) are intentionally left as None

        // Act
        let copied = SimpleMatrix::from_matrix(&source);

        // Assert
        assert_eq!(copied.size(), Size::new(2, 3));
        assert_eq!(copied.get(0, 0)?.map(|v| *v), Some(5));
        assert_eq!(copied.get(0, 1)?, None);
        assert_eq!(copied.get(0, 2)?, None);
        assert_eq!(copied.get(1, 0)?, None);
        assert_eq!(copied.get(1, 1)?, None);
        assert_eq!(copied.get(1, 2)?.map(|v| *v), Some(15));
        Ok(())
    }

    #[test]
    fn test_from_mat_creates_independent_copy() -> Result<(), eyre::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 2)?;
        source.set(0, 0, 100)?;
        source.set(1, 1, 200)?;

        // Act
        let mut copied = SimpleMatrix::from_matrix(&source);
        copied.set(0, 0, 999)?;

        // Assert
        assert_eq!(
            source.get(0, 0)?.map(|v| *v),
            Some(100),
            "Source should be unchanged"
        );
        assert_eq!(
            copied.get(0, 0)?.map(|v| *v),
            Some(999),
            "Copy should be modified"
        );
        Ok(())
    }

    #[test]
    fn test_get_row_returns_correct_row_with_mixed_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 4)?;
        matrix.set(1, 0, 10)?;
        matrix.set(1, 1, 20)?;
        matrix.set(1, 3, 40)?;
        // (1, 2) is intentionally left as None

        // Act
        let row = matrix.get_row(1)?;

        // Assert
        assert_eq!(row.len(), 4);
        assert_eq!(row[0], Some(10));
        assert_eq!(row[1], Some(20));
        assert_eq!(row[2], None);
        assert_eq!(row[3], Some(40));
        Ok(())
    }

    #[test]
    fn test_get_row_returns_error_for_out_of_bounds() -> Result<(), eyre::Error> {
        // Arrange
        let matrix = SimpleMatrix::<i32>::new(3, 4)?;

        // Act
        let result = matrix.get_row(3);

        // Assert
        assert!(result.is_err(), "Expected error for row index at boundary");
        Ok(())
    }

    #[test]
    fn test_set_row_updates_row_with_new_values() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 4)?;
        matrix.set(1, 0, 10)?;
        matrix.set(1, 1, 20)?;
        let new_row = vec![Some(100), Some(200), None, Some(400)];

        // Act
        matrix.set_row(1, &new_row)?;

        // Assert
        assert_eq!(matrix.get(1, 0)?, Some(&100));
        assert_eq!(matrix.get(1, 1)?, Some(&200));
        assert_eq!(matrix.get(1, 2)?, None);
        assert_eq!(matrix.get(1, 3)?, Some(&400));
        Ok(())
    }

    #[test]
    fn test_set_row_preserves_other_rows() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 3)?;
        matrix.set(0, 0, 1)?;
        matrix.set(0, 1, 2)?;
        matrix.set(2, 0, 7)?;
        matrix.set(2, 1, 8)?;
        let new_row = vec![Some(10), Some(20), Some(30)];

        // Act
        matrix.set_row(1, &new_row)?;

        // Assert
        assert_eq!(matrix.get(0, 0)?, Some(&1));
        assert_eq!(matrix.get(0, 1)?, Some(&2));
        assert_eq!(matrix.get(1, 0)?, Some(&10));
        assert_eq!(matrix.get(1, 1)?, Some(&20));
        assert_eq!(matrix.get(1, 2)?, Some(&30));
        assert_eq!(matrix.get(2, 0)?, Some(&7));
        assert_eq!(matrix.get(2, 1)?, Some(&8));
        Ok(())
    }

    #[test]
    fn test_set_row_returns_error_for_out_of_bounds() -> Result<(), eyre::Error> {
        // Arrange
        let mut matrix = SimpleMatrix::<i32>::new(3, 4)?;
        let new_row = vec![Some(1), Some(2), Some(3), Some(4)];

        // Act
        let result = matrix.set_row(3, &new_row);

        // Assert
        assert!(result.is_err(), "Expected error for row index at boundary");
        Ok(())
    }
}
