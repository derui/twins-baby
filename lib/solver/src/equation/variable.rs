use crate::equation::Equation;

/// Representation of a variable equation in a mathematical model.
#[derive(Clone)]
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

    fn clone_box(&self) -> Box<dyn Equation> {
        Box::new(self.clone())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_creates_variable_equation_with_given_name() {
        // arrange

        // act
        let equation = VariableEquation::new("x");

        // assert
        assert_eq!(equation.name, "x");
    }

    #[test]
    fn test_new_creates_variable_equation_with_long_name() {
        // arrange

        // act
        let equation = VariableEquation::new("variable_name");

        // assert
        assert_eq!(equation.name, "variable_name");
    }

    #[test]
    fn test_evaluate_returns_variable_value_from_environment() {
        // arrange
        let equation = VariableEquation::new("x");
        let var = Variable::new("x", 42.5);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 42.5);
    }

    #[test]
    fn test_evaluate_returns_error_when_variable_not_in_environment() {
        // arrange
        let equation = VariableEquation::new("x");
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(super::super::EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec!["x".to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[test]
    fn test_evaluate_returns_error_when_different_variable_in_environment() {
        // arrange
        let equation = VariableEquation::new("x");
        let var = Variable::new("y", 10.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(super::super::EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec!["x".to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[test]
    fn test_evaluate_returns_correct_value_with_multiple_variables_in_environment() {
        // arrange
        let equation = VariableEquation::new("y");
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        let var3 = Variable::new("z", 30.0);
        let env = Environment::from_variables(vec![var1, var2, var3]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_multiple_times_returns_same_value() {
        // arrange
        let equation = VariableEquation::new("x");
        let var = Variable::new("x", 7.5);
        let env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        let result2 = equation.evaluate(&env);
        let result3 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 7.5);
        assert_eq!(result2.unwrap(), 7.5);
        assert_eq!(result3.unwrap(), 7.5);
    }

    #[test]
    fn test_evaluate_reflects_updated_environment_value() {
        // arrange
        let equation = VariableEquation::new("x");
        let var = Variable::new("x", 10.0);
        let mut env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        env.update_variable("x", 25.0).unwrap();
        let result2 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 10.0);
        assert_eq!(result2.unwrap(), 25.0);
    }
}
