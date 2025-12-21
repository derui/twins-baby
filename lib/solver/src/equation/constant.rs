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

impl Equation for ConstantEquation {
    fn evaluate(&self, _env: &Environment) -> Result<f32, EquationError> {
        Ok(self.value)
    }

    fn derive(&self, _variable: &Variable) -> Option<Box<dyn Equation>> {
        // constant can not derive
        None
    }
}

impl std::fmt::Display for ConstantEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ConstantEquation {
    /// Get a new constant equation with value
    pub(crate) fn new(value: f32) -> Self {
        ConstantEquation { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_creates_constant_equation_with_positive_value() {
        // arrange

        // act
        let equation = ConstantEquation::new(42.5);

        // assert
        assert_eq!(equation.value, 42.5);
    }

    #[test]
    fn test_new_creates_constant_equation_with_negative_value() {
        // arrange

        // act
        let equation = ConstantEquation::new(-15.3);

        // assert
        assert_eq!(equation.value, -15.3);
    }

    #[test]
    fn test_new_creates_constant_equation_with_zero() {
        // arrange

        // act
        let equation = ConstantEquation::new(0.0);

        // assert
        assert_eq!(equation.value, 0.0);
    }

    #[test]
    fn test_evaluate_returns_constant_value_with_empty_environment() {
        // arrange
        let equation = ConstantEquation::new(100.0);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 100.0);
    }

    #[test]
    fn test_evaluate_returns_constant_value_with_populated_environment() {
        // arrange
        let equation = ConstantEquation::new(25.0);
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
        let equation = ConstantEquation::new(7.5);
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
        let equation = ConstantEquation::new(50.0);
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
    fn test_derive_returns_none_regardless_of_variable_name() {
        // arrange
        let equation = ConstantEquation::new(100.0);
        let var_x = Variable::new("x", 0.0);
        let var_y = Variable::new("y", 0.0);
        let var_abc = Variable::new("abc", 0.0);

        // act
        let result_x = equation.derive(&var_x);
        let result_y = equation.derive(&var_y);
        let result_abc = equation.derive(&var_abc);

        // assert
        assert_eq!(result_x, None);
        assert_eq!(result_y, None);
        assert_eq!(result_abc, None);
    }

    #[test]
    fn test_derive_returns_consistent_result_on_multiple_calls() {
        // arrange
        let equation = ConstantEquation::new(25.0);
        let variable = Variable::new("x", 0.0);

        // act
        let result1 = equation.derive(&variable);
        let result2 = equation.derive(&variable);
        let result3 = equation.derive(&variable);

        // assert
        assert_eq!(result1, None);
        assert_eq!(result2, None);
        assert_eq!(result3, None);
    }
}
