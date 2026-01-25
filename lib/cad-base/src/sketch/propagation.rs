use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow};
use solver::{environment::Environment, variable::Variable};

use crate::{
    id::{GenerateId, VariableId},
    sketch::registrar::VariableRegistrar,
};

#[derive(Debug, Clone)]
pub struct PropagatableEnv<T: GenerateId<VariableId>> {
    _id_gen: T,

    /// New id and new name mapping.
    ///
    /// [Shape]s in sketch does not know the real name of variable, they only knows the id of the variable they own.
    id_map: HashMap<VariableId, String>,

    /// The real [Environment] .
    environment: Environment,
}

impl<T: GenerateId<VariableId>> VariableRegistrar for PropagatableEnv<T> {
    /// Register a variable to the env.
    fn register(&mut self, var: &Variable) -> VariableId {
        let id = self._id_gen.generate();
        let new_name = format!("{}_{}", id, var.name());

        // add variable with new name. the name is global in a sketch
        self.environment.add_variable(var.with_name(&new_name));
        self.id_map.insert(id, new_name);

        id
    }

    fn deregister(&mut self, id: &VariableId) -> Option<Variable> {
        let var = self.id_map.remove(id)?;

        self.environment.remove_variable(&var)
    }
}

impl<T: GenerateId<VariableId>> PropagatableEnv<T> {
    /// Make a new [PropagatableEnv] with a [GenerateId] trait
    pub fn from_id_gen(id_gen: T) -> Self {
        PropagatableEnv {
            _id_gen: id_gen,
            id_map: HashMap::new(),
            environment: Environment::empty(),
        }
    }

    /// De-register the variable from env.
    pub fn deregister(&mut self, id: &VariableId) -> Result<()> {
        let Some(var) = self.id_map.remove(id) else {
            return Err(anyhow!("Not found id: {}", id));
        };
        self.environment.remove_variable(&var);

        Ok(())
    }

    /// Get a reference of the environment
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Propagate the change of the variable to given reference.
    pub fn propagate(&self, id: &VariableId, var: &mut Variable) -> Result<()> {
        let Some(name) = self.id_map.get(id) else {
            return Err(anyhow!("Do not found id {}", id));
        };

        let Some(internal_var) = self.environment.get_variable(name) else {
            return Err(anyhow!("Do not found id {}", id));
        };
        var.update(internal_var.value());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::DefaultIdGenerator;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    fn create_env() -> PropagatableEnv<DefaultIdGenerator<VariableId>> {
        PropagatableEnv::from_id_gen(DefaultIdGenerator::default())
    }

    #[test]
    fn test_from_id_gen_creates_empty_environment() {
        // Arrange & Act
        let env = create_env();

        // Assert
        assert_eq!(env.environment().list_variables().len(), 0);
    }

    #[test]
    fn test_register_multiple_variables_generates_unique_ids() {
        // Arrange
        let mut env = create_env();
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        let var3 = Variable::new("z", 30.0);

        // Act
        let id1 = env.register(&var1);
        let id2 = env.register(&var2);
        let id3 = env.register(&var3);

        // Assert
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
        assert_eq!(env.environment().list_variables().len(), 3);
    }

    #[test]
    fn test_register_creates_correctly_named_variable() {
        // Arrange
        let mut env = create_env();
        let var = Variable::new("position", 5.5);

        // Act
        let id = env.register(&var);

        // Assert
        let expected_name = format!("{}_{}", id, "position");
        let registered_var = env.environment().get_variable(&expected_name);
        assert!(registered_var.is_some());
        assert_eq!(registered_var.unwrap().name(), expected_name);
    }

    #[test]
    fn test_deregister_removes_registered_variable() {
        // Arrange
        let mut env = create_env();
        let var = Variable::new("x", 10.0);
        let id = env.register(&var);

        // Act
        let result = env.deregister(&id);

        // Assert
        assert!(result.is_ok());
        assert_eq!(env.environment().list_variables().len(), 0);
    }

    #[test]
    fn test_deregister_non_existent_id_returns_error() {
        // Arrange
        let mut env = create_env();
        let non_existent_id = VariableId::from(9999u64);

        // Act
        let result = env.deregister(&non_existent_id);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Not found id: {}", non_existent_id)
        );
    }

