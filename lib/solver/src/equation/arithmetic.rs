use std::{
    fmt::Formatter,
    ops::{Add, Div, Mul, Sub},
};

use crate::equation::Equation;

/// Operator of arithmetic
#[derive(Debug, Copy, Clone)]
pub(crate) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Implementation of arithmetic equation
#[derive(Clone)]
pub(crate) struct ArithmeticEquation {
    operator: Operator,
    first: Box<dyn Equation>,
    second: Box<dyn Equation>,
}

impl Equation for ArithmeticEquation {
    fn evaluate(&self, env: &crate::environment::Environment) -> Result<f32, super::EquationError> {
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

    fn clone_box(&self) -> Box<dyn Equation> {
        Box::new(self.clone())
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArithmeticEquation")
            .field("operator", &self.operator)
            .field("first", &self.first)
            .field("second", &self.second)
            .finish()
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
    use crate::equation::variable::VariableEquation;
    use crate::variable::Variable;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_creates_arithmetic_equation_with_add_operator() {
        // arrange
        let first = ConstantEquation::new(5.0);
        let second = ConstantEquation::new(3.0);

        // act
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);

        // assert
        let env = Environment::empty();
        assert_eq!(equation.evaluate(&env).unwrap(), 8.0);
    }

    #[test]
    fn test_evaluate_add_with_constants() {
        // arrange
        let first = ConstantEquation::new(10.0);
        let second = ConstantEquation::new(7.0);
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 17.0);
    }

    #[test]
    fn test_evaluate_minus_with_constants() {
        // arrange
        let first = ConstantEquation::new(15.0);
        let second = ConstantEquation::new(8.0);
        let equation = ArithmeticEquation::new(Operator::Subtract, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 7.0);
    }

    #[test]
    fn test_evaluate_multiply_with_constants() {
        // arrange
        let first = ConstantEquation::new(6.0);
        let second = ConstantEquation::new(4.0);
        let equation = ArithmeticEquation::new(Operator::Multiply, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 24.0);
    }

    #[test]
    fn test_evaluate_divide_with_constants() {
        // arrange
        let first = ConstantEquation::new(20.0);
        let second = ConstantEquation::new(4.0);
        let equation = ArithmeticEquation::new(Operator::Divide, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_add_with_variables() {
        // arrange
        let first = VariableEquation::new("x");
        let second = VariableEquation::new("y");
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let var1 = Variable::new("x", 12.0);
        let var2 = Variable::new("y", 8.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_minus_with_variables() {
        // arrange
        let first = VariableEquation::new("a");
        let second = VariableEquation::new("b");
        let equation = ArithmeticEquation::new(Operator::Subtract, &first, &second);
        let var1 = Variable::new("a", 25.0);
        let var2 = Variable::new("b", 10.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 15.0);
    }

    #[test]
    fn test_evaluate_multiply_with_variables() {
        // arrange
        let first = VariableEquation::new("x");
        let second = VariableEquation::new("y");
        let equation = ArithmeticEquation::new(Operator::Multiply, &first, &second);
        let var1 = Variable::new("x", 7.0);
        let var2 = Variable::new("y", 3.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 21.0);
    }

    #[test]
    fn test_evaluate_divide_with_variables() {
        // arrange
        let first = VariableEquation::new("numerator");
        let second = VariableEquation::new("denominator");
        let equation = ArithmeticEquation::new(Operator::Divide, &first, &second);
        let var1 = Variable::new("numerator", 30.0);
        let var2 = Variable::new("denominator", 6.0);
        let env = Environment::from_variables(vec![var1, var2]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_mixed_constant_and_variable() {
        // arrange
        let first = ConstantEquation::new(10.0);
        let second = VariableEquation::new("x");
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let var = Variable::new("x", 5.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 15.0);
    }

    #[test]
    fn test_evaluate_returns_error_when_first_variable_not_in_environment() {
        // arrange
        let first = VariableEquation::new("x");
        let second = ConstantEquation::new(5.0);
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
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
    fn test_evaluate_returns_error_when_second_variable_not_in_environment() {
        // arrange
        let first = ConstantEquation::new(10.0);
        let second = VariableEquation::new("y");
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        match result {
            Err(super::super::EquationError::NoVariableInEnvironment(vars)) => {
                assert_eq!(vars, vec!["y".to_string()]);
            }
            _ => panic!("Expected NoVariableInEnvironment error"),
        }
    }

    #[test]
    fn test_evaluate_add_with_zero() {
        // arrange
        let first = ConstantEquation::new(42.0);
        let second = ConstantEquation::new(0.0);
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_minus_with_zero() {
        // arrange
        let first = ConstantEquation::new(50.0);
        let second = ConstantEquation::new(0.0);
        let equation = ArithmeticEquation::new(Operator::Subtract, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 50.0);
    }

    #[test]
    fn test_evaluate_multiply_with_zero() {
        // arrange
        let first = ConstantEquation::new(100.0);
        let second = ConstantEquation::new(0.0);
        let equation = ArithmeticEquation::new(Operator::Multiply, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_multiply_with_one() {
        // arrange
        let first = ConstantEquation::new(75.0);
        let second = ConstantEquation::new(1.0);
        let equation = ArithmeticEquation::new(Operator::Multiply, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 75.0);
    }

    #[test]
    fn test_evaluate_divide_by_one() {
        // arrange
        let first = ConstantEquation::new(99.0);
        let second = ConstantEquation::new(1.0);
        let equation = ArithmeticEquation::new(Operator::Divide, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 99.0);
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
    fn test_clone_creates_independent_copy() {
        // arrange
        let first = ConstantEquation::new(8.0);
        let second = ConstantEquation::new(3.0);
        let equation = ArithmeticEquation::new(Operator::Add, &first, &second);
        let env = Environment::empty();

        // act
        let cloned_equation = equation.clone();
        let result1 = equation.evaluate(&env);
        let result2 = cloned_equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 11.0);
        assert_eq!(result2.unwrap(), 11.0);
    }

    #[test]
    fn test_evaluate_multiple_times_returns_consistent_results() {
        // arrange
        let first = VariableEquation::new("x");
        let second = ConstantEquation::new(5.0);
        let equation = ArithmeticEquation::new(Operator::Multiply, &first, &second);
        let var = Variable::new("x", 4.0);
        let env = Environment::from_variables(vec![var]);

        // act
        let result1 = equation.evaluate(&env);
        let result2 = equation.evaluate(&env);
        let result3 = equation.evaluate(&env);

        // assert
        assert_eq!(result1.unwrap(), 20.0);
        assert_eq!(result2.unwrap(), 20.0);
        assert_eq!(result3.unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_subtract_negative_result() {
        // arrange
        let first = ConstantEquation::new(5.0);
        let second = ConstantEquation::new(10.0);
        let equation = ArithmeticEquation::new(Operator::Subtract, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), -5.0);
    }

    #[test]
    fn test_evaluate_divide_result_less_than_one() {
        // arrange
        let first = ConstantEquation::new(3.0);
        let second = ConstantEquation::new(4.0);
        let equation = ArithmeticEquation::new(Operator::Divide, &first, &second);
        let env = Environment::empty();

        // act
        let result = equation.evaluate(&env);

        // assert
        assert_eq!(result.unwrap(), 0.75);
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
}
