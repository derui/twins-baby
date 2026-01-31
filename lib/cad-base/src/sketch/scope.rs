use std::collections::HashMap;

use solver::{environment::Environment, variable::Variable};

use crate::id::{IdStore, VariableId};

#[derive(Debug, Clone)]
pub struct VariableScope {
    id_gen: IdStore<VariableId>,

    variables: HashMap<VariableId, Variable>,
}

/// Single responsibility trait to register a variable
impl VariableScope {
    pub fn new() -> Self {
        Self {
            id_gen: IdStore::of(),
            variables: HashMap::new(),
        }
    }

    /// Get a new environment. All variables of [Environment] does not be referenced from self.
    pub fn to_environment(&self) -> Environment {
        Environment::from_variables(self.variables.values().cloned().collect())
    }

    /// Get mapping name and variable id. This is useful of between environment.
    pub(crate) fn to_id_name_map(&self) -> HashMap<String, VariableId> {
        let mut map: HashMap<String, VariableId> = HashMap::new();

        for (k, v) in &self.variables {
            map.insert((*v.name).clone(), *k);
        }

        map
    }

    /// Register a variable.
    pub fn register(&mut self, value: f32) -> VariableId {
        let id = self.id_gen.generate();
        let v = Variable::new(&id.to_string(), value);

        self.variables.insert(id, v);
        id
    }

    /// De-register a variable if it registered
    pub fn deregister(&mut self, id: &VariableId) -> Option<Variable> {
        self.variables.remove(id)
    }

    /// Get a variable by id
    pub fn get(&self, id: &VariableId) -> Option<&Variable> {
        self.variables.get(id)
    }

    /// Get a mutable variable by id
    pub fn get_mut(&mut self, id: &VariableId) -> Option<&mut Variable> {
        self.variables.get_mut(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn register_creates_variable_with_given_value() {
        // Arrange
        let mut scope = VariableScope::new();
        let value = 42.0;

        // Act
        let id = scope.register(value);
        let variable = scope.get(&id);

        // Assert
        assert!(variable.is_some());
        assert_relative_eq!(variable.unwrap().value, value);
    }

    #[test]
    fn register_generates_unique_ids_for_multiple_variables() {
        // Arrange
        let mut scope = VariableScope::new();

        // Act
        let id1 = scope.register(10.0);
        let id2 = scope.register(20.0);
        let id3 = scope.register(30.0);

        // Assert
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn register_stores_different_values_independently() {
        // Arrange
        let mut scope = VariableScope::new();

        // Act
        let id1 = scope.register(1.5);
        let id2 = scope.register(2.5);
        let id3 = scope.register(3.5);

        // Assert
        assert_relative_eq!(scope.get(&id1).unwrap().value, 1.5);
        assert_relative_eq!(scope.get(&id2).unwrap().value, 2.5);
        assert_relative_eq!(scope.get(&id3).unwrap().value, 3.5);
    }

    #[test]
    fn get_returns_none_for_non_existent_id() {
        // Arrange
        let scope = VariableScope::new();
        let non_existent_id = VariableId::from(999);

        // Act
        let result = scope.get(&non_existent_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn deregister_removes_variable_and_returns_it() {
        // Arrange
        let mut scope = VariableScope::new();
        let id = scope.register(100.0);

        // Act
        let removed = scope.deregister(&id);

        // Assert
        assert!(removed.is_some());
        assert_relative_eq!(removed.unwrap().value, 100.0);
        assert!(scope.get(&id).is_none());
    }

    #[test]
    fn deregister_returns_none_for_non_existent_id() {
        // Arrange
        let mut scope = VariableScope::new();
        let non_existent_id = VariableId::from(999);

        // Act
        let result = scope.deregister(&non_existent_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn deregister_does_not_affect_other_variables() {
        // Arrange
        let mut scope = VariableScope::new();
        let id1 = scope.register(1.0);
        let id2 = scope.register(2.0);
        let id3 = scope.register(3.0);

        // Act
        scope.deregister(&id2);

        // Assert
        assert!(scope.get(&id1).is_some());
        assert!(scope.get(&id2).is_none());
        assert!(scope.get(&id3).is_some());
    }

    #[test]
    fn get_mut_allows_modifying_variable_value() {
        // Arrange
        let mut scope = VariableScope::new();
        let id = scope.register(5.0);

        // Act
        let variable = scope.get_mut(&id).unwrap();
        variable.value = 10.0;

        // Assert
        assert_relative_eq!(scope.get(&id).unwrap().value, 10.0);
    }

    #[test]
    fn get_mut_returns_none_for_non_existent_id() {
        // Arrange
        let mut scope = VariableScope::new();
        let non_existent_id = VariableId::from(999);

        // Act
        let result = scope.get_mut(&non_existent_id);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn to_environment_creates_environment_with_all_variables() {
        // Arrange
        let mut scope = VariableScope::new();
        scope.register(1.0);
        scope.register(2.0);
        scope.register(3.0);

        // Act
        let env = scope.to_environment();

        // Assert
        assert_eq!(env.variables().len(), 3);
    }

    #[test]
    fn to_environment_reflects_current_state_after_deregistration() {
        // Arrange
        let mut scope = VariableScope::new();
        let id1 = scope.register(1.0);
        let id2 = scope.register(2.0);
        scope.deregister(&id1);

        // Act
        let env = scope.to_environment();

        // Assert
        assert_eq!(env.variables().len(), 1);
        // Verify the remaining variable has the correct value
        let remaining_var = env.get(&id2.to_string());
        assert!(remaining_var.is_some());
        assert_relative_eq!(remaining_var.unwrap().value, 2.0);
    }

    #[test]
    fn to_environment_with_empty_scope_creates_empty_environment() {
        // Arrange
        let scope = VariableScope::new();

        // Act
        let env = scope.to_environment();

        // Assert
        assert_eq!(env.variables().len(), 0);
    }

    #[test]
    fn register_handles_special_float_values() {
        // Arrange
        let mut scope = VariableScope::new();

        // Act
        let id_zero = scope.register(0.0);
        let id_negative = scope.register(-42.5);
        let id_large = scope.register(1e10);
        let id_small = scope.register(1e-10);

        // Assert
        assert_relative_eq!(scope.get(&id_zero).unwrap().value, 0.0);
        assert_relative_eq!(scope.get(&id_negative).unwrap().value, -42.5);
        assert_relative_eq!(scope.get(&id_large).unwrap().value, 1e10);
        assert_relative_eq!(scope.get(&id_small).unwrap().value, 1e-10);
    }

    #[test]
    fn to_id_name_map_returns_empty_for_empty_scope() {
        // Arrange
        let scope = VariableScope::new();

        // Act
        let map = scope.to_id_name_map();

        // Assert
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn to_id_name_map_maps_variable_names_to_ids() {
        // Arrange
        let mut scope = VariableScope::new();
        let id1 = scope.register(1.0);
        let id2 = scope.register(2.0);

        // Act
        let map = scope.to_id_name_map();

        // Assert
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1.to_string()), Some(&id1));
        assert_eq!(map.get(&id2.to_string()), Some(&id2));
    }

    #[test]
    fn to_id_name_map_reflects_current_state_after_deregistration() {
        // Arrange
        let mut scope = VariableScope::new();
        let id1 = scope.register(1.0);
        let id2 = scope.register(2.0);
        scope.deregister(&id1);

        // Act
        let map = scope.to_id_name_map();

        // Assert
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&id2.to_string()), Some(&id2));
        assert_eq!(map.get(&id1.to_string()), None);
    }
}
