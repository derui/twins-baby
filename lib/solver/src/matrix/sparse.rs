use std::cmp::min;

use anyhow::Result;

use crate::matrix::{Matrix, MatrixExtract, simple::SimpleMatrix, size::Size};

/// implement sparse matrix

/// Sparse matrix model. This implementation is based on simple CSR model.
#[derive(Debug, Clone)]
pub struct SparseMatrix<M: std::fmt::Debug> {
    size: Size,
    /// Values of row-ordered non-zero value. Zero is mean None in this type.
    values: Vec<M>,
    /// Column index of values
    col_indices: Vec<usize>,
    /// Pointer of index is to start column at the row
    row_ptr: Vec<usize>,
}

impl<M: Clone + std::fmt::Debug> SparseMatrix<M> {
    /// Create a empty Sparse matrix
    pub fn empty(size: Size) -> Result<Self, anyhow::Error> {
        if size.columns() <= 0 || size.rows() <= 0 {
            return Err(anyhow::anyhow!("can not create 0-sized matrix"));
        }

        let mat = SimpleMatrix::new(size.rows(), size.columns())?;

        Ok(Self::from_matrix(&mat))
    }

    /// Create a sparse matrix from other matrix
    pub fn from_matrix(mat: &impl Matrix<M>) -> Self {
        let size = mat.size();
        let mut values: Vec<Option<M>> = vec![];
        let mut col_indices: Vec<usize> = vec![];
        let mut row_ptr: Vec<usize> = vec![];

        for r in 0..(size.rows()) {
            let mut ptr_recorded = false;

            for c in 0..(size.columns()) {
                if let Ok(Some(v)) = mat.get(r, c) {
                    if !ptr_recorded {
                        ptr_recorded = true;
                        row_ptr.push(values.len())
                    }
                    values.push(Some(v.clone()));
                    col_indices.push(c);
                }
            }

            if !ptr_recorded {
                row_ptr.push(values.len());
            }
        }

        row_ptr.push(values.iter().filter_map(Option::Some).count());

        SparseMatrix {
            size,
            values: values.iter().flatten().cloned().collect(),
            col_indices,
            row_ptr,
        }
    }
}

impl<M: Clone + std::fmt::Debug> Matrix<M> for SparseMatrix<M> {
    fn size(&self) -> super::size::Size {
        self.size
    }

    fn get(&self, row: usize, col: usize) -> Result<Option<&M>, anyhow::Error> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err(anyhow::anyhow!("Index out of bound"));
        }

        let start_values_index_of_row = self.row_ptr[row];
        let value_count_of_row = self.row_ptr[row + 1] - start_values_index_of_row;

        if value_count_of_row == 0 {
            return Ok(None);
        }
        let slice_col_indices = &self.col_indices
            [start_values_index_of_row..(start_values_index_of_row + value_count_of_row)];

        if let Some(v) = slice_col_indices.iter().position(|v| *v == col) {
            Ok(Some(&self.values[start_values_index_of_row + v]))
        } else {
            Ok(None)
        }
    }

    // Sparse matrix does not support set for now.
    fn set(&mut self, _row: usize, _col: usize, _element: M) -> Result<Option<M>, anyhow::Error> {
        todo!()
    }

    fn diagonal_components(&self) -> Option<Vec<Option<M>>> {
        if self.size.rows() != self.size.columns() {
            return None;
        }

        let len = min(self.size.rows(), self.size.columns());
        let mut vec: Vec<Option<M>> = vec![None; len];

        for i in 0..len {
            if let Ok(v) = self.get(i, i) {
                vec[i] = v.cloned();
            }
        }

        Some(vec)
    }
}

