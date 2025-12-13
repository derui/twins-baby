use crate::equation::Equation;

/// Representation of a variable equation in a mathematical model.
pub(crate) struct VariableEquation {
    /// name of variable
    name: String,
}

impl Equation for VariableEquation {
    fn evaluate(&self, env: &crate::environment::Environment) -> Result<f32, super::EquationError> {
        match env.get_variable(&self.name) {
            Some(var) => Ok(var.value()),
            None => Err(super::EquationError::NoVariableInEnvironment(vec![
                self.name.clone(),
            ])),
        }
    }
}

impl VariableEquation {
    /// Create a new variable equation with the given name.
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
