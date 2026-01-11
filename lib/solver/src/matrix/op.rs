/// Operation definition for matrix module.
use std::ops::{Add, Mul};

use anyhow::Result;

use crate::{
    matrix::{Matrix, simple::SimpleMatrix},
    vector::Vector,
};

/// Multiply operation between matrix
pub fn mul<M: std::fmt::Debug, T: Matrix<M>, U: Matrix<M>>(
    lhs: &T,
    rhs: &U,
) -> Result<impl Matrix<M> + use<M, T, U>, anyhow::Error>
where
    M: Add<Output = M> + Mul<Output = M> + Default + Copy,
{
    if lhs.size().columns() != rhs.size().rows() {
        return Err(anyhow::anyhow!(
            "Can not multiply different number of columns and rows : {} / {}",
            lhs.size().columns(),
            rhs.size().rows()
        ));
    }

    let mut ret = SimpleMatrix::new(lhs.size().rows(), rhs.size().columns())?;

    for i in 0..(lhs.size().rows()) {
        for j in 0..(rhs.size().columns()) {
            let mut sum: M = Default::default();

            for k in 0..(lhs.size().columns()) {
                if let (Ok(Some(lhs)), Ok(Some(rhs))) = (lhs.get(i, k), rhs.get(k, j)) {
                    sum = sum + *lhs * *rhs;
                }
            }
            ret.set(i, j, sum)?;
        }
    }

    Ok(ret)
}

/// Solve the matrix and return result as vector
pub fn solve<M: Matrix<f32>>(mat: &M, factors: &Vector) -> Result<Vector, anyhow::Error> {
    let mut mat = SimpleMatrix::from_matrix(mat);
    let mut factors = factors.clone();

    // forward deletion
    for k in 0..mat.size().rows() {
        let Some(kv) = mat.get(k, k)?.map(|v| *v) else {
            continue;
        };

        // row povitting

        // normalize `k` row
        for i in 0..mat.size().columns() {
            let Some(v) = mat.get(k, i)?.map(|v| *v) else {
                continue;
            };

            mat.set(k, i, v / kv)?;
        }
        factors[k] /= kv;

        // delete `k` column
        for i in (k + 1)..mat.size().rows() {
            let Some(kv) = mat.get(i, k)?.map(|v| *v) else {
                continue;
            };

            for j in 0..mat.size().columns() {
                let kj = mat.get(k, j)?.map(|v| *v).unwrap_or(0.0);
                let ij = mat.get(i, j)?.map(|v| *v).unwrap_or(0.0);

                mat.set(i, j, ij - kv * kj)?;
            }

            factors[i] -= kv * factors[k];
        }
    }

    // backward substitution
    for k in (0..factors.len()).rev() {
        for i in 0..k {
            let kv = mat.get(i, k)?.map(|v| *v).unwrap_or(0.0);

            for j in 0..mat.size().columns() {
                let kj = mat.get(k, j)?.map(|v| *v).unwrap_or(0.0);
                let ij = mat.get(i, j)?.map(|v| *v).unwrap_or(0.0);

                mat.set(i, j, ij - kv * kj)?;
            }

            factors[i] -= kv * factors[k];
        }
    }

    Ok(factors)
}

/// New type for LU Splitted matrix to reuse
pub struct LUSplit {
    l_matrix: SimpleMatrix<f32>,
    u_matrix: SimpleMatrix<f32>,
}

impl LUSplit {
    /// Get Left Triangle Matrix
    pub fn l(&self) -> &SimpleMatrix<f32> {
        &self.l_matrix
    }

    /// Get Upper Triangle Matrix
    pub fn u(&self) -> &SimpleMatrix<f32> {
        &self.u_matrix
    }
}

/// Implemetation for LU split algorithm
pub fn lu_split(mat: &impl Matrix<f32>) -> Result<LUSplit> {
    if mat.size().rows() != mat.size().columns() {
        return Err(anyhow::anyhow!(
            "can not make the LU split without exponent matrix"
        ));
    }

    let mut l = SimpleMatrix::<f32>::new(mat.size().rows(), mat.size().columns())?;
    let mut u = SimpleMatrix::<f32>::new(mat.size().rows(), mat.size().columns())?;
    let n = mat.size().min_row_or_col();

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

            let u_ij = mat.get(i, j)?.map(|v| *v).unwrap_or(0.) - sum_u;
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

            let l_ji = mat.get(j, i)?.map(|v| *v).unwrap_or(0.) - sum_l;
            let _ = l.set(j, i, l_ji / u.get(i, i)?.map(|v| *v).unwrap_or(1.0));
        }
    }

    Ok(LUSplit {
        l_matrix: l,
        u_matrix: u,
    })
}

