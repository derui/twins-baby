use anyhow::Result;
use immutable::Im;
use solver::equation::Equation;

use crate::{
    id::{ConstraintId, VariableId},
    sketch::scope::VariableScope,
};

/// Constraint between variables
#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: Im<ConstraintId>,

    /// A equation
    pub equation: Im<Equation>,

    /// A variables related of equation
    pub related_variables: Im<Vec<VariableId>>,
}

impl Constraint {
    pub fn new(_id: ConstraintId, _equation: Equation, _scope: &VariableScope) -> Result<Self> {
        todo!()
    }
}
