use std::cmp::Ordering;
use std::collections::HashSet;

use anyhow::{Result, anyhow};

use crate::{
    environment::Environment,
    equation::{Equation, EquationError, Evaluate},
    variable::Variable,
};

/// Operator of arithmetic
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Operator::Add, Operator::Add) => Some(Ordering::Equal),
            (Operator::Add, Operator::Subtract) => Some(Ordering::Equal),
            (Operator::Add, Operator::Multiply) => Some(Ordering::Less),
            (Operator::Add, Operator::Divide) => Some(Ordering::Less),
            (Operator::Subtract, Operator::Add) => Some(Ordering::Equal),
            (Operator::Subtract, Operator::Subtract) => Some(Ordering::Equal),
            (Operator::Subtract, Operator::Multiply) => Some(Ordering::Less),
            (Operator::Subtract, Operator::Divide) => Some(Ordering::Less),
            (Operator::Multiply, Operator::Add) => Some(Ordering::Greater),
            (Operator::Multiply, Operator::Subtract) => Some(Ordering::Greater),
            (Operator::Multiply, Operator::Multiply) => Some(Ordering::Equal),
            (Operator::Multiply, Operator::Divide) => Some(Ordering::Equal),
            (Operator::Divide, Operator::Add) => Some(Ordering::Greater),
            (Operator::Divide, Operator::Subtract) => Some(Ordering::Greater),
            (Operator::Divide, Operator::Multiply) => Some(Ordering::Equal),
            (Operator::Divide, Operator::Divide) => Some(Ordering::Equal),
        }
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should be success")
    }
}

/// Implementation of arithmetic equation
#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticEquation {
    operator: Operator,
    operands: Vec<Equation>,
}

impl Evaluate for ArithmeticEquation {
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError> {
        let values: Result<Vec<_>, _> = self.operands.iter().map(|e| e.evaluate(env)).collect();
        let values = values?;

        let ret = values.into_iter().reduce(|o1, o2| match self.operator {
            Operator::Add => o1 + o2,
            Operator::Subtract => o1 - o2,
            Operator::Multiply => o1 * o2,
            Operator::Divide => o1 / o2,
        });

        ret.ok_or(EquationError::NoVariableInEnvironment(vec![]))
    }

    fn is_variable_related(&self, variable: &Variable) -> bool {
        self.operands
            .iter()
            .any(|o| o.is_variable_related(variable))
    }

    fn related_variables(&self) -> Vec<String> {
        let mut variables: HashSet<String> = HashSet::new();
        for operand in &self.operands {
            for var in operand.related_variables() {
                variables.insert(var);
            }
        }
        variables.into_iter().collect()
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

        for (i, e) in self.operands.iter().enumerate() {
            if i == self.operands.len() - 1 {
                write!(f, "{}", e)?;
            } else {
                write!(f, "{}{}", e, op)?;
            }
        }
        Ok(())
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
    pub(crate) fn new(operator: Operator, operands: &[Equation]) -> Result<Self> {
        if operands.len() < 2 {
            return Err(anyhow!("Operands must be greater or equal 2"));
        }

        Ok(Self {
            operator,
            operands: Vec::from(operands),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
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
    ) -> Result<()> {
        // arrange
        let first = first_val.into();
        let second = second_val.into();
        let equation = ArithmeticEquation::new(operator, &[first, second])?;
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);

        Ok(())
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
    ) -> Result<()> {
        // arrange
        let first = MonomialEquation::new(1.0, var1_name, 1).into();
        let second = MonomialEquation::new(1.0, var2_name, 1).into();
        let equation = ArithmeticEquation::new(operator, &[first, second])?;
        let var1 = Variable::new(var1_name, var1_val);
        let var2 = Variable::new(var2_name, var2_val);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);
        Ok(())
    }

    #[test]
    fn test_evaluate_mixed_constant_and_variable() -> Result<()> {
        // arrange
        let first = 10.0.into();
        let second = MonomialEquation::new(1.0, "x", 1).into();
        let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 15.0);
        Ok(())
    }

