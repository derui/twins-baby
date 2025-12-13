use crate::{
    environment::Environment,
    equation::{Equation, EquationError, constant::ConstantEquation},
    variable::Variable,
};

/// Implementation of monomial equation
#[derive(Debug, Clone)]
pub(crate) struct MonomialEquation {
    /// The factor of thisequesion
    factor: f32,

    /// Factorized expression
    variable: String,

    /// Exponent of the variable
    exponent: i32,
}

impl Equation for MonomialEquation {
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError> {
        let variable = env
            .get_variable(&self.variable)
            .ok_or_else(|| EquationError::NoVariableInEnvironment(vec![self.variable.clone()]))?;

        Ok(self.factor * variable.value().powf(self.exponent as f32))
    }

    fn derive(&self, variable: &Variable) -> Option<Box<dyn Equation>> {
        if self.variable == variable.name() {
            match self.exponent {
                1 => {
                    // derivative of x^1 is 1
                    Some(Box::new(ConstantEquation::new(self.factor)))
                }
                0 => {
                    // derivative of x^0 is 0
                    None
                }
                _ => {
                    // power rule: d/dx [x^n] = n*x^(n-1)
                    let new_exponent = self.exponent - 1;
                    let new_variable_equation = MonomialEquation::new(
                        self.factor * (self.exponent as f32),
                        &self.variable,
                        new_exponent,
                    );
                    Some(Box::new(new_variable_equation))
                }
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for MonomialEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}^{}", self.factor, self.variable, self.exponent)
    }
}

impl MonomialEquation {
    pub(crate) fn new(factor: f32, variable: &str, exponent: i32) -> Self {
        Self {
            factor,
            variable: variable.to_owned(),
            exponent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_creates_monomial_equation_with_factor_variable_and_exponent() {
        // arrange
        let equation = MonomialEquation::new(2.0, "x", 1);
        let var = Variable::new("x", 3.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_multiplies_factor_and_variable_power() {
        // arrange
        let equation = MonomialEquation::new(5.0, "x", 1);
        let var = Variable::new("x", 4.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_with_exponent_greater_than_one() {
        // arrange
        let equation = MonomialEquation::new(3.0, "x", 2);
        let var = Variable::new("x", 4.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 48.0); // 3 * 4^2 = 3 * 16 = 48
    }

    #[test]
    fn test_evaluate_with_different_variable_name() {
        // arrange
        let equation = MonomialEquation::new(2.0, "y", 1);
        let var = Variable::new("y", 8.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 16.0);
    }

    #[test]
    fn test_evaluate_with_exponent_three() {
        // arrange
        let equation = MonomialEquation::new(2.0, "x", 3);
        let var = Variable::new("x", 3.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 54.0); // 2 * 3^3 = 2 * 27 = 54
    }

    #[test]
    fn test_evaluate_returns_error_when_variable_not_in_environment() {
        // arrange
        let equation = MonomialEquation::new(2.0, "x", 2);
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
    fn test_evaluate_returns_error_when_different_variable_not_in_environment() {
        // arrange
        let equation = MonomialEquation::new(2.0, "y", 1);
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
        let equation = MonomialEquation::new(0.0, "x", 2);
        let var = Variable::new("x", 100.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_with_zero_exponent() {
        // arrange
        let equation = MonomialEquation::new(50.0, "x", 0);
        let var = Variable::new("x", 10.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 50.0); // 50 * 10^0 = 50 * 1 = 50
    }

    #[test]
    fn test_evaluate_with_one_as_factor() {
        // arrange
        let equation = MonomialEquation::new(1.0, "x", 1);
        let var = Variable::new("x", 42.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_with_negative_factor() {
        // arrange
        let equation = MonomialEquation::new(-2.0, "x", 2);
        let var = Variable::new("x", 3.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), -18.0); // -2 * 3^2 = -2 * 9 = -18
    }

    #[test]
    fn test_clone_creates_independent_copy() {
        // arrange
        let equation = MonomialEquation::new(2.0, "x", 1);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

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
        let equation = MonomialEquation::new(3.0, "x", 2);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        let result2 = equation.evaluate(&env);
        let result3 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 75.0); // 3 * 5^2 = 3 * 25 = 75
        assert_eq!(result2.unwrap(), 75.0);
        assert_eq!(result3.unwrap(), 75.0);
    }
}
