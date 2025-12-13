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
pub trait Equation {
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

    /// Format the equation for debugging purposes.
    ///
    /// # Arguments
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    /// Result of the formatting operation
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Clone for Box<dyn Equation> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for Box<dyn Equation> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

impl Into<Box<dyn Equation>> for &dyn Equation {
    fn into(self) -> Box<dyn Equation> {
        self.clone_box()
    }
}
