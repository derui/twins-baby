use std::{cmp::min, error::Error};

use crate::matrix::{FloatingMatrix, Matrix, size::Size};

/// implement sparse matrix

/// Sparse matrix model. This implementation is based on simple CSR model.
#[derive(Debug, Clone)]
struct SparseMatrix<M> {
    size: Size,
    /// Values of row-ordered non-zero value. Zero is mean None in this type.
    values: Vec<M>,
    /// Column index of values
    col_indices: Vec<usize>,
    /// Pointer of index is to start column at the row
    row_ptr: Vec<usize>,
}

impl<M: Clone> SparseMatrix<M> {
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
                    values.push(Some(v));
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
            values: values.iter().cloned().filter_map(|v| v).collect(),
            col_indices,
            row_ptr,
        }
    }
}

impl<M: Clone> Matrix<M> for SparseMatrix<M> {
    fn size(&self) -> super::size::Size {
        self.size
    }

    fn get(&self, row: usize, col: usize) -> Result<Option<M>, Box<dyn Error>> {
        if row >= self.size.rows() || col >= self.size.columns() {
            return Err("Index out of bound".into());
        }

        let start_values_index_of_row = self.row_ptr[row];
        let value_count_of_row = self.row_ptr[row + 1] - start_values_index_of_row;

        if value_count_of_row == 0 {
            return Ok(None);
        }
        let slice_col_indices = &self.col_indices
            [start_values_index_of_row..(start_values_index_of_row + value_count_of_row)];

        if let Some(v) = slice_col_indices.iter().position(|v| *v == col) {
            Ok(Some(self.values[start_values_index_of_row + v].clone()))
        } else {
            Ok(None)
        }
    }

    // Sparse matrix does not support set for now.
    fn set(
        &mut self,
        _row: usize,
        _col: usize,
        _element: M,
    ) -> Result<Option<M>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn extract<T>(&self, extract: T) -> impl Matrix<f32> + FloatingMatrix
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

    fn diagonal_components(&self) -> Vec<Option<M>> {
        let len = min(self.size.rows(), self.size.columns());
        let mut vec: Vec<Option<M>> = vec![None; len];

        for i in 0..len {
            if let Ok(v) = self.get(i, i).into() {
                vec[i] = v;
            }
        }

        vec
    }
}

impl FloatingMatrix for SparseMatrix<f32> {
    fn determinant(&self) -> Option<f32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::simple::SimpleMatrix;
    use pretty_assertions::assert_eq;

