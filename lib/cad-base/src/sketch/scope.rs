use std::collections::HashMap;

use solver::{EquationId, environment::Environment, equation::Equation, variable::Variable};

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

    /// Get new variable. All variables of [Environment] does not be referenced from self.
    pub fn to_environment(&self) -> Environment {
        Environment::from_variables(self.variables.values().cloned().collect())
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
