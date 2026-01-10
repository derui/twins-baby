use std::collections::HashMap;

use crate::{
    environment::Environment,
    matrix::{size::Size, sparse::SparseMatrix},
};

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

/// status to indicate if the dimension specification is correct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionSpecificationStatus {
    /// Incorrect dimension specification
    Incorrect,
    /// Correct dimension specification
    Correct,
}

/// Solver struct.
pub struct Solver {
    status: DimensionSpecificationStatus,

    /// Current jacobian. It is updated when equation or variable updated.
    jacobian: Jacobian,

    /// Current variables
    variables: Environment,

    /// dimensions for solver. There are similar to constant while solving
    dimensions: Environment,

    /// current equations with equation id. Currently, equation id is not
    /// member of equation mod, because equation does not have identity of it.
    equations: HashMap<EquationId, Box<dyn equation::Equation>>,

    generator: Box<dyn EquationIdGenerator>,
}

/// Trait for specialized generating equation id
pub trait EquationIdGenerator: EquationIdGeneratorClone {
    /// Generate a new equation Id
    fn generate(&mut self) -> EquationId;
}

pub trait EquationIdGeneratorClone {
    fn clone_box(&self) -> Box<dyn EquationIdGenerator>;
}

impl<T> EquationIdGeneratorClone for T
where
    T: 'static + EquationIdGenerator + Clone,
{
    fn clone_box(&self) -> Box<dyn EquationIdGenerator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EquationIdGenerator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct DefaultEquationIdGenerator {
    current: u64,
}

impl Default for DefaultEquationIdGenerator {
    fn default() -> Self {
        Self { current: 0 }
    }
}

impl EquationIdGenerator for DefaultEquationIdGenerator {
    fn generate(&mut self) -> EquationId {
        let new_id = (self.current + 1).into();
        self.current += 1;
        new_id
    }
}

impl Solver {
    /// make new solver
    fn new(generator: Box<dyn EquationIdGenerator>) -> Self {
        Solver {
            status: DimensionSpecificationStatus::Incorrect,
            jacobian: Jacobian(SparseMatrix::empty(Size::new(1, 1)).expect("should be suceess")),
            variables: Environment::empty(),
            dimensions: Environment::empty(),
            equations: HashMap::new(),
            generator: generator.clone(),
        }
    }
}
