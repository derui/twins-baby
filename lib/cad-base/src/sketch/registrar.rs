use solver::{EquationId, equation::Equation, variable::Variable};

use crate::id::VariableId;

/// Single responsibility trait to register a variable
pub trait VariableRegistrar {
    /// Register a variable.
    fn register(&mut self, value: f32) -> VariableId;

    /// De-register a variable if it registered
    fn deregister(&mut self, id: &VariableId) -> Option<Variable>;
}

/// Single responsibility trait to index a variable
pub trait VariableIndex {
    /// Get a variable by id
    fn get(&self, id: &VariableId) -> Option<&Variable>;

    /// Get a mutable variable by id
    fn get_mut(&mut self, id: &VariableId) -> Option<&mut Variable>;
}

/// Single responsibility trait to register a equation
pub trait EquationRegistrar {
    /// Register a equation.
    fn register(&mut self, var: &Equation) -> EquationId;

    /// De-register a equation if it registered
    fn deregister(&mut self, id: &EquationId) -> Option<Equation>;

    /// De-register all equations that are related the [variable].
    ///
    /// # Returns
    /// - de-registered equation and id pairs.
    fn deregister_related_variable(&mut self, variable: &Variable) -> Vec<(EquationId, Equation)>;
}

/// Single responsibility trait to index a equation
pub trait EquationIndex {
    /// Get a equation by id
    fn get(&self, id: &EquationId) -> Option<&Equation>;

    /// Update a equation by id
    ///
    /// Equation does not be mutable because equation does not expose any internal state.
    fn update(&mut self, id: &EquationId, equation: Equation) -> Option<()>;
}
