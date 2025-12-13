pub(crate) mod arithmetic;
pub(crate) mod constant;
pub(crate) mod unary;
pub(crate) mod variable;

use crate::environment::Environment;

/// Error cases for solving equation
#[derive(Debug, Clone)]
pub enum EquationError {
    /// Can not found variables in the environment
    NoVariableInEnvironment(Vec<String>),
}

/// Equation trait should provide some of the equation behavior of the solver
pub trait Equation: std::fmt::Debug {
    /// Evaluate the equation.
    ///
    /// # Arguments
    /// * `env` - current environment
    ///
    /// # Returns
    /// result of equation with the environment. Error when some errors
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError>;

    /// Clone the equation into a Box.
    ///
    /// Equation is trait, but it should be clone as value
    ///
    /// # Returns
    /// A boxed clone of the equation
    fn clone_box(&self) -> Box<dyn Equation>;
}

impl Clone for Box<dyn Equation> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl From<&dyn Equation> for Box<dyn Equation> {
    fn from(val: &dyn Equation) -> Self {
        val.clone_box()
    }
}
