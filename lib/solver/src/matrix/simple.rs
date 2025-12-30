use std::{cmp::min, error::Error};

use crate::matrix::{FloatingMatrix, Matrix, size::Size};

/// implement simple matrix.

#[derive(Debug, Clone)]
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
}

struct LUSplit {
    l_matrix: SimpleMatrix<f32>,
    u_mattix: SimpleMatrix<f32>,
}

impl SimpleMatrix<f32> {
    /// LU split algorithm implementation for the matrix
    ///
    /// # Return
    /// * Ok with splited LU matrix
    fn lu_split(&self) -> Result<LUSplit, Box<dyn Error>> {
        if self.size.rows() != self.size.columns() {
            return Err("can not make the LU split without exponent matrix".into());
        }

        let mut l = SimpleMatrix::<f32>::new(self.size.rows(), self.size.columns())?;
        let mut u = SimpleMatrix::<f32>::new(self.size.rows(), self.size.columns())?;
        let n = self.size.min();

        // initialize L/U matrix
        for i in 0..(n) {
            l.set(i, i, 1.0)?;
        }

        for i in 0..(n) {
            for j in i..(n) {
                let mut sum_u = 0.0;

                for k in 0..i {
                    sum_u += match (l.get(i, k), u.get(k, j)) {
                        (Ok(Some(l)), Ok(Some(u))) => l * u,
                        (_, _) => 0.0,
                    };
                }

                let u_ij = self.get(i, j)?.unwrap_or(0.) - sum_u;
                let _ = u.set(i, j, u_ij);
            }

            for j in (i + 1)..(n) {
                let mut sum_l = 0.0;

                for k in 0..i {
                    sum_l += match (l.get(j, k), u.get(k, i)) {
                        (Ok(Some(l)), Ok(Some(u))) => l * u,
                        (_, _) => 0.0,
                    };
                }

                let l_ji = self.get(j, i)?.unwrap_or(0.) - sum_l;
                let _ = l.set(j, i, l_ji / u.get(i, i)?.unwrap_or(1.0));
            }
        }

        Ok(LUSplit {
            l_matrix: l,
            u_mattix: u,
        })
    }
}

impl FloatingMatrix for SimpleMatrix<f32> {
    fn determinant(&self) -> Option<f32> {
        if let Ok(splited) = self.lu_split() {
            let u = splited.u_mattix;
            dbg!(&u);

            let mut sum = 1.0;
            for i in 0..(self.size.min()) {
                sum *= u.get(i, i).unwrap_or(None).unwrap_or(0.0);
            }

            Some(sum)
        } else {
            None
        }
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
        let diagonal = diagonal.expect("Should return Some for square matrix");
        assert_eq!(diagonal.len(), 3);
        assert_eq!(diagonal[0], Some(1));
        assert_eq!(diagonal[1], Some(5));
        assert_eq!(diagonal[2], Some(9));
        Ok(())
    }

    #[test]
    fn test_diagonal_components_returns_none_for_rectangular_matrix() -> Result<(), Box<dyn Error>>
    {
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
    fn test_diagonal_components_with_some_none_values() -> Result<(), Box<dyn Error>> {
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

    #[test]
    fn test_determinant_2x2_matrix() -> Result<(), Box<dyn Error>> {
        // Arrange
        // Matrix: | 1  2 |
        //         | 3  4 |
        // det = 1*4 - 2*3 = -2
        let mut matrix = SimpleMatrix::<f32>::new(2, 2)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(0, 1, 2.0)?;
        matrix.set(1, 0, 3.0)?;
        matrix.set(1, 1, 4.0)?;

        // Act
        let det = matrix.determinant();

        // Assert
        assert_eq!(det, Some(-2.0));
        Ok(())
    }

    #[test]
    fn test_determinant_3x3_matrix() -> Result<(), Box<dyn Error>> {
        // Arrange
        // Matrix: | 1  2  3 |
        //         | 4  5  6 |
        //         | 7  8  9 |
        // det = 1*(5*9-6*8) - 2*(4*9-6*7) + 3*(4*8-5*7)
        //     = 1*(45-48) - 2*(36-42) + 3*(32-35)
        //     = 1*(-3) - 2*(-6) + 3*(-3)
        //     = -3 + 12 - 9 = 0
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(0, 1, 2.0)?;
        matrix.set(0, 2, 3.0)?;
        matrix.set(1, 0, 4.0)?;
        matrix.set(1, 1, 5.0)?;
        matrix.set(1, 2, 6.0)?;
        matrix.set(2, 0, 7.0)?;
        matrix.set(2, 1, 8.0)?;
        matrix.set(2, 2, 9.0)?;

        // Act
        let det = matrix.determinant();

        // Assert
        assert_eq!(det, Some(0.0));
        Ok(())
    }

    #[test]
    fn test_determinant_identity_matrix() -> Result<(), Box<dyn Error>> {
        // Arrange
        // Identity matrix has determinant = 1
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(1, 1, 1.0)?;
        matrix.set(2, 2, 1.0)?;

        // Act
        let det = matrix.determinant();

        // Assert
        assert_eq!(det, Some(1.0));
        Ok(())
    }

    #[rstest]
    #[case(2, 3, "more columns than rows")]
    #[case(3, 2, "more rows than columns")]
    #[case(1, 4, "single row")]
    #[case(4, 1, "single column")]
    fn test_determinant_returns_none_for_non_square_matrix(
        #[case] rows: usize,
        #[case] cols: usize,
        #[case] description: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let matrix = SimpleMatrix::<f32>::new(rows, cols)?;

        // Act
        let det = matrix.determinant();

        // Assert
        assert!(
            det.is_none(),
            "Expected None for {} ({}x{}), but got {:?}",
            description,
            rows,
            cols,
            det
        );
        Ok(())
    }
}
