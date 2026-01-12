use std::{
    fmt::Display,
    ops::{Add, Div, DivAssign, Mul, Sub},
    path::PrefixComponent,
};

use crate::{
    environment::Environment,
    equation::{
        arithmetic::{ArithmeticEquation, Operator},
        constant::ConstantEquation,
        monomial::MonomialEquation,
    },
    variable::Variable,
};

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

/// Operation wrapper
pub struct Ops(Box<dyn Equation>);

impl Ops {
    /// Make constant from the value
    pub fn constant(v: f32) -> Self {
        Self(v.into())
    }

    /// Make monomial from the value
    pub fn monomial(factor: f32, name: &str, exponent: i32) -> Self {
        Self(MonomialEquation::new(factor, name, exponent).into())
    }
}

impl From<(f32, &str, i32)> for Ops {
    fn from(value: (f32, &str, i32)) -> Self {
        Ops::monomial(value.0, value.1, value.2)
    }
}

impl From<(&str, i32)> for Ops {
    fn from(value: (&str, i32)) -> Self {
        Ops::monomial(1.0, value.0, value.1)
    }
}

impl From<&str> for Ops {
    fn from(value: &str) -> Self {
        Ops::monomial(1.0, value, 1)
    }
}

impl From<f32> for Ops {
    fn from(value: f32) -> Self {
        Ops::constant(value)
    }
}

impl From<Ops> for Box<dyn Equation> {
    fn from(value: Ops) -> Self {
        value.0
    }
}

// ops for ops
impl Add for Ops {
    type Output = Ops;

    fn add(self, rhs: Self) -> Self::Output {
        let left = self.0;
        let right = rhs.0;

        Ops(ArithmeticEquation::new(Operator::Add, &[left, right])
            .expect("must be success")
            .into())
    }
}

impl Sub for Ops {
    type Output = Ops;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self.0;
        let right = rhs.0;

        Ops(ArithmeticEquation::new(Operator::Subtract, &[left, right])
            .expect("must be success")
            .into())
    }
}

impl Mul for Ops {
    type Output = Ops;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = self.0;
        let right = rhs.0;

        Ops(ArithmeticEquation::new(Operator::Multiply, &[left, right])
            .expect("must be success")
            .into())
    }
}

impl Div for Ops {
    type Output = Ops;

    fn div(self, rhs: Self) -> Self::Output {
        let left = self.0;
        let right = rhs.0;

        Ops(ArithmeticEquation::new(Operator::Divide, &[left, right])
            .expect("must be success")
            .into())
    }
}
