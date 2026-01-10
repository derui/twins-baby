use crate::{environment::Environment, matrix::sparse::SparseMatrix};

pub mod environment;
pub mod equation;
pub mod matrix;
pub mod variable;
pub mod vector;

/// Internal Jacobian matrix. It is a matrix of constraint matrix.
struct Jacobian(SparseMatrix<Box<dyn equation::Equation>>);

/// Solver struct.
pub struct Solver {
    /// Current jacobian. It is updated when equation or variable updated.
    jacobian: Jacobian,

    /// Current variables
    variables: Environment,

    /// dimensions for solver. There are similar to constant while solving
    dimensions: Environment,
}
