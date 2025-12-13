pub(crate) mod arithmetic;
pub(crate) mod constant;
pub(crate) mod power;
pub(crate) mod unary;
pub(crate) mod variable;

use std::fmt::Display;

use crate::environment::Environment;

/// Error cases for solving equation
#[derive(Debug, Clone)]
pub enum EquationError {
    /// Can not found variables in the environment
    NoVariableInEnvironment(Vec<String>),
}

/// Equation trait should provide some of the equation behavior of the solver
pub trait Equation: std::fmt::Debug + EquationClone + Display {
    /// Evaluate the equation.
    ///
    /// # Arguments
    /// * `env` - current environment
    ///
    /// # Returns
    /// result of equation with the environment. Error when some errors
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError>;
}

/// A support trait to define Clone for Box<dyn Equation>
pub trait EquationClone {
    /// Clone the equation into a boxed equation
    fn clone_box(&self) -> Box<dyn Equation>;
}

impl<T> EquationClone for T
where
    T: 'static + Equation + Clone,
{
    fn clone_box(&self) -> Box<dyn Equation> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Equation> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Equation> {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arithmetic::{ArithmeticEquation, Operator};
    use constant::ConstantEquation;
    use power::PowerEquation;
    use pretty_assertions::assert_eq;
    use unary::UnaryEquation;
    use variable::VariableEquation;

    #[test]
    fn test_constant_equations_equal_with_same_value() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(ConstantEquation::new(42.0));
        let eq2: Box<dyn Equation> = Box::new(ConstantEquation::new(42.0));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_constant_equations_not_equal_with_different_values() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(ConstantEquation::new(42.0));
        let eq2: Box<dyn Equation> = Box::new(ConstantEquation::new(43.0));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_variable_equations_equal_with_same_name() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(VariableEquation::new("x"));
        let eq2: Box<dyn Equation> = Box::new(VariableEquation::new("x"));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_variable_equations_not_equal_with_different_names() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(VariableEquation::new("x"));
        let eq2: Box<dyn Equation> = Box::new(VariableEquation::new("y"));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_constant_and_variable_equations_not_equal() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(ConstantEquation::new(42.0));
        let eq2: Box<dyn Equation> = Box::new(VariableEquation::new("x"));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_arithmetic_equations_equal_with_same_structure() {
        // arrange
        let const1 = ConstantEquation::new(5.0);
        let const2 = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const2));
        let eq2: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const2));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_different_operators() {
        // arrange
        let const1 = ConstantEquation::new(5.0);
        let const2 = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const2));
        let eq2: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Subtract,
            &const1,
            &const2,
        ));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_different_operands() {
        // arrange
        let const1 = ConstantEquation::new(5.0);
        let const2 = ConstantEquation::new(3.0);
        let const3 = ConstantEquation::new(7.0);
        let eq1: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const2));
        let eq2: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const3));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_swapped_operands() {
        // arrange
        let const1 = ConstantEquation::new(5.0);
        let const2 = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Subtract,
            &const1,
            &const2,
        ));
        let eq2: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Subtract,
            &const2,
            &const1,
        ));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_power_equations_equal_with_same_base_and_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);
        let exponent = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> = Box::new(PowerEquation::new(&base, &exponent));
        let eq2: Box<dyn Equation> = Box::new(PowerEquation::new(&base, &exponent));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_power_equations_not_equal_with_different_base() {
        // arrange
        let base1 = ConstantEquation::new(2.0);
        let base2 = ConstantEquation::new(3.0);
        let exponent = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> = Box::new(PowerEquation::new(&base1, &exponent));
        let eq2: Box<dyn Equation> = Box::new(PowerEquation::new(&base2, &exponent));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_power_equations_not_equal_with_different_exponent() {
        // arrange
        let base = ConstantEquation::new(2.0);
        let exponent1 = ConstantEquation::new(3.0);
        let exponent2 = ConstantEquation::new(4.0);
        let eq1: Box<dyn Equation> = Box::new(PowerEquation::new(&base, &exponent1));
        let eq2: Box<dyn Equation> = Box::new(PowerEquation::new(&base, &exponent2));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_unary_equations_equal_with_same_factor_and_expression() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression = ConstantEquation::new(5.0);
        let eq1: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor, &expression));
        let eq2: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor, &expression));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_unary_equations_not_equal_with_different_factor() {
        // arrange
        let factor1 = ConstantEquation::new(2.0);
        let factor2 = ConstantEquation::new(3.0);
        let expression = ConstantEquation::new(5.0);
        let eq1: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor1, &expression));
        let eq2: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor2, &expression));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_unary_equations_not_equal_with_different_expression() {
        // arrange
        let factor = ConstantEquation::new(2.0);
        let expression1 = ConstantEquation::new(5.0);
        let expression2 = ConstantEquation::new(7.0);
        let eq1: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor, &expression1));
        let eq2: Box<dyn Equation> = Box::new(UnaryEquation::new(&factor, &expression2));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_different_equation_types_not_equal() {
        // arrange
        let const_eq: Box<dyn Equation> = Box::new(ConstantEquation::new(6.0));
        let const1 = ConstantEquation::new(2.0);
        let const2 = ConstantEquation::new(3.0);
        let arith_eq: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Multiply,
            &const1,
            &const2,
        ));

        // act
        let result = const_eq == arith_eq;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_nested_arithmetic_equations_equal_with_same_structure() {
        // arrange
        let const1 = ConstantEquation::new(2.0);
        let const2 = ConstantEquation::new(3.0);
        let inner1 = ArithmeticEquation::new(Operator::Add, &const1, &const2);
        let const3 = ConstantEquation::new(4.0);
        let eq1: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Multiply,
            &inner1,
            &const3,
        ));

        let inner2 = ArithmeticEquation::new(Operator::Add, &const1, &const2);
        let eq2: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Multiply,
            &inner2,
            &const3,
        ));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_nested_arithmetic_equations_not_equal_with_different_inner_structure() {
        // arrange
        let const1 = ConstantEquation::new(2.0);
        let const2 = ConstantEquation::new(3.0);
        let inner1 = ArithmeticEquation::new(Operator::Add, &const1, &const2);
        let const3 = ConstantEquation::new(4.0);
        let eq1: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Multiply,
            &inner1,
            &const3,
        ));

        let inner2 = ArithmeticEquation::new(Operator::Subtract, &const1, &const2);
        let eq2: Box<dyn Equation> = Box::new(ArithmeticEquation::new(
            Operator::Multiply,
            &inner2,
            &const3,
        ));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_complex_mixed_equations_equal() {
        // arrange
        // (2^3) + (4 * 5)
        let base = ConstantEquation::new(2.0);
        let exp = ConstantEquation::new(3.0);
        let power = PowerEquation::new(&base, &exp);

        let mul_left = ConstantEquation::new(4.0);
        let mul_right = ConstantEquation::new(5.0);
        let multiply = ArithmeticEquation::new(Operator::Multiply, &mul_left, &mul_right);

        let eq1: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &power, &multiply));

        let base2 = ConstantEquation::new(2.0);
        let exp2 = ConstantEquation::new(3.0);
        let power2 = PowerEquation::new(&base2, &exp2);

        let mul_left2 = ConstantEquation::new(4.0);
        let mul_right2 = ConstantEquation::new(5.0);
        let multiply2 = ArithmeticEquation::new(Operator::Multiply, &mul_left2, &mul_right2);

        let eq2: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &power2, &multiply2));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_cloned_equations_are_equal() {
        // arrange
        let const1 = ConstantEquation::new(5.0);
        let const2 = ConstantEquation::new(3.0);
        let eq1: Box<dyn Equation> =
            Box::new(ArithmeticEquation::new(Operator::Add, &const1, &const2));

        // act
        let eq2 = eq1.clone();
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_variable_equations_with_different_case_not_equal() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(VariableEquation::new("Variable"));
        let eq2: Box<dyn Equation> = Box::new(VariableEquation::new("variable"));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }
}
