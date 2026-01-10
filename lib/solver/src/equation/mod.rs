pub(crate) mod arithmetic;
pub(crate) mod constant;
pub(crate) mod monomial;

use std::{collections::HashMap, fmt::Display, iter::Map};

use crate::{environment::Environment, variable::Variable};

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

    /// derive a derivative for the equation with respect to the variable
    ///
    /// # Arguments
    /// * `variable` - The variable to derive with respect to
    ///
    /// # Returns
    /// The derived equation. None if it can not be derived.
    fn derive(&self, variable: &Variable) -> Option<Box<dyn Equation>>;

    /// return the equation related or not
    fn is_variable_related(&self, variable: &Variable) -> bool;
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
    use monomial::MonomialEquation;
    use pretty_assertions::assert_eq;

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
    fn test_monomial_equations_equal_with_same_factor_variable_and_exponent() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(MonomialEquation::new(2.0, "x", 3));
        let eq2: Box<dyn Equation> = Box::new(MonomialEquation::new(2.0, "x", 3));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_monomial_equations_not_equal_with_different_factor() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(MonomialEquation::new(2.0, "x", 3));
        let eq2: Box<dyn Equation> = Box::new(MonomialEquation::new(3.0, "x", 3));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_monomial_equations_not_equal_with_different_exponent() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(MonomialEquation::new(2.0, "x", 3));
        let eq2: Box<dyn Equation> = Box::new(MonomialEquation::new(2.0, "x", 4));

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
    fn test_monomial_equations_with_different_case_not_equal() {
        // arrange
        let eq1: Box<dyn Equation> = Box::new(MonomialEquation::new(1.0, "Variable", 1));
        let eq2: Box<dyn Equation> = Box::new(MonomialEquation::new(1.0, "variable", 1));

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }
}
