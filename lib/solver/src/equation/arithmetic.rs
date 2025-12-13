use crate::{environment::Environment, equation::Equation, variable::Variable};

use super::EquationError;

/// Operator of arithmetic
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Implementation of arithmetic equation
#[derive(Debug, Clone)]
pub(crate) struct ArithmeticEquation {
    operator: Operator,
    first: Box<dyn Equation>,
    second: Box<dyn Equation>,
}

impl Equation for ArithmeticEquation {
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError> {
        let first = self.first.evaluate(env)?;
        let second = self.second.evaluate(env)?;

        let ret = match self.operator {
            Operator::Add => first + second,
            Operator::Subtract => first - second,
            Operator::Multiply => first * second,
            Operator::Divide => first / second,
        };

        Ok(ret)
    }

    fn derive(&self, variable: &Variable) -> Option<Box<dyn Equation>> {
        let first = self.first.derive(variable);
        let second = self.second.derive(variable);

        match (first, second) {
            (None, None) => None,
            (None, Some(e)) => Some(e),
            (Some(e), None) => Some(e),
            (Some(e1), Some(e2)) => {
                Some(Box::new(ArithmeticEquation::new(self.operator, &*e1, &*e2)))
            }
        }
    }
}

impl std::fmt::Display for ArithmeticEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.operator {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}{}{}", self.first, op, self.second)
    }
}

