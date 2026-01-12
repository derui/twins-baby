use crate::{
    environment::Environment,
    equation::{Equation, EquationError},
    variable::Variable,
};

/// Represents a constant equation that always evaluates to a fixed value.
#[derive(Debug, Clone)]
pub(crate) struct ConstantEquation {
    value: f32,
}

impl Equation for f32 {
    fn evaluate(&self, _env: &Environment) -> Result<f32, EquationError> {
        Ok(*self)
    }

    fn is_variable_related(&self, _variable: &Variable) -> bool {
        false
    }
}

impl std::fmt::Display for ConstantEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<f32> for Box<dyn Equation> {
    fn from(value: f32) -> Self {
        Box::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_evaluate_returns_constant_value_with_empty_environment() {
        // arrange
        let equation = 100.0;
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 100.0);
    }

    #[test]
    fn test_evaluate_returns_constant_value_with_populated_environment() {
        // arrange
        let equation = 25.0;
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 25.0);
    }

    #[test]
    fn test_evaluate_returns_same_value_on_multiple_calls() {
        // arrange
        let equation = 7.5;
        let env = Environment::empty();

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
    fn test_evaluate_with_different_environments_returns_same_value() {
        // arrange
        let equation = 50.0;
        let env1 = Environment::empty();
        let env2 = Environment::from_variables(vec![Variable::new("x", 100.0)]);
        let env3 = Environment::from_variables(vec![
            Variable::new("a", 1.0),
            Variable::new("b", 2.0),
            Variable::new("c", 3.0),
        ]);

        // act
        let result1 = equation.evaluate(&env1);
        let result2 = equation.evaluate(&env2);
        let result3 = equation.evaluate(&env3);

        // assert
        assert_eq!(result1.unwrap(), 50.0);
        assert_eq!(result2.unwrap(), 50.0);
        assert_eq!(result3.unwrap(), 50.0);
    }

    #[test]
    fn test_is_variable_related_returns_false_for_any_variable() {
        // arrange
        let equation = 42.0;
        let var_x = Variable::new("x", 0.0);

        // act
        let result = equation.is_variable_related(&var_x);

        // assert
        assert_eq!(result, false);
    }
}
