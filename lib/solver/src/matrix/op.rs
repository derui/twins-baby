/// Operation definition for matrix module.
use std::{
    error::Error,
    ops::{Add, Mul},
};

use crate::matrix::{Matrix, simple::SimpleMatrix};

/// Multiply operation between matrix
pub fn mul<M>(lhs: &impl Matrix<M>, rhs: &impl Matrix<M>) -> Result<impl Matrix<M>, Box<dyn Error>>
where
    M: Add<Output = M> + Mul<Output = M> + Default + Copy,
{
    if lhs.size().columns() != rhs.size().rows() {
        return Err(format!(
            "Can not multiply different number of columns and rows : {} / {}",
            lhs.size().columns(),
            rhs.size().rows()
        )
        .into());
    }

    let mut ret = SimpleMatrix::new(lhs.size().rows(), rhs.size().columns())?;

    for i in 0..(lhs.size().rows()) {
        for j in 0..(rhs.size().columns()) {
            let mut sum: M = Default::default();

            for k in 0..(lhs.size().columns()) {
                if let (Ok(Some(lhs)), Ok(Some(rhs))) = (lhs.get(i, k), rhs.get(k, j)) {
                    sum = sum + lhs * rhs;
                }
            }
            ret.set(i, j, sum)?;
        }
    }

    Ok(ret)
}

pub struct LUSplit {
    l_matrix: SimpleMatrix<f32>,
    u_matrix: SimpleMatrix<f32>,
}

impl LUSplit {
    pub fn l(&self) -> &SimpleMatrix<f32> {
        &self.l_matrix
    }

    pub fn u(&self) -> &SimpleMatrix<f32> {
        &self.u_matrix
    }
}

/// Implemetation for LU split algorithm
pub fn lu_split(mat: &impl Matrix<f32>) -> Result<LUSplit, Box<dyn Error>> {
    if mat.size().rows() != mat.size().columns() {
        return Err("can not make the LU split without exponent matrix".into());
    }

    let mut l = SimpleMatrix::<f32>::new(mat.size().rows(), mat.size().columns())?;
    let mut u = SimpleMatrix::<f32>::new(mat.size().rows(), mat.size().columns())?;
    let n = mat.size().min();

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

            let u_ij = mat.get(i, j)?.unwrap_or(0.) - sum_u;
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

            let l_ji = mat.get(j, i)?.unwrap_or(0.) - sum_l;
            let _ = l.set(j, i, l_ji / u.get(i, i)?.unwrap_or(1.0));
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
        for i in 0..(mat.size().min()) {
            sum *= u.get(i, i).unwrap_or(None).unwrap_or(0.0);
        }

        Some(sum)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_mul_with_i32_matrices() -> Result<(), Box<dyn Error>> {
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
        assert_eq!(result.get(0, 0)?, Some(58));
        assert_eq!(result.get(0, 1)?, Some(64));
        assert_eq!(result.get(1, 0)?, Some(139));
        assert_eq!(result.get(1, 1)?, Some(154));
        Ok(())
    }

    #[test]
    fn test_mul_with_f32_matrices() -> Result<(), Box<dyn Error>> {
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
        assert_eq!(result.get(0, 0)?, Some(26.5));
        assert_eq!(result.get(0, 1)?, Some(34.0));
        assert_eq!(result.get(1, 0)?, Some(53.5));
        assert_eq!(result.get(1, 1)?, Some(70.0));
        Ok(())
    }

    #[test]
    fn test_mul_returns_error_for_incompatible_dimensions() -> Result<(), Box<dyn Error>> {
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
        let det = determinant(&matrix);

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
        let det = determinant(&matrix);

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
    ) -> Result<(), Box<dyn Error>> {
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
}
