pub(crate) mod arithmetic;
pub(crate) mod constant;
pub(crate) mod monomial;
mod ops;
mod parser;

use std::fmt::Display;

/// Error cases for solving equation
#[derive(Debug, Clone)]
pub enum EquationError {
    /// Can not found variables in the environment
    NoVariableInEnvironment(Vec<String>),
}

/// Equation trait should provide some of the equation behavior of the solver
#[delegatable_trait]
pub trait Evaluate {
    /// Evaluate the equation.
    ///
    /// # Arguments
    /// * `env` - current environment
    ///
    /// # Returns
    /// result of equation with the environment. Error when some errors
    fn evaluate(&self, env: &Environment) -> Result<f32, EquationError>;

    /// return the equation related or not
    fn is_variable_related(&self, variable: &Variable) -> bool;
}

#[delegatable_trait_remote]
trait Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>;
}

/// A central Equation, it avoid to Boxing
#[derive(Debug, Clone, PartialEq, Delegate)]
#[delegate(Evaluate)]
#[delegate(std::fmt::Display)]
pub enum Equation {
    Constant(ConstantEquation),
    Monomial(MonomialEquation),
    Arithmetic(ArithmeticEquation),
}

use ambassador::{Delegate, delegatable_trait, delegatable_trait_remote};
pub use ops::*;

use crate::{
    environment::Environment,
    equation::{
        arithmetic::ArithmeticEquation, constant::ConstantEquation, monomial::MonomialEquation,
    },
    variable::Variable,
};

impl From<f32> for Equation {
    fn from(value: f32) -> Self {
        Equation::Constant(value.into())
    }
}

impl From<ConstantEquation> for Equation {
    fn from(value: ConstantEquation) -> Self {
        Equation::Constant(value)
    }
}

impl From<MonomialEquation> for Equation {
    fn from(value: MonomialEquation) -> Self {
        Equation::Monomial(value)
    }
}

impl From<ArithmeticEquation> for Equation {
    fn from(value: ArithmeticEquation) -> Self {
        Equation::Arithmetic(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arithmetic::{ArithmeticEquation, Operator};
    use monomial::MonomialEquation;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_constant_equations_equal_with_same_value() {
        // arrange
        let eq1: Equation = 42.0.into();
        let eq2: Equation = 42.0.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_constant_equations_not_equal_with_different_values() {
        // arrange
        let eq1: Equation = 42.0.into();
        let eq2: Equation = 43.0.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_arithmetic_equations_equal_with_same_structure() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 5.0.into();
        let const2: Equation = 3.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Add, &[const1.clone(), const2.clone()])?.into();
        let eq2: Equation = ArithmeticEquation::new(Operator::Add, &[const1, const2])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
        Ok(())
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_different_operators() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 5.0.into();
        let const2: Equation = 3.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Add, &[const1.clone(), const2.clone()])?.into();
        let eq2: Equation = ArithmeticEquation::new(Operator::Subtract, &[const1, const2])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
        Ok(())
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_different_operands() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 5.0.into();
        let const2: Equation = 3.0.into();
        let const3: Equation = 7.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Add, &[const1.clone(), const2])?.into();
        let eq2: Equation = ArithmeticEquation::new(Operator::Add, &[const1, const3])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
        Ok(())
    }

    #[test]
    fn test_arithmetic_equations_not_equal_with_swapped_operands() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 5.0.into();
        let const2: Equation = 3.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Subtract, &[const1.clone(), const2.clone()])?.into();
        let eq2: Equation = ArithmeticEquation::new(Operator::Subtract, &[const2, const1])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
        Ok(())
    }

    #[test]
    fn test_monomial_equations_equal_with_same_factor_variable_and_exponent() {
        // arrange
        let eq1: Equation = MonomialEquation::new(2.0, "x", 3).into();
        let eq2: Equation = MonomialEquation::new(2.0, "x", 3).into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_monomial_equations_not_equal_with_different_factor() {
        // arrange
        let eq1: Equation = MonomialEquation::new(2.0, "x", 3).into();
        let eq2: Equation = MonomialEquation::new(3.0, "x", 3).into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_monomial_equations_not_equal_with_different_exponent() {
        // arrange
        let eq1: Equation = MonomialEquation::new(2.0, "x", 3).into();
        let eq2: Equation = MonomialEquation::new(2.0, "x", 4).into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_different_equation_types_not_equal() -> anyhow::Result<()> {
        // arrange
        let const_eq: Equation = 6.0.into();
        let const1: Equation = 2.0.into();
        let const2: Equation = 3.0.into();
        let arith_eq: Equation =
            ArithmeticEquation::new(Operator::Multiply, &[const1, const2])?.into();

        // act
        let result = const_eq == arith_eq;

        // assert
        assert_eq!(result, false);
        Ok(())
    }

    #[test]
    fn test_nested_arithmetic_equations_equal_with_same_structure() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 2.0.into();
        let const2: Equation = 3.0.into();
        let inner1 = ArithmeticEquation::new(Operator::Add, &[const1.clone(), const2.clone()])?;
        let const3: Equation = 4.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Multiply, &[inner1.into(), const3.clone()])?.into();

        let inner2 = ArithmeticEquation::new(Operator::Add, &[const1, const2])?;
        let eq2: Equation =
            ArithmeticEquation::new(Operator::Multiply, &[inner2.into(), const3])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
        Ok(())
    }

    #[test]
    fn test_nested_arithmetic_equations_not_equal_with_different_inner_structure()
    -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 2.0.into();
        let const2: Equation = 3.0.into();
        let inner1 = ArithmeticEquation::new(Operator::Add, &[const1.clone(), const2.clone()])?;
        let const3: Equation = 4.0.into();
        let eq1: Equation =
            ArithmeticEquation::new(Operator::Multiply, &[inner1.into(), const3.clone()])?.into();

        let inner2 = ArithmeticEquation::new(Operator::Subtract, &[const1, const2])?;
        let eq2: Equation =
            ArithmeticEquation::new(Operator::Multiply, &[inner2.into(), const3])?.into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
        Ok(())
    }

    #[test]
    fn test_cloned_equations_are_equal() -> anyhow::Result<()> {
        // arrange
        let const1: Equation = 5.0.into();
        let const2: Equation = 3.0.into();
        let eq1: Equation = ArithmeticEquation::new(Operator::Add, &[const1, const2])?.into();

        // act
        let eq2 = eq1.clone();
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, true);
        Ok(())
    }

    #[test]
    fn test_monomial_equations_with_different_case_not_equal() {
        // arrange
        let eq1: Equation = MonomialEquation::new(1.0, "Variable", 1).into();
        let eq2: Equation = MonomialEquation::new(1.0, "variable", 1).into();

        // act
        let result = eq1 == eq2;

        // assert
        assert_eq!(result, false);
    }
}
