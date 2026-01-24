use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use ambassador::{delegatable_trait, delegatable_trait_remote};

use crate::{
    environment::Environment,
    equation::{
        Equation,
        arithmetic::{ArithmeticEquation, Operator},
        monomial::MonomialEquation,
    },
    variable::Variable,
};

/// Operation wrapper
pub struct Ops(Equation);

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

impl From<Ops> for Equation {
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
