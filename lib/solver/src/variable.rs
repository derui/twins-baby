use std::ops::{Add, Div, Mul, Sub};

/// A simple variable representation
pub struct Variable {
    /// Name of the variable
    name: String,
    /// Current value of the variable
    value: f32,
}

// implementation for arithmetic operations
impl Add<f32> for Variable {
    type Output = Variable;

    fn add(self, rhs: f32) -> Self::Output {
        Variable {
            value: self.value + rhs,
            ..self
        }
    }
}

impl Sub<f32> for Variable {
    type Output = Variable;

    fn sub(self, rhs: f32) -> Self::Output {
        Variable {
            value: self.value - rhs,
            ..self
        }
    }
}

impl Mul<f32> for Variable {
    type Output = Variable;

    fn mul(self, rhs: f32) -> Self::Output {
        Variable {
            value: self.value * rhs,
            ..self
        }
    }
}

impl Div<f32> for Variable {
    type Output = Variable;

    fn div(self, rhs: f32) -> Self::Output {
        Variable {
            value: self.value / rhs,
            ..self
        }
    }
}

impl Variable {
    /// Make a new variable with name
    pub fn new(name: &str, value: f32) -> Self {
        Variable {
            name: name.to_string(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_variable_new() {
        let var = Variable::new("x", 5.0);
        assert_eq!(var.name, "x");
        assert_eq!(var.value, 5.0);
    }

    #[test]
    fn test_variable_add() {
        let var = Variable::new("x", 10.0);
        let result = var + 5.0;
        assert_eq!(result.name, "x");
        assert_eq!(result.value, 15.0);
    }

    #[test]
    fn test_variable_sub() {
        let var = Variable::new("y", 20.0);
        let result = var - 8.0;
        assert_eq!(result.name, "y");
        assert_eq!(result.value, 12.0);
    }

    #[test]
    fn test_variable_mul() {
        let var = Variable::new("z", 4.0);
        let result = var * 3.0;
        assert_eq!(result.name, "z");
        assert_eq!(result.value, 12.0);
    }

    #[test]
    fn test_variable_div() {
        let var = Variable::new("w", 20.0);
        let result = var / 4.0;
        assert_eq!(result.name, "w");
        assert_eq!(result.value, 5.0);
    }

    #[test]
    fn test_variable_chained_operations() {
        let var = Variable::new("a", 10.0);
        let result = ((var + 5.0) * 2.0 - 10.0) / 5.0;
        assert_eq!(result.name, "a");
        assert_eq!(result.value, 4.0);
    }

    #[test]
    fn test_variable_with_negative_value() {
        let var = Variable::new("neg", -10.0);
        let result = var + 15.0;
        assert_eq!(result.value, 5.0);
    }

    #[test]
    fn test_variable_with_zero() {
        let var = Variable::new("zero", 0.0);
        let result = var + 0.0;
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn test_variable_floating_point_precision() {
        let var = Variable::new("float", 0.1);
        let result = var + 0.2;
        assert!((result.value - 0.3).abs() < 1e-6);
    }
}
