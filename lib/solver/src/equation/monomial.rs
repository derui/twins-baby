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
                    Some(ConstantEquation::new(self.factor).into())
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
                    Some(new_variable_equation.into())
                }
            }
        } else {
            None
        }
    }

    fn is_variable_related(&self, variable: &Variable) -> bool {
        self.variable == variable.name()
    }
}

impl std::fmt::Display for MonomialEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}^{}", self.factor, self.variable, self.exponent)
    }
}

impl From<MonomialEquation> for Box<dyn Equation> {
    fn from(value: MonomialEquation) -> Self {
        Box::new(value.clone())
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

    mod derive_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case(1.0, "x", 1, "x", Some("1"))]
        #[case(5.0, "x", 1, "x", Some("5"))]
        #[case(1.0, "x", 2, "x", Some("2x^1"))]
        #[case(1.0, "x", 3, "x", Some("3x^2"))]
        #[case(3.0, "x", 2, "x", Some("6x^1"))]
        #[case(2.0, "x", 3, "x", Some("6x^2"))]
        #[case(4.0, "y", 2, "y", Some("8y^1"))]
        fn test_derive_with_power_rule(
            #[case] factor: f32,
            #[case] var_name: &str,
            #[case] exponent: i32,
            #[case] derive_var: &str,
            #[case] expected: Option<&str>,
        ) {
            // arrange
            let equation = MonomialEquation::new(factor, var_name, exponent);
            let variable = Variable::new(derive_var, 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            match expected {
                Some(expected_str) => {
                    assert!(result.is_some());
                    assert_eq!(format!("{}", result.unwrap()), expected_str);
                }
                None => {
                    assert_eq!(result, None);
                }
            }
        }

        #[rstest]
        #[case(5.0, "x", 0, "x")]
        #[case(10.0, "x", 0, "x")]
        #[case(1.0, "y", 0, "y")]
        fn test_derive_with_zero_exponent_returns_none(
            #[case] factor: f32,
            #[case] var_name: &str,
            #[case] exponent: i32,
            #[case] derive_var: &str,
        ) {
            // arrange
            let equation = MonomialEquation::new(factor, var_name, exponent);
            let variable = Variable::new(derive_var, 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert_eq!(result, None);
        }

        #[rstest]
        #[case(2.0, "x", 2, "y")]
        #[case(3.0, "x", 3, "z")]
        #[case(1.0, "a", 1, "b")]
        #[case(5.0, "foo", 2, "bar")]
        fn test_derive_with_different_variable_returns_none(
            #[case] factor: f32,
            #[case] var_name: &str,
            #[case] exponent: i32,
            #[case] derive_var: &str,
        ) {
            // arrange
            let equation = MonomialEquation::new(factor, var_name, exponent);
            let variable = Variable::new(derive_var, 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert_eq!(result, None);
        }
    }

    mod is_variable_related_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_is_variable_related_returns_true_for_matching_variable() {
            // arrange
            let equation = MonomialEquation::new(2.0, "x", 1);
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, true);
        }

        #[test]
        fn test_is_variable_related_returns_false_for_different_variable() {
            // arrange
            let equation = MonomialEquation::new(2.0, "x", 1);
            let variable = Variable::new("y", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, false);
        }

        #[test]
        fn test_is_variable_related_case_sensitive() {
            // arrange
            let equation = MonomialEquation::new(1.0, "Variable", 1);
            let var_lowercase = Variable::new("variable", 0.0);
            let var_correct = Variable::new("Variable", 0.0);

            // act
            let result_lowercase = equation.is_variable_related(&var_lowercase);
            let result_correct = equation.is_variable_related(&var_correct);

            // assert
            assert_eq!(result_lowercase, false);
            assert_eq!(result_correct, true);
        }
    }
}
