use std::collections::HashSet;

use anyhow::Result;
use immutable::Im;
use solver::equation::{Equation, Evaluate};

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
    /// Make a new constraint with the equation.
    ///
    /// # Summary
    /// This function will extract the related variables from the equation and map them to their corresponding VariableId using the provided VariableScope.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the constraint.
    /// * `equation` - The equation representing the constraint.
    /// * `scope` - The variable scope used to map variable names to their IDs.
    ///
    /// # Returns
    /// * `Result<Constraint>` - Returns a Result containing the newly created Constraint or an error if any variable in the equation is not found in the scope.
    pub fn new(id: ConstraintId, equation: Equation, scope: &VariableScope) -> Result<Self> {
        let mut vars: HashSet<String> = HashSet::from_iter(equation.related_variables());

        let env = scope.to_id_name_map();
        let mut related = vec![];

        for var in vars.clone() {
            if let Some(v) = env.get(&var) {
                related.push(*v);
                vars.remove(&var);
            }
        }

        if related.len() != vars.len() {
            let vars = vars.iter().cloned().collect::<Vec<_>>();
            return Err(anyhow::anyhow!(
                "Do not found variables {}",
                vars.join(", ")
            ));
        }

        Ok(Constraint {
            id: id.into(),
            equation: equation.into(),
            related_variables: related.into(),
        })
    }
}
