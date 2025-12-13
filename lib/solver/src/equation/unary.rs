use crate::{
    environment::Environment,
    equation::{Equation, EquationError},
};

/// Implementation of equation
#[derive(Debug, Clone)]
pub(crate) struct UnaryEquation {
    /// The factor of thisequesion
    factor: Box<dyn Equation>,

    /// Factorized expression
    expression: Box<dyn Equation>,
}

impl Equation for UnaryEquation {
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError> {
        let factor = self.factor.evaluate(env)?;

        let expression = self.expression.evaluate(env)?;

        Ok(factor * expression)
    }
}

impl std::fmt::Display for UnaryEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.factor, self.expression)
    }
}

impl UnaryEquation {
    pub(crate) fn new(factor: &impl Equation, expression: &impl Equation) -> Self {
        Self {
            factor: factor.clone_box(),
            expression: expression.clone_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equation::constant::ConstantEquation;
    use crate::equation::variable::VariableEquation;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_creates_unary_equation_with_constant_factor_and_expression() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression = ConstantEquation::new(3.0);

        // act
        let equation = UnaryEquation::new(&factor, &expression);

        // assert
        let env = Environment::empty();
        assert_eq!(equation.evaluate(&env).unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_multiplies_factor_and_expression() {
        // arrange
        let factor = ConstantEquation::new(5.0);
        let expression = ConstantEquation::new(4.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_with_variable_factor() {
        // arrange
        let factor = VariableEquation::new("x");
        let expression = ConstantEquation::new(3.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let var = Variable::new("x", 7.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 21.0);
    }

    #[test]
    fn test_evaluate_with_variable_expression() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression = VariableEquation::new("y");
        let equation = UnaryEquation::new(&factor, &expression);
        let var = Variable::new("y", 8.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 16.0);
    }

    #[test]
    fn test_evaluate_with_both_variables() {
        // arrange
        let factor = VariableEquation::new("a");
        let expression = VariableEquation::new("b");
        let equation = UnaryEquation::new(&factor, &expression);
        let var1 = Variable::new("a", 3.0);
        let var2 = Variable::new("b", 4.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 12.0);
    }

    #[test]
    fn test_evaluate_returns_error_when_factor_variable_not_in_environment() {
        // arrange
        let factor = VariableEquation::new("x");
        let expression = ConstantEquation::new(5.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec!["x".to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[test]
    fn test_evaluate_returns_error_when_expression_variable_not_in_environment() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression = VariableEquation::new("y");
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec!["y".to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[test]
    fn test_evaluate_with_zero_factor() {
        // arrange
        let factor = ConstantEquation::new(0.0);
        let expression = ConstantEquation::new(100.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_with_zero_expression() {
        // arrange
        let factor = ConstantEquation::new(50.0);
        let expression = ConstantEquation::new(0.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_with_one_as_factor() {
        // arrange
        let factor = ConstantEquation::new(1.0);
        let expression = ConstantEquation::new(42.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_with_nested_unary_equations() {
        // arrange
        let inner_factor = ConstantEquation::new(2.0);
        let inner_expression = ConstantEquation::new(3.0);
        let inner_equation = UnaryEquation::new(&inner_factor, &inner_expression);

        let outer_factor = ConstantEquation::new(4.0);
        let outer_equation = UnaryEquation::new(&outer_factor, &inner_equation);
        let env = Environment::empty();

        // act
        let result = outer_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 24.0); // 4 * (2 * 3) = 24
    }

    #[test]
    fn test_clone_creates_independent_copy() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression = ConstantEquation::new(5.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let env = Environment::empty();

        // act
        let cloned_equation = equation.clone();
        let result1 = equation.evaluate(&env);
        let result2 = cloned_equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 10.0);
        assert_eq!(result2.unwrap(), 10.0);
    }

    #[test]
    fn test_evaluate_multiple_times_returns_consistent_results() {
        // arrange
        let factor = VariableEquation::new("x");
        let expression = ConstantEquation::new(3.0);
        let equation = UnaryEquation::new(&factor, &expression);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        let result2 = equation.evaluate(&env);
        let result3 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 15.0);
        assert_eq!(result2.unwrap(), 15.0);
        assert_eq!(result3.unwrap(), 15.0);
    }
}