impl ArithmeticEquation {
    /// Create a new arithmetic equation with the given operator and operands.
    ///
    /// # Arguments
    /// * `operator` - The arithmetic operator
    /// * `first` - The first operand equation
    /// * `second` - The second operand equation
    ///
    /// # Returns
    /// A new instance of `ArithmeticEquation`
    pub(crate) fn new(operator: Operator, first: &dyn Equation, second: &dyn Equation) -> Self {
        Self {
            operator,
            first: first.clone_box(),
            second: second.clone_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::equation::constant::ConstantEquation;
    use crate::equation::monomial::MonomialEquation;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(Operator::Add, 10.0, 7.0, 17.0)]
    #[case(Operator::Subtract, 15.0, 8.0, 7.0)]
    #[case(Operator::Multiply, 6.0, 4.0, 24.0)]
    #[case(Operator::Divide, 20.0, 4.0, 5.0)]
    fn test_evaluate_with_constants(
        #[case] operator: Operator,
        #[case] first_val: f32,
        #[case] second_val: f32,
        #[case] expected: f32,
    ) {
        // arrange
        let first = ConstantEquation::new(first_val);
        let second = ConstantEquation::new(second_val);
        let equation = ArithmeticEquation::new(operator, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(Operator::Add, "x", 12.0, "y", 8.0, 20.0)]
    #[case(Operator::Subtract, "a", 25.0, "b", 10.0, 15.0)]
    #[case(Operator::Multiply, "x", 7.0, "y", 3.0, 21.0)]
    #[case(Operator::Divide, "numerator", 30.0, "denominator", 6.0, 5.0)]
    fn test_evaluate_with_variables(
        #[case] operator: Operator,
        #[case] var1_name: &str,
        #[case] var1_val: f32,
        #[case] var2_name: &str,
        #[case] var2_val: f32,
        #[case] expected: f32,
    ) {
        // arrange
        let first = MonomialEquation::new(1.0, var1_name, 1);
        let second = MonomialEquation::new(1.0, var2_name, 1);
        let equation = ArithmeticEquation::new(operator, &first, &second);
        let var1 = Variable::new(var1_name, var1_val);
        let var2 = Variable::new(var2_name, var2_val);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_evaluate_mixed_constant_and_variable() {
        // arrange
        let first = ConstantEquation::new(10.0);
        let second = MonomialEquation::new(1.0, "x", 1);
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 15.0);
    }

    #[rstest]
    #[case("x", true)]
    #[case("y", false)]
    fn test_evaluate_returns_error_when_variable_not_in_environment(
        #[case] var_name: &str,
        #[case] first_is_variable: bool,
    ) {
        // arrange
        let first: Box<dyn Equation> = if first_is_variable {
            Box::new(MonomialEquation::new(1.0, var_name, 1))
        } else {
            Box::new(ConstantEquation::new(10.0))
        };
        let second: Box<dyn Equation> = if first_is_variable {
            Box::new(ConstantEquation::new(5.0))
        } else {
            Box::new(MonomialEquation::new(1.0, var_name, 1))
        };
        let equation = ArithmeticEquation::new(Operator::Add, &*first, &*second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(super::super::EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec![var_name.to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[rstest]
    #[case(Operator::Add, 42.0, 0.0, 42.0)]
    #[case(Operator::Subtract, 50.0, 0.0, 50.0)]
    #[case(Operator::Multiply, 100.0, 0.0, 0.0)]
    #[case(Operator::Multiply, 75.0, 1.0, 75.0)]
    #[case(Operator::Divide, 99.0, 1.0, 99.0)]
    fn test_evaluate_with_identity_elements(
        #[case] operator: Operator,
        #[case] first_val: f32,
        #[case] second_val: f32,
        #[case] expected: f32,
    ) {
        // arrange
        let first = ConstantEquation::new(first_val);
        let second = ConstantEquation::new(second_val);
        let equation = ArithmeticEquation::new(operator, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_evaluate_with_nested_arithmetic_equations() {
        // arrange
        let inner_first = ConstantEquation::new(3.0);
        let inner_second = ConstantEquation::new(4.0);
        let inner_equation = ArithmeticEquation::new(Operator::Add, &inner_first, &inner_second);

        let outer_second = ConstantEquation::new(2.0);
        let outer_equation =
            ArithmeticEquation::new(Operator::Multiply, &inner_equation, &outer_second);
        let env = Environment::empty();

        // act
        let result = outer_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 14.0); // (3 + 4) * 2 = 14
    }

    #[test]
    fn test_evaluate_complex_nested_expression() {
        // arrange
        // ((10 + 5) * 2) - (6 / 3) = (15 * 2) - 2 = 30 - 2 = 28
        let add_left = ConstantEquation::new(10.0);
        let add_right = ConstantEquation::new(5.0);
        let add_equation = ArithmeticEquation::new(Operator::Add, &add_left, &add_right);

        let mul_right = ConstantEquation::new(2.0);
        let mul_equation = ArithmeticEquation::new(Operator::Multiply, &add_equation, &mul_right);

        let div_left = ConstantEquation::new(6.0);
        let div_right = ConstantEquation::new(3.0);
        let div_equation = ArithmeticEquation::new(Operator::Divide, &div_left, &div_right);

        let final_equation =
            ArithmeticEquation::new(Operator::Subtract, &mul_equation, &div_equation);
        let env = Environment::empty();

        // act
        let result = final_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 28.0);
    }

    mod derive_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case(Operator::Add, 5.0, 3.0)]
        #[case(Operator::Subtract, 5.0, 3.0)]
        #[case(Operator::Multiply, 2.0, 7.0)]
        #[case(Operator::Divide, 10.0, 2.0)]
        fn test_derive_with_both_constants_returns_none(
            #[case] operator: Operator,
            #[case] first_val: f32,
            #[case] second_val: f32,
        ) {
            // arrange
            let first = ConstantEquation::new(first_val);
            let second = ConstantEquation::new(second_val);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert_eq!(result, None);
        }

        #[rstest]
        #[case(Operator::Add, "2")]
        #[case(Operator::Subtract, "2")]
        #[case(Operator::Multiply, "2")]
        #[case(Operator::Divide, "2")]
        fn test_derive_with_first_variable_second_constant(
            #[case] operator: Operator,
            #[case] expected: &str,
        ) {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1);
            let second = ConstantEquation::new(3.0);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert!(result.is_some());
            assert_eq!(format!("{}", result.unwrap()), expected);
        }

        #[rstest]
        #[case(Operator::Add, "3")]
        #[case(Operator::Subtract, "3")]
        #[case(Operator::Multiply, "3")]
        #[case(Operator::Divide, "3")]
        fn test_derive_with_first_constant_second_variable(
            #[case] operator: Operator,
            #[case] expected: &str,
        ) {
            // arrange
            let first = ConstantEquation::new(5.0);
            let second = MonomialEquation::new(3.0, "y", 1);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new("y", 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert!(result.is_some());
            assert_eq!(format!("{}", result.unwrap()), expected);
        }

        #[rstest]
        #[case(Operator::Add, "2+3")]
        #[case(Operator::Subtract, "2-3")]
        #[case(Operator::Multiply, "2*3")]
        #[case(Operator::Divide, "2/3")]
        fn test_derive_with_both_same_variables(
            #[case] operator: Operator,
            #[case] expected: &str,
        ) {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1);
            let second = MonomialEquation::new(3.0, "x", 1);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert!(result.is_some());
            assert_eq!(format!("{}", result.unwrap()), expected);
        }

        #[rstest]
        #[case(Operator::Add)]
        #[case(Operator::Subtract)]
        #[case(Operator::Multiply)]
        #[case(Operator::Divide)]
        fn test_derive_with_both_same_variables_different_derive_var_returns_none(
            #[case] operator: Operator,
        ) {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1);
            let second = MonomialEquation::new(3.0, "x", 1);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new("y", 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert_eq!(result, None);
        }

        #[rstest]
        #[case(Operator::Add, "x", "2")]
        #[case(Operator::Subtract, "x", "2")]
        #[case(Operator::Multiply, "x", "2")]
        #[case(Operator::Divide, "x", "2")]
        #[case(Operator::Add, "y", "3")]
        #[case(Operator::Subtract, "y", "3")]
        #[case(Operator::Multiply, "y", "3")]
        #[case(Operator::Divide, "y", "3")]
        fn test_derive_with_different_variables(
            #[case] operator: Operator,
            #[case] derive_var: &str,
            #[case] expected: &str,
        ) {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1);
            let second = MonomialEquation::new(3.0, "y", 1);
            let equation = ArithmeticEquation::new(operator, &first, &second);
            let variable = Variable::new(derive_var, 0.0);

            // act
            let result = equation.derive(&variable);

            // assert
            assert!(result.is_some());
            assert_eq!(format!("{}", result.unwrap()), expected);
        }

        #[test]
        fn test_derive_with_nested_equations() {
            // arrange
            // (x + 2) * 3, derive with respect to x
            let inner_first = MonomialEquation::new(1.0, "x", 1);
            let inner_second = ConstantEquation::new(2.0);
            let inner_equation =
                ArithmeticEquation::new(Operator::Add, &inner_first, &inner_second);

            let outer_second = ConstantEquation::new(3.0);
            let outer_equation =
                ArithmeticEquation::new(Operator::Multiply, &inner_equation, &outer_second);
            let variable = Variable::new("x", 0.0);

            // act
            let result = outer_equation.derive(&variable);

            // assert
            assert!(result.is_some());
            assert_eq!(format!("{}", result.unwrap()), "1");
        }
    }
}
