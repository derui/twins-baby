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

#[derive(Debug, Clone, Default)]
pub struct DefaultEquationIdGenerator {
    current: u64,
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
    pub fn new(generator: Box<dyn EquationIdGenerator>) -> Self {
        Solver {
            status: DimensionSpecificationStatus::Incorrect,
            jacobian: Jacobian(SparseMatrix::empty(Size::new(1, 1)).expect("should be suceess")),
            variables: Environment::empty(),
            dimensions: Environment::empty(),
            equations: HashMap::new(),
            generator: generator.clone(),
        }
    }

    /// update variables for solver
    pub fn update_variables(&mut self, env: &Environment) {
        self.variables = env.clone();

        self.recaluculate_status()
    }

    /// Update dimensions for solver
    pub fn update_dimensions(&mut self, env: &Environment) {
        self.dimensions = env.clone()
    }

    /// Re-calculate dimension specification
    fn recaluculate_status(&mut self) {
        let variable_count = self.variables.list_variables().len();
        let equation_count = self.equations.values().len();

        if variable_count == equation_count {
            self.status = DimensionSpecificationStatus::Correct
        } else {
            self.status = DimensionSpecificationStatus::Incorrect
        }
    }
}
