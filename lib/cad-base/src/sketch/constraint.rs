use solver::equation::Equation;

use crate::id::VariableId;

/// Constraint between variables
#[derive(Debug, Clone)]
pub struct Constraint {
    /// A equation
    equation: Equation,

    /// A variables related of equation
    related_variables: Vec<VariableId>,
}
