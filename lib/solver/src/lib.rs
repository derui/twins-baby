use std::collections::HashMap;

use crate::{environment::Environment, matrix::sparse::SparseMatrix};

pub mod environment;
pub mod equation;
pub mod matrix;
pub mod variable;
pub mod vector;

/// Internal Jacobian matrix. It is a matrix of constraint matrix.
struct Jacobian(SparseMatrix<Box<dyn equation::Equation>>);

/// Wrapper of Equation Id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EquationId(u64);

/// Convertion logics
impl From<u64> for EquationId {
    fn from(value: u64) -> Self {
        EquationId(value)
    }
}

impl From<EquationId> for u64 {
    fn from(value: EquationId) -> Self {
        value.0
    }
}

/// Solver struct.
pub struct Solver {
    /// Current jacobian. It is updated when equation or variable updated.
    jacobian: Jacobian,

    /// Current variables
    variables: Environment,

    /// dimensions for solver. There are similar to constant while solving
    dimensions: Environment,

    /// current equations with equation id. Currently, equation id is not
    /// member of equation mod, because equation does not have identity of it.
    equations: HashMap<EquationId, Box<dyn equation::Equation>>,
}