    #[rstest]
    #[case("x", true)]
    #[case("y", false)]
    fn test_evaluate_returns_error_when_variable_not_in_environment(
        #[case] var_name: &str,
        #[case] first_is_variable: bool,
    ) -> Result<()> {
        // arrange
        let first: Equation = if first_is_variable {
            MonomialEquation::new(1.0, var_name, 1).into()
        } else {
            10.0.into()
        };
        let second: Equation = if first_is_variable {
            5.0.into()
        } else {
            MonomialEquation::new(1.0, var_name, 1).into()
        };
        let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
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
        Ok(())
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
    ) -> Result<()> {
        // arrange
        let first = first_val.into();
        let second = second_val.into();
        let equation = ArithmeticEquation::new(operator, &[first, second])?;
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), expected);
        Ok(())
    }

    #[test]
    fn test_evaluate_with_nested_arithmetic_equations() -> Result<()> {
        // arrange
        let inner_first = 3.0.into();
        let inner_second = 4.0.into();
        let inner_equation = ArithmeticEquation::new(Operator::Add, &[inner_first, inner_second])?;

        let outer_second = 2.0.into();
        let outer_equation =
            ArithmeticEquation::new(Operator::Multiply, &[inner_equation.into(), outer_second])?;
        let env = Environment::empty();

        // act
        let result = outer_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 14.0); // (3 + 4) * 2 = 14
        Ok(())
    }

    #[test]
    fn test_evaluate_complex_nested_expression() -> Result<()> {
        // arrange
        // ((10 + 5) * 2) - (6 / 3) = (15 * 2) - 2 = 30 - 2 = 28
        let add_left = 10.0.into();
        let add_right = 5.0.into();
        let add_equation = ArithmeticEquation::new(Operator::Add, &[add_left, add_right])?;

        let mul_right = 2.0.into();
        let mul_equation =
            ArithmeticEquation::new(Operator::Multiply, &[add_equation.into(), mul_right])?;

        let div_left = 6.0.into();
        let div_right = 3.0.into();
        let div_equation = ArithmeticEquation::new(Operator::Divide, &[div_left, div_right])?;

        let final_equation = ArithmeticEquation::new(
            Operator::Subtract,
            &[mul_equation.into(), div_equation.into()],
        )?;
        let env = Environment::empty();

        // act
        let result = final_equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 28.0);
        Ok(())
    }

    mod operator_ordering_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use std::cmp::Ordering;

        #[rstest]
        #[case(Operator::Add, Operator::Add, Ordering::Equal)]
        #[case(Operator::Add, Operator::Subtract, Ordering::Equal)]
        #[case(Operator::Subtract, Operator::Add, Ordering::Equal)]
        #[case(Operator::Subtract, Operator::Subtract, Ordering::Equal)]
        #[case(Operator::Multiply, Operator::Multiply, Ordering::Equal)]
        #[case(Operator::Multiply, Operator::Divide, Ordering::Equal)]
        #[case(Operator::Divide, Operator::Multiply, Ordering::Equal)]
        #[case(Operator::Divide, Operator::Divide, Ordering::Equal)]
        fn test_operators_with_equal_precedence(
            #[case] op1: Operator,
            #[case] op2: Operator,
            #[case] expected: Ordering,
        ) {
            // arrange & act
            let result = op1.partial_cmp(&op2);

            // assert
            assert_eq!(result, Some(expected));
        }

        #[rstest]
        #[case(Operator::Multiply, Operator::Add, Ordering::Greater)]
        #[case(Operator::Multiply, Operator::Subtract, Ordering::Greater)]
        #[case(Operator::Divide, Operator::Add, Ordering::Greater)]
        #[case(Operator::Divide, Operator::Subtract, Ordering::Greater)]
        fn test_multiplication_and_division_have_higher_precedence_than_addition_and_subtraction(
            #[case] higher_op: Operator,
            #[case] lower_op: Operator,
            #[case] expected: Ordering,
        ) {
            // arrange & act
            let result = higher_op.partial_cmp(&lower_op);

            // assert
            assert_eq!(result, Some(expected));
        }

        #[rstest]
        #[case(Operator::Add, Operator::Multiply, Ordering::Less)]
        #[case(Operator::Add, Operator::Divide, Ordering::Less)]
        #[case(Operator::Subtract, Operator::Multiply, Ordering::Less)]
        #[case(Operator::Subtract, Operator::Divide, Ordering::Less)]
        fn test_addition_and_subtraction_have_lower_precedence_than_multiplication_and_division(
            #[case] lower_op: Operator,
            #[case] higher_op: Operator,
            #[case] expected: Ordering,
        ) {
            // arrange & act
            let result = lower_op.partial_cmp(&higher_op);

            // assert
            assert_eq!(result, Some(expected));
        }

        #[test]
        fn test_ord_is_consistent_with_partial_ord() {
            // arrange
            let operators = vec![
                Operator::Add,
                Operator::Subtract,
                Operator::Multiply,
                Operator::Divide,
            ];

            // act & assert
            for op1 in &operators {
                for op2 in &operators {
                    let ord_result = op1.cmp(op2);
                    let partial_ord_result = op1.partial_cmp(op2);
                    assert_eq!(Some(ord_result), partial_ord_result);
                }
            }
        }

        #[test]
        fn test_operators_can_be_sorted_by_precedence() {
            // arrange
            let mut operators = vec![
                Operator::Add,
                Operator::Divide,
                Operator::Subtract,
                Operator::Multiply,
            ];

            // act
            operators.sort();

            // assert
            // After sorting, operators with lower precedence should come first
            // Add and Subtract have equal precedence (lower)
            assert!(operators[0] == Operator::Add || operators[0] == Operator::Subtract);
            assert!(operators[1] == Operator::Add || operators[1] == Operator::Subtract);
            // Multiply and Divide have equal precedence (higher)
            assert!(operators[2] == Operator::Multiply || operators[2] == Operator::Divide);
            assert!(operators[3] == Operator::Multiply || operators[3] == Operator::Divide);
            // Ensure the two groups are distinct
            assert_ne!(operators[0], operators[2]);
            assert_ne!(operators[1], operators[3]);
        }

        #[test]
        fn test_max_returns_higher_precedence_operator() {
            // arrange & act
            let result1 = Operator::Add.max(Operator::Multiply);
            let result2 = Operator::Divide.max(Operator::Subtract);

            // assert
            assert_eq!(result1, Operator::Multiply);
            assert_eq!(result2, Operator::Divide);
        }

        #[test]
        fn test_min_returns_lower_precedence_operator() {
            // arrange & act
            let result1 = Operator::Add.min(Operator::Multiply);
            let result2 = Operator::Divide.min(Operator::Subtract);

            // assert
            assert_eq!(result1, Operator::Add);
            assert_eq!(result2, Operator::Subtract);
        }
    }

    mod is_variable_related_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_is_variable_related_returns_false_for_both_constants() -> Result<()> {
            // arrange
            let first = 5.0.into();
            let second = 3.0.into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, false);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_returns_true_when_first_operand_has_variable() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = 3.0.into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, true);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_returns_true_when_second_operand_has_variable() -> Result<()> {
            // arrange
            let first = 5.0.into();
            let second = MonomialEquation::new(3.0, "y", 1).into();
            let equation = ArithmeticEquation::new(Operator::Multiply, &[first, second])?;
            let variable = Variable::new("y", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, true);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_returns_true_when_both_operands_have_same_variable()
        -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = MonomialEquation::new(3.0, "x", 2).into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
            let variable = Variable::new("x", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, true);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_returns_true_when_either_operand_has_variable() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = MonomialEquation::new(3.0, "y", 1).into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
            let var_x = Variable::new("x", 0.0);
            let var_y = Variable::new("y", 0.0);

            // act
            let result_x = equation.is_variable_related(&var_x);
            let result_y = equation.is_variable_related(&var_y);

            // assert
            assert_eq!(result_x, true);
            assert_eq!(result_y, true);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_returns_false_when_different_variable() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = MonomialEquation::new(3.0, "y", 1).into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;
            let variable = Variable::new("z", 0.0);

            // act
            let result = equation.is_variable_related(&variable);

            // assert
            assert_eq!(result, false);
            Ok(())
        }

        #[test]
        fn test_is_variable_related_with_nested_equations() -> Result<()> {
            // arrange
            // (x + 2) * 3
            let inner_first = MonomialEquation::new(1.0, "x", 1).into();
            let inner_second = 2.0.into();
            let inner_equation =
                ArithmeticEquation::new(Operator::Add, &[inner_first, inner_second])?;
            let outer_second = 3.0.into();
            let outer_equation = ArithmeticEquation::new(
                Operator::Multiply,
                &[inner_equation.into(), outer_second],
            )?;
            let var_x = Variable::new("x", 0.0);
            let var_y = Variable::new("y", 0.0);

            // act
            let result_x = outer_equation.is_variable_related(&var_x);
            let result_y = outer_equation.is_variable_related(&var_y);

            // assert
            assert_eq!(result_x, true);
            assert_eq!(result_y, false);
            Ok(())
        }
    }

    mod related_variables_tests {
        use super::*;
        use crate::equation::monomial::MonomialEquation;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_related_variables_returns_empty_for_constants_only() -> Result<()> {
            // arrange
            let first = 5.0.into();
            let second = 3.0.into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;

            // act
            let result = equation.related_variables();

            // assert
            assert_eq!(result, Vec::<String>::new());
            Ok(())
        }

        #[test]
        fn test_related_variables_returns_single_variable_from_first_operand() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = 3.0.into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;

            // act
            let mut result = equation.related_variables();
            result.sort();

            // assert
            assert_eq!(result, vec!["x".to_string()]);
            Ok(())
        }

        #[test]
        fn test_related_variables_returns_single_variable_from_second_operand() -> Result<()> {
            // arrange
            let first = 5.0.into();
            let second = MonomialEquation::new(3.0, "y", 1).into();
            let equation = ArithmeticEquation::new(Operator::Multiply, &[first, second])?;

            // act
            let mut result = equation.related_variables();
            result.sort();

            // assert
            assert_eq!(result, vec!["y".to_string()]);
            Ok(())
        }

        #[test]
        fn test_related_variables_returns_multiple_unique_variables() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = MonomialEquation::new(3.0, "y", 1).into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;

            // act
            let mut result = equation.related_variables();
            result.sort();

            // assert
            assert_eq!(result, vec!["x".to_string(), "y".to_string()]);
            Ok(())
        }

        #[test]
        fn test_related_variables_deduplicates_same_variable() -> Result<()> {
            // arrange
            let first = MonomialEquation::new(2.0, "x", 1).into();
            let second = MonomialEquation::new(3.0, "x", 2).into();
            let equation = ArithmeticEquation::new(Operator::Add, &[first, second])?;

            // act
            let mut result = equation.related_variables();
            result.sort();

            // assert
            assert_eq!(result, vec!["x".to_string()]);
            Ok(())
        }

        #[test]
        fn test_related_variables_with_nested_equations() -> Result<()> {
            // arrange
            // (x + y) * z
            let inner_first = MonomialEquation::new(1.0, "x", 1).into();
            let inner_second = MonomialEquation::new(1.0, "y", 1).into();
            let inner_equation =
                ArithmeticEquation::new(Operator::Add, &[inner_first, inner_second])?;
            let outer_second = MonomialEquation::new(1.0, "z", 1).into();
            let outer_equation = ArithmeticEquation::new(
                Operator::Multiply,
                &[inner_equation.into(), outer_second],
            )?;

            // act
            let mut result = outer_equation.related_variables();
            result.sort();

            // assert
            assert_eq!(
                result,
                vec!["x".to_string(), "y".to_string(), "z".to_string()]
            );
            Ok(())
        }

        #[test]
        fn test_related_variables_with_nested_equations_and_duplicates() -> Result<()> {
            // arrange
            // (x + 2) * (x + 3)
            let left_var = MonomialEquation::new(1.0, "x", 1).into();
            let left_const = 2.0.into();
            let left_equation = ArithmeticEquation::new(Operator::Add, &[left_var, left_const])?;

            let right_var = MonomialEquation::new(1.0, "x", 1).into();
            let right_const = 3.0.into();
            let right_equation = ArithmeticEquation::new(Operator::Add, &[right_var, right_const])?;

            let outer_equation = ArithmeticEquation::new(
                Operator::Multiply,
                &[left_equation.into(), right_equation.into()],
            )?;

            // act
            let mut result = outer_equation.related_variables();
            result.sort();

            // assert
            assert_eq!(result, vec!["x".to_string()]);
            Ok(())
        }
    }
}
