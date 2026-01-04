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

    for i in 0..(ret.size().rows()) {
        for j in 0..(ret.size().columns()) {
            let mut sum: M = Default::default();
            for k in 0..(rhs.size().columns()) {
                match (lhs.get(i, k), rhs.get(k, i)) {
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
