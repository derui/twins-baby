use std::collections::HashSet;

use color_eyre::eyre::Result;
use immutable::Im;
use solver::equation::{Equation, Evaluate};

use crate::{id::VariableId, sketch::scope::VariableScope};

/// Constraint between variables
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Name of equation
    pub name: Im<String>,

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
    pub fn new(name: &str, equation: Equation, scope: &VariableScope) -> Result<Self> {
        let mut vars: HashSet<String> = HashSet::from_iter(equation.related_variables());

        let env = scope.to_id_name_map();
        let mut related = vec![];

        for var in vars.clone() {
            if let Some(v) = env.get(&var) {
                related.push(*v);
                vars.remove(&var);
            }
        }

        if !vars.is_empty() {
            let vars = vars.iter().cloned().collect::<Vec<_>>();
            return Err(color_eyre::eyre::eyre!(
                "Do not found variables {}",
                vars.join(", ")
            ));
        }

        Ok(Constraint {
            name: name.to_string().into(),
            equation: equation.into(),
            related_variables: related.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use solver::equation::parse;

    #[test]
    fn creates_constraint_with_no_variables() {
        // Arrange
        let equation = parse("5.0").unwrap();
        let scope = VariableScope::new();

        // Act
        let result = Constraint::new("constant_constraint", equation.clone(), &scope);

        // Assert
        let constraint = result.unwrap();
        assert_eq!(*constraint.name, "constant_constraint");
        assert_eq!(*constraint.equation, equation);
        assert_eq!(constraint.related_variables.len(), 0);
    }

    #[test]
    fn creates_constraint_with_single_variable() {
        // Arrange
        let mut scope = VariableScope::new();
        let var_id = scope.register(5.0);
        let equation = parse(&var_id.to_string()).unwrap();

        // Act
        let result = Constraint::new("single_var_constraint", equation, &scope);

        // Assert
        let constraint = result.unwrap();
        assert_eq!(constraint.related_variables.len(), 1);
        assert_eq!(constraint.related_variables[0], var_id);
    }

    #[test]
    fn creates_constraint_with_multiple_variables() {
        // Arrange
        let mut scope = VariableScope::new();
        let var1 = scope.register(1.0);
        let var2 = scope.register(2.0);
        let var3 = scope.register(3.0);
        let equation = parse(&format!("{} + {} + {}", var1, var2, var3)).unwrap();

        // Act
        let result = Constraint::new("multi_var_constraint", equation, &scope);

        // Assert
        let constraint = result.unwrap();
        assert_eq!(constraint.related_variables.len(), 3);
        assert!(constraint.related_variables.contains(&var1));
        assert!(constraint.related_variables.contains(&var2));
        assert!(constraint.related_variables.contains(&var3));
    }

    #[test]
    fn deduplicates_repeated_variables() {
        // Arrange
        let mut scope = VariableScope::new();
        let var_id = scope.register(5.0);
        let equation = parse(&format!("{} + {} * 2.0", var_id, var_id)).unwrap();

        // Act
        let result = Constraint::new("duplicate_var_constraint", equation, &scope);

        // Assert
        let constraint = result.unwrap();
        assert_eq!(constraint.related_variables.len(), 1);
        assert_eq!(constraint.related_variables[0], var_id);
    }

    #[test]
    fn returns_error_when_all_variables_missing_from_scope() {
        // Arrange
        let equation = parse("x").unwrap();
        let scope = VariableScope::new();

        // Act
        let result = Constraint::new("missing_var_constraint", equation, &scope);

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Do not found variables")
        );
    }

    #[test]
    fn returns_error_when_some_variables_missing_from_scope() {
        // Arrange
        let mut scope = VariableScope::new();
        let var1 = scope.register(1.0);
        let equation = parse(&format!("{} + unknown", var1)).unwrap();

        // Act
        let result = Constraint::new("partial_missing_constraint", equation, &scope);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn uses_only_relevant_variables_from_scope() {
        // Arrange
        let mut scope = VariableScope::new();
        let var1 = scope.register(1.0);
        scope.register(2.0); // var2 - not used
        scope.register(3.0); // var3 - not used
        let equation = parse(&var1.to_string()).unwrap();

        // Act
        let result = Constraint::new("relevant_var_constraint", equation, &scope);

        // Assert
        let constraint = result.unwrap();
        assert_eq!(constraint.related_variables.len(), 1);
        assert_eq!(constraint.related_variables[0], var1);
    }
}
