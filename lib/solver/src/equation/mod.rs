mod constant;
mod variable;

use crate::environment::{self, Environment};

/// Error cases for solving equation
#[derive(Debug, Clone)]
pub enum EquationError {
    /// Can not found variables in the environment
    NoVariableInEnvironment(Vec<String>),
}

/// Equation trait should provide some of the equation behavior of the solver
pub trait Equation {
    /// Evaluate the equation.
    ///
    /// # Arguments
    /// * `env` - current environment
    ///
    /// # Returns
    /// result of equation with the environment. Error when some errors
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError>;
}
