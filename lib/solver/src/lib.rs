use std::{collections::HashMap, error::Error};

use crate::{
    environment::Environment,
    equation::Equation,
    matrix::{Matrix, op::solve, simple::SimpleMatrix, size::Size, sparse::SparseMatrix},
    variable::Variable,
    vector::{TransposeMethod, Vector},
};

pub mod environment;
pub mod equation;
pub mod matrix;
pub mod variable;
pub mod vector;

const DEFAULT_EPSILON: f32 = 0.0000001;

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

    /// The resolution of solving
    epsilon: f32,
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
            epsilon: DEFAULT_EPSILON,
        }
    }

    /// Get the status
    pub fn status(&self) -> DimensionSpecificationStatus {
        self.status
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

    /// Solve current equations and get variables.
    pub fn solve(&mut self) -> Result<Environment, Box<dyn Error>> {
        if self.status != DimensionSpecificationStatus::Correct {
            return Err("Can not solve incorrect solver".into());
        }

        // make direct solve
        // x_1 = x_0 - J_0^-1 * f_0 -> J_0 * x_1 = J_0 * x_0 - f_0 -> Ax = b
        let mut ordered = self.variables.list_variables();
        ordered.sort_by_key(|f| f.name());
        let mut equation_order = self.equations.keys().collect::<Vec<_>>();
        equation_order.sort();
        let equations: Vec<&Box<dyn Equation>> =
            equation_order.iter().map(|k| &self.equations[k]).collect();

        // initial value
        let mut x0 = Vector::new(&ordered.iter().map(|f| f.value()).collect::<Vec<_>>())?;

        let j0 = {
            let extractor = self.variables.clone();
            self.jacobian.0.extract(move |e| {
                e.evaluate(&extractor)
                    .expect("this evaluation must success")
            })
        };

        // resolve targets
        let x1 = Vector::zero(x0.len())?;

        loop {
            if (x0.norm() - x1.norm()).abs() < self.epsilon {
                break;
            }

            // calculate rhs. this result is simple vector that is row-transposed
            let b = {
                let extractor = self.variables.clone();
                let f0 = equations
                    .iter()
                    .map(|e| e.evaluate(&extractor).unwrap_or(0.0))
                    .collect::<Vec<_>>();
                let x0 = x0.to_matrix(TransposeMethod::Row);
                let b = matrix::op::mul(&j0, &x0)?;
                let mut ret = Vector::zero(ordered.len())?;

                for i in 0..ret.len() {
                    ret[i] = b.get(i, 0)?.unwrap_or(0.0) - f0[i];
                }

                ret
            };
            let x1 = solve(&j0, &b)?;

            for i in 0..(ordered.len()) {
                self.variables.update_variable(&ordered[i].name(), x1[i])?;
                x0[i] = x1[i]
            }
        }

        let mut result = Environment::empty();

        for i in 0..(ordered.len()) {
            let mut v = ordered[i].clone();
            v.update(x1[i]);
            result.add_variable(v);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod status {
        use crate::environment::Environment;
        use crate::equation::Equation;
        use crate::equation::constant::ConstantEquation;
        use crate::variable::Variable;
        use crate::{DefaultEquationIdGenerator, DimensionSpecificationStatus, Solver};
        use pretty_assertions::assert_eq;

        #[test]
        fn test_new_solver_has_incorrect_status() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());

            // Act
            let solver = Solver::new(generator);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Incorrect);
        }

        #[test]
        fn test_status_remains_incorrect_with_only_variables() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let env = Environment::from_variables(vec![Variable::new("x", 1.0)]);

            // Act
            solver.update_variables(&env);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Incorrect);
        }

        #[test]
        fn test_status_remains_incorrect_with_only_equations() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let equation: Box<dyn Equation> = Box::new(ConstantEquation::new(1.0));

            // Act
            solver.add_equation(equation);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Incorrect);
        }

        #[test]
        fn test_status_becomes_correct_when_variable_and_equation_counts_match() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let env = Environment::from_variables(vec![Variable::new("x", 1.0)]);
            let equation: Box<dyn Equation> = Box::new(ConstantEquation::new(1.0));

            // Act
            solver.add_equation(equation);
            solver.update_variables(&env);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Correct);
        }

        #[test]
        fn test_status_becomes_correct_with_multiple_variables_and_equations() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let env = Environment::from_variables(vec![
                Variable::new("x", 1.0),
                Variable::new("y", 2.0),
                Variable::new("z", 3.0),
            ]);

            // Act
            solver.add_equation(Box::new(ConstantEquation::new(1.0)));
            solver.add_equation(Box::new(ConstantEquation::new(2.0)));
            solver.add_equation(Box::new(ConstantEquation::new(3.0)));
            solver.update_variables(&env);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Correct);
        }

        #[test]
        fn test_status_becomes_incorrect_when_adding_more_variables() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let env1 = Environment::from_variables(vec![Variable::new("x", 1.0)]);
            solver.add_equation(Box::new(ConstantEquation::new(1.0)));
            solver.update_variables(&env1);

            // Act
            let env2 =
                Environment::from_variables(vec![Variable::new("x", 1.0), Variable::new("y", 2.0)]);
            solver.update_variables(&env2);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Incorrect);
        }

        #[test]
        fn test_status_becomes_incorrect_after_removing_equation() {
            // Arrange
            let generator = Box::new(DefaultEquationIdGenerator::default());
            let mut solver = Solver::new(generator);
            let env = Environment::from_variables(vec![Variable::new("x", 1.0)]);
            let eq_id = solver.add_equation(Box::new(ConstantEquation::new(1.0)));
            solver.update_variables(&env);

            // Act
            solver.remove_equation(eq_id);

            // Assert
            assert_eq!(solver.status(), DimensionSpecificationStatus::Incorrect);
        }
    }
}