    /// Test that from_matrix creates a sparse matrix with correct size
    #[test]
    fn test_from_matrix_preserves_size() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 4);
        source.set(0, 0, 1).unwrap();
        source.set(1, 2, 5).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.size(), Size::new(3, 4));
    }

    /// Test that from_matrix correctly converts a matrix with sparse values
    #[test]
    fn test_from_matrix_converts_sparse_values_correctly() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3);
        source.set(0, 0, 1).unwrap();
        source.set(0, 2, 3).unwrap();
        source.set(1, 1, 5).unwrap();
        source.set(2, 0, 7).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0).unwrap(), Some(1));
        assert_eq!(sparse.get(0, 2).unwrap(), Some(3));
        assert_eq!(sparse.get(1, 1).unwrap(), Some(5));
        assert_eq!(sparse.get(2, 0).unwrap(), Some(7));
        assert_eq!(sparse.get(0, 1).unwrap(), None);
        assert_eq!(sparse.get(1, 0).unwrap(), None);
        assert_eq!(sparse.get(2, 2).unwrap(), None);
    }

    /// Test that from_matrix handles empty matrix (all None values)
    #[test]
    fn test_from_matrix_handles_empty_matrix() {
        // Arrange
        let source = SimpleMatrix::<i32>::new(2, 3);

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0).unwrap(), None);
        assert_eq!(sparse.get(1, 2).unwrap(), None);
        assert_eq!(sparse.size(), Size::new(2, 3));
    }

    /// Test that from_matrix handles dense matrix (all values present)
    #[test]
    fn test_from_matrix_handles_dense_matrix() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 2);
        source.set(0, 0, 1).unwrap();
        source.set(0, 1, 2).unwrap();
        source.set(1, 0, 3).unwrap();
        source.set(1, 1, 4).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0).unwrap(), Some(1));
        assert_eq!(sparse.get(0, 1).unwrap(), Some(2));
        assert_eq!(sparse.get(1, 0).unwrap(), Some(3));
        assert_eq!(sparse.get(1, 1).unwrap(), Some(4));
    }

    /// Test that get returns error for out of bounds row
    #[test]
    fn test_get_returns_error_for_out_of_bounds_row() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 3);
        source.set(0, 0, 1).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(2, 0);

        // Assert
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "Index out of bound");
    }

    /// Test that get returns error for out of bounds column
    #[test]
    fn test_get_returns_error_for_out_of_bounds_column() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 3);
        source.set(0, 0, 1).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(0, 3);

        // Assert
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "Index out of bound");
    }

    /// Test that get retrieves None for unset values within bounds
    #[test]
    fn test_get_returns_none_for_unset_values() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3);
        source.set(0, 0, 1).unwrap();
        source.set(2, 2, 9).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let result = sparse.get(1, 1);

        // Assert
        assert_eq!(result.unwrap(), None);
    }

    /// Test that extract correctly transforms values using the provided function
    #[test]
    fn test_extract_transforms_values_correctly() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 2);
        source.set(0, 0, 10).unwrap();
        source.set(1, 1, 20).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let extracted = sparse.extract(|&v| v as f32 * 2.0);

        // Assert
        assert_eq!(extracted.get(0, 0).unwrap(), Some(20.0));
        assert_eq!(extracted.get(1, 1).unwrap(), Some(40.0));
        assert_eq!(extracted.get(0, 1).unwrap(), None);
    }

    /// Test that extract preserves the matrix structure
    #[test]
    fn test_extract_preserves_matrix_structure() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 4);
        source.set(0, 1, 5).unwrap();
        source.set(2, 3, 15).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let extracted = sparse.extract(|&v| v as f32);

        // Assert
        assert_eq!(extracted.size(), Size::new(3, 4));
        assert_eq!(extracted.get(0, 1).unwrap(), Some(5.0));
        assert_eq!(extracted.get(2, 3).unwrap(), Some(15.0));
        assert_eq!(extracted.get(1, 1).unwrap(), None);
    }

    /// Test that diagonal_components returns correct values for square matrix
    #[test]
    fn test_diagonal_components_returns_correct_values_for_square_matrix() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3);
        source.set(0, 0, 1).unwrap();
        source.set(1, 1, 5).unwrap();
        source.set(2, 2, 9).unwrap();
        source.set(0, 1, 2).unwrap(); // Non-diagonal
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
        assert_eq!(diagonal[2], Some(9));
    }

    /// Test that diagonal_components handles missing diagonal values
    #[test]
    fn test_diagonal_components_handles_missing_diagonal_values() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(3, 3);
        source.set(0, 0, 1).unwrap();
        source.set(2, 2, 9).unwrap();
        // 1,1 is intentionally not set
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], None);
        assert_eq!(diagonal[2], Some(9));
    }

    /// Test that diagonal_components works for non-square matrix (more rows)
    #[test]
    fn test_diagonal_components_for_tall_matrix() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(4, 2);
        source.set(0, 0, 1).unwrap();
        source.set(1, 1, 5).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 2); // min(4, 2) = 2
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
    }

    /// Test that diagonal_components works for non-square matrix (more columns)
    #[test]
    fn test_diagonal_components_for_wide_matrix() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(2, 4);
        source.set(0, 0, 1).unwrap();
        source.set(1, 1, 5).unwrap();
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let diagonal = sparse.diagonal_components();

        // Assert
        assert_eq!(diagonal.len(), 2); // min(2, 4) = 2
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
    }

    /// Test that size returns correct dimensions
    #[test]
    fn test_size_returns_correct_dimensions() {
        // Arrange
        let source = SimpleMatrix::<i32>::new(5, 7);
        let sparse = SparseMatrix::from_matrix(&source);

        // Act
        let size = sparse.size();

        // Assert
        assert_eq!(size.rows(), 5);
        assert_eq!(size.columns(), 7);
    }

    /// Test that sparse matrix works with f32 values
    #[test]
    fn test_sparse_matrix_with_f32_values() {
        // Arrange
        let mut source = SimpleMatrix::<f32>::new(2, 2);
        source.set(0, 0, 1.5).unwrap();
        source.set(1, 1, 2.7).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0).unwrap(), Some(1.5));
        assert_eq!(sparse.get(1, 1).unwrap(), Some(2.7));
        assert_eq!(sparse.get(0, 1).unwrap(), None);
    }

    /// Test that from_matrix handles single element matrix
    #[test]
    fn test_from_matrix_handles_single_element() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(1, 1);
        source.set(0, 0, 42).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.size(), Size::new(1, 1));
        assert_eq!(sparse.get(0, 0).unwrap(), Some(42));
    }

    /// Test that from_matrix handles single row matrix
    #[test]
    fn test_from_matrix_handles_single_row() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(1, 5);
        source.set(0, 0, 1).unwrap();
        source.set(0, 2, 3).unwrap();
        source.set(0, 4, 5).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        dbg!(&sparse);
        assert_eq!(sparse.get(0, 0).unwrap(), Some(1));
        assert_eq!(sparse.get(0, 2).unwrap(), Some(3));
        assert_eq!(sparse.get(0, 4).unwrap(), Some(5));
        assert_eq!(sparse.get(0, 1).unwrap(), None);
        assert_eq!(sparse.get(0, 3).unwrap(), None);
    }

    /// Test that from_matrix handles single column matrix
    #[test]
    fn test_from_matrix_handles_single_column() {
        // Arrange
        let mut source = SimpleMatrix::<i32>::new(5, 1);
        source.set(0, 0, 1).unwrap();
        source.set(2, 0, 3).unwrap();
        source.set(4, 0, 5).unwrap();

        // Act
        let sparse = SparseMatrix::from_matrix(&source);

        // Assert
        assert_eq!(sparse.get(0, 0).unwrap(), Some(1));
        assert_eq!(sparse.get(2, 0).unwrap(), Some(3));
        assert_eq!(sparse.get(4, 0).unwrap(), Some(5));
        assert_eq!(sparse.get(1, 0).unwrap(), None);
        assert_eq!(sparse.get(3, 0).unwrap(), None);
    }
}
