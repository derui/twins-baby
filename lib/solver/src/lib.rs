use std::{collections::HashMap, error::Error};

use crate::{
    environment::Environment,
    equation::Equation,
    matrix::{Matrix, simple::SimpleMatrix, size::Size, sparse::SparseMatrix},
    variable::Variable,
};

pub mod environment;
pub mod equation;
pub mod matrix;
pub mod variable;
pub mod vector;

/// Internal Jacobian matrix. It is a matrix of constraint matrix.
struct Jacobian(SparseMatrix<Box<dyn equation::Equation>>);

impl Jacobian {
    /// Create Jacobian from equations and variables
    fn from_equations(
        equations: &[Box<dyn Equation>],
        variables: &[Variable],
    ) -> Result<Self, Box<dyn Error>> {
        if equations.len() != variables.len() {
            return Err("Can not create valid jacobian".into());
        }

        let mut matrix = SimpleMatrix::new(equations.len(), equations.len())?;

        for (i, equation) in equations.iter().enumerate() {
            for (j, variable) in variables.iter().enumerate() {
                // keep empty when can not derive
                let Some(derived) = equation.derive(variable) else {
                    continue;
                };
                matrix.set(i, j, derived)?;
            }
        }

        Ok(Jacobian(SparseMatrix::from_matrix(&matrix)))
    }
}

/// Wrapper of Equation Id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    equations: HashMap<EquationId, Box<dyn Equation>>,

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
    /// Creates a new solver instance with the given equation ID generator.
    ///
    /// # Parameters
    /// * `generator` - Custom equation ID generator for creating unique equation identifiers
    ///
    /// # Returns
    /// * A new solver with empty variables, dimensions, and equations
    ///
    /// # Initial State
    /// * Status is set to `Incorrect` until variables match equation count
    /// * Jacobian is initialized with minimal 1x1 sparse matrix
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

    /// Updates the solver's variable environment and recalculates dimension specification status.
    ///
    /// # Parameters
    /// * `env` - New environment containing variable definitions
    pub fn update_variables(&mut self, env: &Environment) {
        self.variables = env.clone();

        self.recaluculate_status()
    }

    /// Updates the solver's dimension environment.
    ///
    /// Dimensions act as constants during the solving process and are not modified
    /// by the solver iterations.
    ///
    /// # Parameters
    /// * `env` - New environment containing dimension definitions
    pub fn update_dimensions(&mut self, env: &Environment) {
        self.dimensions = env.clone()
    }

    /// Recalculates the dimension specification status.
    ///
    /// Compares the number of variables against the number of equations to determine
    /// if the system is properly specified for solving.
    ///
    /// # Status Rules
    /// * `Correct` - Variable count equals equation count (system is solvable)
    /// * `Incorrect` - Variable count differs from equation count (under/over-determined)
    fn recaluculate_status(&mut self) {
        let variable_count = self.variables.list_variables().len();
        let equation_count = self.equations.values().len();

        if variable_count == equation_count && variable_count > 0 {
            self.status = DimensionSpecificationStatus::Correct
        } else {
            self.status = DimensionSpecificationStatus::Incorrect
        }

        match self.status {
            DimensionSpecificationStatus::Incorrect => (),
            DimensionSpecificationStatus::Correct => {
                let mut equations = self.equations.iter().collect::<Vec<_>>();
                equations.sort_by_key(|(k, _)| *k);
                let equations = equations
                    .iter()
                    .map(|(_, v)| *v)
                    .cloned()
                    .collect::<Vec<_>>();

                self.jacobian =
                    Jacobian::from_equations(&equations, &self.variables.list_variables())
                        .expect("Must be valid")
            }
        }
    }

    /// Adds an equation to the solver and returns its unique identifier.
    ///
    /// # Parameters
    /// * `equation` - Boxed equation trait object to add to the system
    ///
    /// # Returns
    /// * `EquationId` - Unique identifier for the added equation
    ///
    /// # Example
    /// ```ignore
    /// let id = solver.add_equation(Box::new(my_equation));
    /// // Use id to reference or remove the equation later
    /// ```
    pub fn add_equation(&mut self, equation: Box<dyn Equation>) -> EquationId {
        let new_id = self.generator.generate();

        self.equations.insert(new_id, equation.clone());

        self.recaluculate_status();

        new_id
    }

    /// Removes an equation from the solver by its identifier.
    ///
    /// # Parameters
    /// * `id` - Unique identifier of the equation to remove
    ///
    /// # Returns
    /// * `Some(Box<dyn Equation>)` - The removed equation if it existed
    /// * `None` - If no equation with the given ID was found
    pub fn remove_equation(&mut self, id: EquationId) -> Option<Box<dyn Equation>> {
        let v = self.equations.remove(&id);

        self.recaluculate_status();

        v
    }
}
