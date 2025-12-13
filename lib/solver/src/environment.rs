use std::collections::HashMap;

use crate::variable::Variable;

/// The enviromnet that has all variables in a scope
///
/// An environment tracks all variables in a scope, and can update them by method. But current implementation is too simple,
/// because it can not update variables simoultaneously.
#[derive(Debug, Clone)]
pub struct Environment {
    /// variables in this environment
    variables: HashMap<String, Variable>,
}

impl Environment {
    /// Get a new empty environment
    pub fn empty() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    /// Get a new environment with variables.
    ///
    /// NOTICE: If there are duplicated variable names, the last one will be used.
    ///
    /// # Arguments
    /// * `variables` - A vector of variables to be included in the environment
    ///
    /// # Returns
    /// * `Environment` - A new environment containing the provided variables
    pub fn from_variables(variables: Vec<Variable>) -> Self {
        let mut vars_map = HashMap::new();
        for var in variables {
            vars_map.insert(var.name().to_string(), var);
        }
        Environment {
            variables: vars_map,
        }
    }

    /// Add a variable to the environment
    ///
    /// This method will override variable if there is already a variable with the same name.
    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.insert(variable.name(), variable);
    }

    /// Remove a variable from the environment
    pub fn remove_variable(&mut self, variable: &Variable) {
        // ignore errors
        self.variables.remove(&variable.name());
    }

    /// Update the variable
    ///
    /// # Arguments
    /// * `name` - The name of the variable to update
    /// * `value` - The new value to set for the variable
    ///
    /// # Returns
    /// * `Result<Variable, String>` - Ok if the variable was updated successfully, Err with a message if the variable was not found
    pub fn update_variable(&mut self, name: &str, value: f32) -> Result<Variable, String> {
        if let Some(var) = self.variables.get_mut(name) {
            var.update(value);
            Ok(var.clone())
        } else {
            Err(format!("Variable '{}' not found in the environment", name))
        }
    }

    /// Get a variable copy by name
    ///
    /// Updating returned variable will not update the variable in environment.
    pub fn get_variable(&self, name: &str) -> Option<Variable> {
        self.variables.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_empty_creates_empty_environment() {
        // arrange

        // act
        let env = Environment::empty();

        // assert
        assert_eq!(env.variables.len(), 0);
    }

    #[test]
    fn test_from_variables_creates_environment_with_variables() {
        // arrange
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        let var3 = Variable::new("z", 30.0);

        // act
        let env = Environment::from_variables(vec![var1.clone(), var2.clone(), var3.clone()]);

        // assert
        assert_eq!(env.variables.len(), 3);
        assert_eq!(env.get_variable("x"), Some(var1));
        assert_eq!(env.get_variable("y"), Some(var2));
        assert_eq!(env.get_variable("z"), Some(var3));
    }

    #[test]
    fn test_from_variables_with_duplicates_uses_last_one() {
        // arrange
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("x", 20.0);
        let var3 = Variable::new("x", 30.0);

        // act
        let env = Environment::from_variables(vec![var1, var2, var3.clone()]);

        // assert
        assert_eq!(env.variables.len(), 1);
        assert_eq!(env.get_variable("x"), Some(var3));
    }

    #[test]
    fn test_add_variable_adds_new_variable() {
        // arrange
        let mut env = Environment::empty();
        let var = Variable::new("x", 10.0);

        // act
        env.add_variable(var.clone());

        // assert
        assert_eq!(env.variables.len(), 1);
        assert_eq!(env.get_variable("x"), Some(var));
    }

    #[test]
    fn test_add_variable_overrides_existing_variable() {
        // arrange
        let mut env = Environment::empty();
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("x", 20.0);
        env.add_variable(var1);

        // act
        env.add_variable(var2.clone());

        // assert
        assert_eq!(env.variables.len(), 1);
        assert_eq!(env.get_variable("x"), Some(var2));
    }

    #[test]
    fn test_remove_variable_removes_existing_variable() {
        // arrange
        let mut env = Environment::empty();
        let var = Variable::new("x", 10.0);
        env.add_variable(var.clone());

        // act
        env.remove_variable(&var);

        // assert
        assert_eq!(env.variables.len(), 0);
        assert_eq!(env.get_variable("x"), None);
    }

    #[test]
    fn test_remove_variable_ignores_non_existing_variable() {
        // arrange
        let mut env = Environment::empty();
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        env.add_variable(var1.clone());

        // act
        env.remove_variable(&var2);

        // assert
        assert_eq!(env.variables.len(), 1);
        assert_eq!(env.get_variable("x"), Some(var1));
    }

    #[test]
    fn test_update_variable_updates_existing_variable() {
        // arrange
        let mut env = Environment::empty();
        let var = Variable::new("x", 10.0);
        env.add_variable(var);

        // act
        let result = env.update_variable("x", 25.0);

        // assert
        let updated_var = env.get_variable("x").unwrap();
        let expected_var = Variable::new("x", 25.0);
        assert_eq!(updated_var, expected_var);
    }

    #[test]
    fn test_update_variable_returns_error_for_non_existing_variable() {
        // arrange
        let mut env = Environment::empty();

        // act
        let result = env.update_variable("x", 10.0);

        // assert
        assert_eq!(
            result.unwrap_err(),
            "Variable 'x' not found in the environment"
        );
    }

    #[test]
    fn test_get_variable_returns_some_for_existing_variable() {
        // arrange
        let mut env = Environment::empty();
        let var = Variable::new("x", 10.0);
        env.add_variable(var.clone());

        // act
        let result = env.get_variable("x");

        // assert
        assert_eq!(result, Some(var));
    }

    #[test]
    fn test_get_variable_returns_none_for_non_existing_variable() {
        // arrange
        let env = Environment::empty();

        // act
        let result = env.get_variable("x");

        // assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_variable_returns_copy_not_reference() {
        // arrange
        let mut env = Environment::empty();
        let var = Variable::new("x", 10.0);
        env.add_variable(var);

        // act
        let mut copied_var = env.get_variable("x").unwrap();
        copied_var.update(20.0);

        // assert
        let original_var = env.get_variable("x").unwrap();
        let expected_original = Variable::new("x", 10.0);
        assert_eq!(original_var, expected_original);
    }
}
