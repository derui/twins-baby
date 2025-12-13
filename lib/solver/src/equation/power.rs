use crate::{
    environment::Environment,
    equation::{Equation, EquationError, constant::ConstantEquation},
};

/// Implementation of power equation (base^exponent)
#[derive(Debug, Clone)]
pub(crate) struct PowerEquation {
    /// The base of the power expression
    base: Box<dyn Equation>,

    /// The exponent of the power expression
    exponent: Box<dyn Equation>,
}

impl Equation for PowerEquation {
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError> {
        let base = self.base.evaluate(env)?;
        let exponent = self.exponent.evaluate(env)?;

        Ok(base.powf(exponent))
    }

    fn clone_box(&self) -> Box<dyn Equation> {
        Box::new(self.clone())
    }
}

impl PowerEquation {
    /// Create a new power equation with the given base and exponent.
    ///
    /// # Arguments
    /// * `base` - The base equation
    /// * `exponent` - The exponent equation
    ///
    /// # Returns
    /// A new instance of `PowerEquation`
    pub(crate) fn new(base: &impl Equation, exponent: &impl Equation) -> Self {
        Self {
            base: base.clone_box(),
            exponent: exponent.clone_box(),
        }
    }

    /// Create a new power equation with the given base and raw exponent
    pub(crate) fn new_with_raw_exponent(base: &impl Equation, exponent: f32) -> Self {
        Self {
            base: base.clone_box(),
            exponent: Box::new(ConstantEquation::new(exponent)),
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
    fn test_new_creates_power_equation_with_constant_base_and_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);
        let exponent = ConstantEquation::new(3.0);

        // act
        let equation = PowerEquation::new(&base, &exponent);

        // assert
        let env = Environment::empty();
        assert_eq!(equation.evaluate(&env).unwrap(), 8.0);
    }

    #[test]
    fn test_new_creates_power_equation_with_constant_base_and_raw_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);

        // act
        let equation = PowerEquation::new_with_raw_exponent(&base, 3.);

        // assert
        let env = Environment::empty();
        assert_eq!(equation.evaluate(&env).unwrap(), 8.0);
    }

    #[test]
    fn test_evaluate_power_with_constants() {
        // arrange
        let base = ConstantEquation::new(5.0);
        let exponent = ConstantEquation::new(2.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 25.0);
    }

    #[test]
    fn test_evaluate_power_with_variable_base() {
        // arrange
        let base = VariableEquation::new("x");
        let exponent = ConstantEquation::new(3.0);
        let equation = PowerEquation::new(&base, &exponent);
        let var = Variable::new("x", 2.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 8.0);
    }

    #[test]
    fn test_evaluate_power_with_variable_exponent() {
        // arrange
        let base = ConstantEquation::new(3.0);
        let exponent = VariableEquation::new("y");
        let equation = PowerEquation::new(&base, &exponent);
        let var = Variable::new("y", 4.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 81.0);
    }

    #[test]
    fn test_evaluate_power_with_both_variables() {
        // arrange
        let base = VariableEquation::new("a");
        let exponent = VariableEquation::new("b");
        let equation = PowerEquation::new(&base, &exponent);
        let var1 = Variable::new("a", 2.0);
        let var2 = Variable::new("b", 5.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 32.0);
    }

    #[test]
    fn test_evaluate_returns_error_when_base_variable_not_in_environment() {
        // arrange
        let base = VariableEquation::new("x");
        let exponent = ConstantEquation::new(2.0);
        let equation = PowerEquation::new(&base, &exponent);
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
    fn test_evaluate_returns_error_when_exponent_variable_not_in_environment() {
        // arrange
        let base = ConstantEquation::new(3.0);
        let exponent = VariableEquation::new("y");
        let equation = PowerEquation::new(&base, &exponent);
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
    fn test_evaluate_power_with_zero_exponent() {
        // arrange
        let base = ConstantEquation::new(100.0);
        let exponent = ConstantEquation::new(0.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 1.0);
    }

    #[test]
    fn test_evaluate_power_with_one_as_exponent() {
        // arrange
        let base = ConstantEquation::new(42.0);
        let exponent = ConstantEquation::new(1.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_power_with_negative_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);
        let exponent = ConstantEquation::new(-2.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.25);
    }

    #[test]
    fn test_evaluate_power_with_fractional_exponent() {
        // arrange
        let base = ConstantEquation::new(4.0);
        let exponent = ConstantEquation::new(0.5);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_evaluate_power_with_zero_base_and_positive_exponent() {
        // arrange
        let base = ConstantEquation::new(0.0);
        let exponent = ConstantEquation::new(5.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_power_with_one_as_base() {
        // arrange
        let base = ConstantEquation::new(1.0);
        let exponent = ConstantEquation::new(100.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 1.0);
    }

    #[test]
    fn test_evaluate_with_nested_power_equations() {
        // arrange
        let inner_base = ConstantEquation::new(2.0);
        let inner_exponent = ConstantEquation::new(3.0);
        let inner_equation = PowerEquation::new(&inner_base, &inner_exponent);

        let outer_exponent = ConstantEquation::new(2.0);
        let outer_equation = PowerEquation::new(&inner_equation, &outer_exponent);
        let env = Environment::empty();

        // act
        let result = outer_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 64.0); // (2^3)^2 = 8^2 = 64
    }

    #[test]
    fn test_clone_creates_independent_copy() {
        // arrange
        let base = ConstantEquation::new(3.0);
        let exponent = ConstantEquation::new(2.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let cloned_equation = equation.clone();
        let result1 = equation.evaluate(&env);
        let result2 = cloned_equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 9.0);
        assert_eq!(result2.unwrap(), 9.0);
    }

    #[test]
    fn test_evaluate_multiple_times_returns_consistent_results() {
        // arrange
        let base = VariableEquation::new("x");
        let exponent = ConstantEquation::new(2.0);
        let equation = PowerEquation::new(&base, &exponent);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        let result2 = equation.evaluate(&env);
        let result3 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 25.0);
        assert_eq!(result2.unwrap(), 25.0);
        assert_eq!(result3.unwrap(), 25.0);
    }

    #[test]
    fn test_evaluate_large_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);
        let exponent = ConstantEquation::new(10.0);
        let equation = PowerEquation::new(&base, &exponent);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 1024.0);
    }
}
