use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow};
use solver::{environment::Environment, variable::Variable};

use crate::id::GenerateId;

/// Internal id for manage variable in sketch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariableId(u64);

impl Display for VariableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var{}", self.0)
    }
}

impl From<u64> for VariableId {
    fn from(value: u64) -> Self {
        VariableId(value)
    }
}

impl From<VariableId> for u64 {
    fn from(value: VariableId) -> Self {
        value.0
    }
}

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

impl<T: GenerateId<VariableId>> PropagatableEnv<T> {
    /// Make a new [PropagatableEnv] with a [GenerateId] trait
    pub fn from_id_gen(id_gen: T) -> Self {
        PropagatableEnv {
            _id_gen: id_gen,
            id_map: HashMap::new(),
            environment: Environment::empty(),
        }
    }

    /// Register a variable to the env.
    pub fn register(&mut self, var: &Variable) -> VariableId {
        let id = self._id_gen.generate();
        let new_name = format!("{}_{}", id, var.name());

        // add variable with new name. the name is global in a sketch
        self.environment.add_variable(var.with_name(&new_name));
        self.id_map.insert(id, new_name);

        id
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