/// Get the determinant when it is square matrix and defined
pub fn determinant(mat: &impl Matrix<f32>) -> Option<f32> {
    if let Ok(splited) = lu_split(mat) {
        let u = splited.u();

        let mut sum = 1.0;
        for i in 0..(mat.size().min_row_or_col()) {
            sum *= u.get(i, i).unwrap_or(None).map(|v| *v).unwrap_or(0.0);
        }

        Some(sum)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::{size::Size, sparse::SparseMatrix};
    use anyhow::Result;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_mul_with_i32_matrices() -> Result<()> {
        // Arrange
        let mut lhs = SimpleMatrix::<i32>::new(2, 3)?;
        lhs.set(0, 0, 1)?;
        lhs.set(0, 1, 2)?;
        lhs.set(0, 2, 3)?;
        lhs.set(1, 0, 4)?;
        lhs.set(1, 1, 5)?;
        lhs.set(1, 2, 6)?;

        let mut rhs = SimpleMatrix::<i32>::new(3, 2)?;
        rhs.set(0, 0, 7)?;
        rhs.set(0, 1, 8)?;
        rhs.set(1, 0, 9)?;
        rhs.set(1, 1, 10)?;
        rhs.set(2, 0, 11)?;
        rhs.set(2, 1, 12)?;

        // Act
        let result = mul(&lhs, &rhs)?;

        // Assert
        assert_eq!(result.size().rows(), 2);
        assert_eq!(result.size().columns(), 2);
        assert_eq!(result.get(0, 0)?.map(|v| *v), Some(58));
        assert_eq!(result.get(0, 1)?.map(|v| *v), Some(64));
        assert_eq!(result.get(1, 0)?.map(|v| *v), Some(139));
        assert_eq!(result.get(1, 1)?.map(|v| *v), Some(154));
        Ok(())
    }

    #[test]
    fn test_mul_with_f32_matrices() -> Result<()> {
        // Arrange
        let mut lhs = SimpleMatrix::<f32>::new(2, 3)?;
        lhs.set(0, 0, 1.5)?;
        lhs.set(0, 1, 2.5)?;
        lhs.set(0, 2, 3.5)?;
        lhs.set(1, 0, 4.5)?;
        lhs.set(1, 1, 5.5)?;
        lhs.set(1, 2, 6.5)?;

        let mut rhs = SimpleMatrix::<f32>::new(3, 2)?;
        rhs.set(0, 0, 1.0)?;
        rhs.set(0, 1, 2.0)?;
        rhs.set(1, 0, 3.0)?;
        rhs.set(1, 1, 4.0)?;
        rhs.set(2, 0, 5.0)?;
        rhs.set(2, 1, 6.0)?;

        // Act
        let result = mul(&lhs, &rhs)?;

        // Assert
        assert_eq!(result.size().rows(), 2);
        assert_eq!(result.size().columns(), 2);
        assert_eq!(result.get(0, 0)?.map(|v| *v), Some(26.5));
        assert_eq!(result.get(0, 1)?.map(|v| *v), Some(34.0));
        assert_eq!(result.get(1, 0)?.map(|v| *v), Some(53.5));
        assert_eq!(result.get(1, 1)?.map(|v| *v), Some(70.0));
        Ok(())
    }

    #[test]
    fn test_mul_returns_error_for_incompatible_dimensions() -> Result<()> {
        // Arrange
        let lhs = SimpleMatrix::<i32>::new(2, 3)?;
        let rhs = SimpleMatrix::<i32>::new(2, 2)?;

        // Act
        let result = mul(&lhs, &rhs);

        // Assert
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_determinant_2x2_matrix() -> Result<()> {
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
        let det = determinant(&matrix);

        // Assert
        assert_eq!(det, Some(-2.0));
        Ok(())
    }

    #[test]
    fn test_determinant_3x3_matrix() -> Result<()> {
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
        let det = determinant(&matrix);

        // Assert
        assert_eq!(det, Some(0.0));
        Ok(())
    }

    #[test]
    fn test_determinant_identity_matrix() -> Result<()> {
        // Arrange
        // Identity matrix has determinant = 1
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(1, 1, 1.0)?;
        matrix.set(2, 2, 1.0)?;

        // Act
        let det = determinant(&matrix);

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
    ) -> Result<()> {
        // Arrange
        let matrix = SimpleMatrix::<f32>::new(rows, cols)?;

        // Act
        let det = determinant(&matrix);

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

    #[test]
    fn test_determinant_with_sparse_matrix() -> Result<()> {
        // Arrange
        // Create a sparse 3x3 matrix with mostly zero values
        // Matrix: | 2  0  1 |
        //         | 0  3  0 |
        //         | 1  0  2 |
        // det = 2*(3*2 - 0*0) - 0*(0*2 - 0*1) + 1*(0*0 - 3*1)
        //     = 2*6 - 0 + 1*(-3)
        //     = 12 - 3 = 9
        let mut source = SimpleMatrix::<f32>::new(3, 3)?;
        source.set(0, 0, 2.0)?;
        source.set(0, 2, 1.0)?;
        source.set(1, 1, 3.0)?;
        source.set(2, 0, 1.0)?;
        source.set(2, 2, 2.0)?;
        let sparse_matrix = SparseMatrix::from_matrix(&source);

        // Act
        let det = determinant(&sparse_matrix);

        // Assert
        assert_eq!(det, Some(9.0));
        Ok(())
    }

    #[test]
    fn test_mul_with_identity_matrix() -> Result<()> {
        // Arrange
        // Matrix A (2x2):  | 3.0  4.0 |
        //                  | 5.0  6.0 |
        let mut matrix = SimpleMatrix::<f32>::new(2, 2)?;
        matrix.set(0, 0, 3.0)?;
        matrix.set(0, 1, 4.0)?;
        matrix.set(1, 0, 5.0)?;
        matrix.set(1, 1, 6.0)?;

        // Identity matrix (2x2): | 1.0  0.0 |
        //                         | 0.0  1.0 |
        let mut identity = SimpleMatrix::<f32>::new(2, 2)?;
        identity.set(0, 0, 1.0)?;
        identity.set(1, 1, 1.0)?;

        // Act
        let result = mul(&matrix, &identity)?;

        // Assert
        // Result should equal original matrix
        assert_eq!(result.get(0, 0)?.map(|v| *v), Some(3.0));
        assert_eq!(result.get(0, 1)?.map(|v| *v), Some(4.0));
        assert_eq!(result.get(1, 0)?.map(|v| *v), Some(5.0));
        assert_eq!(result.get(1, 1)?.map(|v| *v), Some(6.0));
        Ok(())
    }

    #[test]
    fn test_mul_with_sparse_matrices() -> Result<()> {
        // Arrange
        // Matrix A (2x3) with some None values:
        //   | 2.0  None  3.0 |
        //   | None 5.0   None|
        let mut lhs = SimpleMatrix::<f32>::new(2, 3)?;
        lhs.set(0, 0, 2.0)?;
        lhs.set(0, 2, 3.0)?;
        lhs.set(1, 1, 5.0)?;

        // Matrix B (3x2) with some None values:
        //   | 1.0  None |
        //   | 4.0  2.0  |
        //   | None 3.0  |
        let mut rhs = SimpleMatrix::<f32>::new(3, 2)?;
        rhs.set(0, 0, 1.0)?;
        rhs.set(1, 0, 4.0)?;
        rhs.set(1, 1, 2.0)?;
        rhs.set(2, 1, 3.0)?;

        // Act
        let result = mul(&lhs, &rhs)?;

        // Assert
        // Result (2x2):
        //   | 2.0*1.0 + 0 + 0           0 + 0 + 3.0*3.0      | = | 2.0  9.0  |
        //   | 0 + 5.0*4.0 + 0           0 + 5.0*2.0 + 0      |   | 20.0 10.0 |
        assert_eq!(result.size(), Size::new(2, 2));
        assert_eq!(result.get(0, 0)?.map(|v| *v), Some(2.0));
        assert_eq!(result.get(0, 1)?.map(|v| *v), Some(9.0));
        assert_eq!(result.get(1, 0)?.map(|v| *v), Some(20.0));
        assert_eq!(result.get(1, 1)?.map(|v| *v), Some(10.0));
        Ok(())
    }

    #[test]
    fn test_mul_with_single_element_matrices() -> Result<()> {
        // Arrange
        let mut lhs = SimpleMatrix::<f32>::new(1, 1)?;
        lhs.set(0, 0, 5.0)?;

        let mut rhs = SimpleMatrix::<f32>::new(1, 1)?;
        rhs.set(0, 0, 3.0)?;

        // Act
        let result = mul(&lhs, &rhs)?;

        // Assert
        assert_eq!(result.size(), Size::new(1, 1));
        assert_eq!(result.get(0, 0)?.map(|v| *v), Some(15.0));
        Ok(())
    }

    #[test]
    fn test_solve_2x2_system() -> Result<()> {
        // Arrange
        // System of equations:
        //   x + 2y = 5
        //   3x + 4y = 11
        // Expected solution: x = 1, y = 2
        let mut matrix = SimpleMatrix::<f32>::new(2, 2)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(0, 1, 2.0)?;
        matrix.set(1, 0, 3.0)?;
        matrix.set(1, 1, 4.0)?;

        let factors = Vector::new(&[5.0, 11.0])?;

        // Act
        let result = solve(&matrix, &factors)?;

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 2.0);
        Ok(())
    }

    #[test]
    fn test_solve_3x3_system() -> Result<()> {
        // Arrange
        // System of equations:
        //   2x + y + z = 5
        //   x + 2y + z = 5
        //   x + y + 2z = 6
        // Expected solution: x = 1, y = 1, z = 2
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 2.0)?;
        matrix.set(0, 1, 1.0)?;
        matrix.set(0, 2, 1.0)?;
        matrix.set(1, 0, 1.0)?;
        matrix.set(1, 1, 2.0)?;
        matrix.set(1, 2, 1.0)?;
        matrix.set(2, 0, 1.0)?;
        matrix.set(2, 1, 1.0)?;
        matrix.set(2, 2, 2.0)?;

        let factors = Vector::new(&[5.0, 5.0, 6.0])?;

        // Act
        let result = solve(&matrix, &factors)?;

        // Assert
        assert_eq!(result.len(), 3);
        assert_relative_eq!(result[0], 1.0, epsilon = 1e-5);
        assert_relative_eq!(result[1], 1.0, epsilon = 1e-5);
        assert_relative_eq!(result[2], 2.0, epsilon = 1e-5);
        Ok(())
    }

    #[test]
    fn test_solve_with_singular_matrix() -> Result<()> {
        // Arrange
        // Singular matrix (determinant = 0):
        //   1x + 2y + 3z = 1
        //   2x + 4y + 6z = 2
        //   3x + 6y + 9z = 3
        // The second and third rows are multiples of the first row
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(0, 1, 2.0)?;
        matrix.set(0, 2, 3.0)?;
        matrix.set(1, 0, 2.0)?;
        matrix.set(1, 1, 4.0)?;
        matrix.set(1, 2, 6.0)?;
        matrix.set(2, 0, 3.0)?;
        matrix.set(2, 1, 6.0)?;
        matrix.set(2, 2, 9.0)?;

        let factors = Vector::new(&[1.0, 2.0, 3.0])?;

        // Act
        let result = solve(&matrix, &factors)?;

        // Assert
        // For a singular matrix, the solve function may return values
        // but they involve division by zero or near-zero values
        // Check that result contains NaN or Inf values
        assert_eq!(result.len(), 3);
        assert!(
            result[0].is_nan()
                || result[0].is_infinite()
                || result[1].is_nan()
                || result[1].is_infinite()
                || result[2].is_nan()
                || result[2].is_infinite()
        );
        Ok(())
    }

    #[test]
    fn test_solve_with_identity_matrix() -> Result<()> {
        // Arrange
        // Identity matrix system:
        //   1x + 0y + 0z = 3
        //   0x + 1y + 0z = 5
        //   0x + 0y + 1z = 7
        // Expected solution: x = 3, y = 5, z = 7
        let mut matrix = SimpleMatrix::<f32>::new(3, 3)?;
        matrix.set(0, 0, 1.0)?;
        matrix.set(1, 1, 1.0)?;
        matrix.set(2, 2, 1.0)?;

        let factors = Vector::new(&[3.0, 5.0, 7.0])?;

        // Act
        let result = solve(&matrix, &factors)?;

        // Assert
        assert_eq!(result.len(), 3);
        assert_relative_eq!(result[0], 3.0, epsilon = 1e-5);
        assert_relative_eq!(result[1], 5.0, epsilon = 1e-5);
        assert_relative_eq!(result[2], 7.0, epsilon = 1e-5);
        Ok(())
    }
}
