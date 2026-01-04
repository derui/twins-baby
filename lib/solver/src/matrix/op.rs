/// Operation definition for matrix module.
use std::{
    error::Error,
    ops::{Add, Mul},
};

use crate::matrix::{Matrix, simple::SimpleMatrix};

/// Multiply operation between matrix
pub fn mul<M>(lhs: impl Matrix<M>, rhs: impl Matrix<M>) -> Result<impl Matrix<M>, Box<dyn Error>>
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
                match (lhs.get(i, k), rhs.get(k, j)) {
                    (Ok(Some(lhs)), Ok(Some(rhs))) => {
                        sum = sum + lhs * rhs;
                    }
                    _ => (),
                }
            }
            ret.set(i, j, sum)?;
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        let result = mul(lhs, rhs)?;

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
        let result = mul(lhs, rhs)?;

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
        let result = mul(lhs, rhs);

        // Assert
        assert!(result.is_err());
        Ok(())
    }
}