    #[test]
    fn test_deregister_already_deregistered_id_returns_error() {
        // Arrange
        let mut env = create_env();
        let var = Variable::new("x", 10.0);
        let id = env.register(&var);
        let _ = env.deregister(&id);

        // Act
        let result = env.deregister(&id);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_propagate_updates_variable_with_environment_value() {
        // Arrange
        let mut env = create_env();
        let original_var = Variable::new("x", 10.0);
        let id = env.register(&original_var);

        // Simulate solver updating the variable in the environment
        let var_name = format!("{}_{}", id, "x");
        let _ = env.environment.update_variable(&var_name, 25.5);

        let mut external_var = Variable::new("x", 10.0);

        // Act
        let result = env.propagate(&id, &mut external_var);

        // Assert
        assert!(result.is_ok());
        assert_relative_eq!(external_var.value(), 25.5);
    }

    #[test]
    fn test_propagate_with_non_existent_id_returns_error() {
        // Arrange
        let env = create_env();
        let non_existent_id = VariableId::from(9999u64);
        let mut var = Variable::new("x", 10.0);

        // Act
        let result = env.propagate(&non_existent_id, &mut var);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Do not found id {}", non_existent_id)
        );
    }

    #[test]
    fn test_propagate_does_not_change_variable_name() {
        // Arrange
        let mut env = create_env();
        let original_var = Variable::new("position", 10.0);
        let id = env.register(&original_var);

        let var_name = format!("{}_{}", id, "position");
        let _ = env.environment.update_variable(&var_name, 42.0);

        let mut external_var = Variable::new("position", 10.0);

        // Act
        let _ = env.propagate(&id, &mut external_var);

        // Assert
        assert_eq!(external_var.name(), "position");
        assert_relative_eq!(external_var.value(), 42.0);
    }

    #[test]
    fn test_workflow_register_update_propagate() {
        // Arrange
        let mut env = create_env();
        let var1 = Variable::new("x", 1.0);
        let var2 = Variable::new("y", 2.0);

        let id1 = env.register(&var1);
        let id2 = env.register(&var2);

        // Act - Simulate solver updating variables
        let _ = env
            .environment
            .update_variable(&format!("{}_{}", id1, "x"), 100.0);
        let _ = env
            .environment
            .update_variable(&format!("{}_{}", id2, "y"), 200.0);

        let mut result_var1 = Variable::new("x", 0.0);
        let mut result_var2 = Variable::new("y", 0.0);
        let _ = env.propagate(&id1, &mut result_var1);
        let _ = env.propagate(&id2, &mut result_var2);

        // Assert
        assert_relative_eq!(result_var1.value(), 100.0);
        assert_relative_eq!(result_var2.value(), 200.0);
    }

    #[test]
    fn test_deregister_removes_variable_from_environment() {
        // Arrange
        let mut env = create_env();
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        let id1 = env.register(&var1);
        let id2 = env.register(&var2);

        // Act
        let _ = env.deregister(&id1);

        // Assert
        assert_eq!(env.environment().list_variables().len(), 1);
        let var_name = format!("{}_{}", id2, "y");
        assert!(env.environment().get_variable(&var_name).is_some());
    }

    #[test]
    fn test_propagate_after_deregister_returns_error() {
        // Arrange
        let mut env = create_env();
        let var = Variable::new("x", 10.0);
        let id = env.register(&var);
        let _ = env.deregister(&id);
        let mut external_var = Variable::new("x", 10.0);

        // Act
        let result = env.propagate(&id, &mut external_var);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_register_same_variable_name_multiple_times() {
        // Arrange
        let mut env = create_env();
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("x", 20.0);

        // Act
        let id1 = env.register(&var1);
        let id2 = env.register(&var2);

        // Assert
        assert_ne!(id1, id2);
        assert_eq!(env.environment().list_variables().len(), 2);

        // Both variables should exist with different internal names
        let name1 = format!("{}_{}", id1, "x");
        let name2 = format!("{}_{}", id2, "x");
        assert!(env.environment().get_variable(&name1).is_some());
        assert!(env.environment().get_variable(&name2).is_some());
    }

    #[test]
    fn test_multiple_propagate_calls_with_same_id() {
        // Arrange
        let mut env = create_env();
        let var = Variable::new("x", 10.0);
        let id = env.register(&var);

        // Update environment value
        let var_name = format!("{}_{}", id, "x");
        let _ = env.environment.update_variable(&var_name, 50.0);

        let mut external_var1 = Variable::new("x", 0.0);
        let mut external_var2 = Variable::new("x", 0.0);

        // Act
        let _ = env.propagate(&id, &mut external_var1);
        let _ = env.propagate(&id, &mut external_var2);

        // Assert
        assert_relative_eq!(external_var1.value(), 50.0);
        assert_relative_eq!(external_var2.value(), 50.0);
    }
}
