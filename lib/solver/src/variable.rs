use immutable::Im;

/// A simple variable representation
#[derive(Debug, Clone)]
pub struct Variable {
    /// Name of the variable
    pub name: Im<String>,
    /// Current value of the variable
    pub value: f32,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        // same name as same variable.
        *self.name == *other.name
    }
}

impl From<Variable> for f32 {
    fn from(value: Variable) -> Self {
        value.value
    }
}

impl From<&Variable> for f32 {
    fn from(value: &Variable) -> Self {
        value.value
    }
}

impl Variable {
    /// Make a new variable with name
    pub fn new(name: &str, value: f32) -> Self {
        assert!(!name.trim().is_empty(), "Variable name cannot be empty");

        Variable {
            name: name.trim().to_string().into(),
            value,
        }
    }

    /// Make a new variable with new name
    pub fn with_name(&self, name: &str) -> Self {
        Variable {
            name: name.to_string().into(),
            value: self.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn test_empty_name() {
        // arrange

        // act

        // assert
        Variable::new("", 10.0);
    }

    #[test]
    #[should_panic]
    fn test_name_only_blank() {
        // arrange

        // act

        // assert
        Variable::new("  \t\n", 10.0);
    }

    #[test]
    fn test_name_should_trimmed() {
        // arrange

        // act
        let v = Variable::new("  x \t\n", 5.);

        // assert
        assert_eq!(*v.name, "x");
    }

    #[test]
    fn test_equality_same_name_same_value() {
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("x", 10.0);
        assert_eq!(var1, var2);
    }

    #[test]
    fn test_equality_same_name_different_value() {
        // Variables with same name are equal regardless of value
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("x", 20.0);
        assert_eq!(var1, var2);
    }

    #[test]
    fn test_inequality_different_name() {
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 10.0);
        assert_ne!(var1, var2);
    }

    #[test]
    fn test_inequality_different_name_different_value() {
        let var1 = Variable::new("x", 10.0);
        let var2 = Variable::new("y", 20.0);
        assert_ne!(var1, var2);
    }
}