impl<M: Clone + std::fmt::Debug> MatrixExtract<M> for SparseMatrix<M> {
    fn extract<T>(&self, extract: T) -> impl Matrix<f32>
    where
        T: Fn(&M) -> f32,
    {
        SparseMatrix::<f32> {
            size: self.size,
            values: self.values.iter().map(extract).collect(),
            col_indices: self.col_indices.clone(),
            row_ptr: self.row_ptr.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::simple::SimpleMatrix;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    /// Test that from_matrix creates a sparse matrix with correct size
    #[test]
    fn test_from_matrix_preserves_size() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 4)?;
        source.set(0, 0, 1)?;
        source.set(1, 2, 5)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.size(), Size::new(3, 4));
        Ok(())
    }

    /// Test that from_matrix correctly converts a matrix with sparse values
    #[test]
    fn test_from_matrix_converts_sparse_values_correctly() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3)?;
        source.set(0, 0, 1)?;
        source.set(0, 2, 3)?;
        source.set(1, 1, 5)?;
        source.set(2, 0, 7)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(1));
        assert_eq!(sparse.get(0, 2)?.map(|v| *v), Some(3));
        assert_eq!(sparse.get(1, 1)?.map(|v| *v), Some(5));
        assert_eq!(sparse.get(2, 0)?.map(|v| *v), Some(7));
        assert_eq!(sparse.get(0, 1)?, None);
        assert_eq!(sparse.get(1, 0)?, None);
        assert_eq!(sparse.get(2, 2)?, None);
        Ok(())
    }

    /// Test that from_matrix handles empty matrix (all None values)
    #[test]
    fn test_from_matrix_handles_empty_matrix() -> Result<(), anyhow::Error> {
        // Arrange
        let source = SimpleMatrix::<i32>::new(2, 3)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0)?, None);
        assert_eq!(sparse.get(1, 2)?, None);
        assert_eq!(sparse.size(), Size::new(2, 3));
        Ok(())
    }

    /// Test that from_matrix handles dense matrix (all values present)
    #[test]
    fn test_from_matrix_handles_dense_matrix() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 2)?;
        source.set(0, 0, 1)?;
        source.set(0, 1, 2)?;
        source.set(1, 0, 3)?;
        source.set(1, 1, 4)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(1));
        assert_eq!(sparse.get(0, 1)?.map(|v| *v), Some(2));
        assert_eq!(sparse.get(1, 0)?.map(|v| *v), Some(3));
        assert_eq!(sparse.get(1, 1)?.map(|v| *v), Some(4));
        Ok(())
    }

    /// Test that get returns error for out of bounds row
    #[test]
    fn test_get_returns_error_for_out_of_bounds_row() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 3)?;
        source.set(0, 0, 1)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(2, 0);

        // Assert
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "Index out of bound");
        Ok(())
    }

    /// Test that get returns error for out of bounds column
    #[test]
    fn test_get_returns_error_for_out_of_bounds_column() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 3)?;
        source.set(0, 0, 1)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(0, 3);

        // Assert
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "Index out of bound");
        Ok(())
    }

    /// Test that get retrieves None for unset values within bounds
    #[test]
    fn test_get_returns_none_for_unset_values() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3)?;
        source.set(0, 0, 1)?;
        source.set(2, 2, 9)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(1, 1)?;

        // Assert
        assert_eq!(result, None);
        Ok(())
    }

    /// Test that extract correctly transforms values using the provided function
    #[test]
    fn test_extract_transforms_values_correctly() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 2)?;
        source.set(0, 0, 10)?;
        source.set(1, 1, 20)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let extracted = sparse.extract(|&v| v as f32 * 2.0);

        // Assert
        assert_eq!(extracted.get(0, 0)?.map(|v| *v), Some(20.0));
        assert_eq!(extracted.get(1, 1)?.map(|v| *v), Some(40.0));
        assert_eq!(extracted.get(0, 1)?, None);
        Ok(())
    }

    /// Test that extract preserves the matrix structure
    #[test]
    fn test_extract_preserves_matrix_structure() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 4)?;
        source.set(0, 1, 5)?;
        source.set(2, 3, 15)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let extracted = sparse.extract(|&v| v as f32);

        // Assert
        assert_eq!(extracted.size(), Size::new(3, 4));
        assert_eq!(extracted.get(0, 1)?.map(|v| *v), Some(5.0));
        assert_eq!(extracted.get(2, 3)?.map(|v| *v), Some(15.0));
        assert_eq!(extracted.get(1, 1)?, None);
        Ok(())
    }

    /// Test that diagonal_components returns correct values for square matrix
    #[test]
    fn test_diagonal_components_returns_correct_values_for_square_matrix()
    -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3)?;
        source.set(0, 0, 1)?;
        source.set(1, 1, 5)?;
        source.set(2, 2, 9)?;
        source.set(0, 1, 2)?; // Non-diagonal
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        let diagonal = diagonal.expect("Should return Some for square matrix");
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    /// Test that diagonal_components handles missing diagonal values
    #[test]
    fn test_diagonal_components_handles_missing_diagonal_values() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3)?;
        source.set(0, 0, 1)?;
        source.set(2, 2, 9)?;
        // 1,1 is intentionally not set
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        let diagonal = diagonal.expect("Should return Some for square matrix");
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], None);
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    /// Test that diagonal_components returns None for non-square matrix (more rows)
    #[test]
    fn test_diagonal_components_returns_none_for_tall_matrix() -> Result<(), anyhow::Error> {
        // Arrange
        let source = SimpleMatrix::<i32>::new(4, 2)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert!(
            diagonal.is_none(),
            "Should return None for non-square matrix"
        );
        Ok(())
    }

    /// Test that diagonal_components returns None for non-square matrix (more columns)
    #[test]
    fn test_diagonal_components_returns_none_for_wide_matrix() -> Result<(), anyhow::Error> {
        // Arrange
        let source = SimpleMatrix::<i32>::new(2, 4)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert!(
            diagonal.is_none(),
            "Should return None for non-square matrix"
        );
        Ok(())
    }

    /// Test that size returns correct dimensions
    #[test]
    fn test_size_returns_correct_dimensions() -> Result<(), anyhow::Error> {
        // Arrange
        let source = SimpleMatrix::<i32>::new(5, 7)?;
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let size = sparse.size();

        // Assert
        assert_eq!(size.rows(), 5);
        assert_eq!(size.columns(), 7);
        Ok(())
    }

    /// Test that sparse matrix works with f32 values
    #[test]
    fn test_sparse_matrix_with_f32_values() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<f32>::new(2, 2)?;
        source.set(0, 0, 1.5)?;
        source.set(1, 1, 2.7)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(1.5));
        assert_eq!(sparse.get(1, 1)?.map(|v| *v), Some(2.7));
        assert_eq!(sparse.get(0, 1)?, None);
        Ok(())
    }

    /// Test that from_matrix handles single element matrix
    #[test]
    fn test_from_matrix_handles_single_element() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(1, 1)?;
        source.set(0, 0, 42)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.size(), Size::new(1, 1));
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(42));
        Ok(())
    }

    /// Test that from_matrix handles single row matrix
    #[test]
    fn test_from_matrix_handles_single_row() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(1, 5)?;
        source.set(0, 0, 1)?;
        source.set(0, 2, 3)?;
        source.set(0, 4, 5)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        dbg!(&sparse);
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(1));
        assert_eq!(sparse.get(0, 2)?.map(|v| *v), Some(3));
        assert_eq!(sparse.get(0, 4)?.map(|v| *v), Some(5));
        assert_eq!(sparse.get(0, 1)?, None);
        assert_eq!(sparse.get(0, 3)?, None);
        Ok(())
    }

    /// Test that from_matrix handles single column matrix
    #[test]
    fn test_from_matrix_handles_single_column() -> Result<(), anyhow::Error> {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(5, 1)?;
        source.set(0, 0, 1)?;
        source.set(2, 0, 3)?;
        source.set(4, 0, 5)?;

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0)?.map(|v| *v), Some(1));
        assert_eq!(sparse.get(2, 0)?.map(|v| *v), Some(3));
        assert_eq!(sparse.get(4, 0)?.map(|v| *v), Some(5));
        assert_eq!(sparse.get(1, 0)?, None);
        assert_eq!(sparse.get(3, 0)?, None);
        Ok(())
    }

    /// Test that empty creates a sparse matrix with correct size and all None values
    #[test]
    fn test_empty_creates_matrix_with_correct_size_and_all_none_values() -> Result<(), anyhow::Error>
    {
        // Arrange
        let size = Size::new(3, 4);

        // Act
        let sparse = SparseMatrix::<i32>::empty(size)?;

        // Assert
        assert_eq!(sparse.size(), Size::new(3, 4));
        assert_eq!(sparse.get(0, 0)?, None);
        assert_eq!(sparse.get(1, 2)?, None);
        assert_eq!(sparse.get(2, 3)?, None);
        Ok(())
    }

    /// Test that empty returns error for invalid dimensions
    #[rstest]
    #[case(0, 3, "zero rows")]
    #[case(3, 0, "zero columns")]
    #[case(0, 0, "both zero")]
    fn test_empty_returns_error_for_zero_dimensions(
        #[case] rows: usize,
        #[case] cols: usize,
        #[case] description: &str,
    ) {
        // Arrange
        let size = Size::new(rows, cols);

        // Act
        let result = SparseMatrix::<i32>::empty(size);

        // Assert
        assert!(result.is_err(), "Should return error for {}", description);
        assert_eq!(
            result.unwrap_err().to_string(),
            "can not create 0-sized matrix",
            "Error message should be correct for {}",
            description
        );
    }
}
