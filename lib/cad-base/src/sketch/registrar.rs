use solver::{EquationId, equation::Equation, variable::Variable};

use crate::id::VariableId;

/// Single responsibility trait to register a variable
pub trait VariableRegistrar {
    /// Register a variable.
    ///
    /// Notice, requirement for behavior of this function is **able to add same variable as diffirent variable**.
    /// Responsibility of duplication enforces the user to checking.
    /// Because, the name of variable can duplicate easily, so registrar can not determine the variable given is duplicated or not.
    fn register(&mut self, var: &Variable) -> VariableId;

    /// De-register a variable if it registered
    fn deregister(&mut self, id: &VariableId) -> Option<Variable>;
}

pub trait EquationRegistrar {
    /// Register a equation.
    fn register(&mut self, var: &Equation) -> EquationId;

    /// De-register a variable if it registered
    fn deregister(&mut self, id: &EquationId) -> Option<Equation>;

    /// De-register all equations that are related the [variable].
    ///
    /// # Returns
    /// - de-registered equation and id pairs.
    fn deregister_related_variable(&mut self, variable: &Variable) -> Vec<(EquationId, Equation)>;
}
