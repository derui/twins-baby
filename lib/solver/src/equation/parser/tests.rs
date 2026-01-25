use anyhow::{Result, anyhow};
use approx::assert_relative_eq;

use super::parse;
use crate::{environment::Environment, equation::Evaluate};

#[test]
fn test_parse_constant() -> Result<()> {
    // Arrange
    let input = "42.0";
    let env = Environment::empty();

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 42.0);

    Ok(())
}

#[test]
fn test_parse_zero_constant() -> Result<()> {
    // Arrange
    let input = "0.0";
    let env = Environment::empty();

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 0.0);

    Ok(())
}

#[test]
fn test_parse_negative_constant() -> Result<()> {
    // Arrange
    let input = "-5.0";
    let env = Environment::empty();

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, -5.0);

    Ok(())
}

#[test]
fn test_parse_simple_monomial() -> Result<()> {
    // Arrange
    let input = "x";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 3.0);

    Ok(())
}

#[test]
fn test_parse_monomial_with_coefficient() -> Result<()> {
    // Arrange
    let input = "3.5x";
    let env = Environment::from_tuples(&[("x", 2.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 7.0);

    Ok(())
}

#[test]
fn test_parse_monomial_with_exponent() -> Result<()> {
    // Arrange
    let input = "x^2";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 9.0);

    Ok(())
}

#[test]
fn test_parse_monomial_with_coefficient_and_exponent() -> Result<()> {
    // Arrange
    let input = "2.5y^3";
    let env = Environment::from_tuples(&[("y", 2.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 20.0);

    Ok(())
}

#[test]
fn test_parse_negative_exponent() -> Result<()> {
    // Arrange
    let input = "x^-2";
    let env = Environment::from_tuples(&[("x", 2.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 0.25);

    Ok(())
}

#[test]
fn test_parse_variable_with_underscore() -> Result<()> {
    // Arrange
    let input = "var_name";
    let env = Environment::from_tuples(&[("var_name", 5.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 5.0);

    Ok(())
}

#[test]
fn test_parse_addition() -> Result<()> {
    // Arrange
    let input = "x + 5.0";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 8.0);

    Ok(())
}

#[test]
fn test_parse_subtraction() -> Result<()> {
    // Arrange
    let input = "x - 3.0";
    let env = Environment::from_tuples(&[("x", 10.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 7.0);

    Ok(())
}

#[test]
fn test_parse_multiplication() -> Result<()> {
    // Arrange
    let input = "2.0 * x";
    let env = Environment::from_tuples(&[("x", 4.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 8.0);

    Ok(())
}

#[test]
fn test_parse_division() -> Result<()> {
    // Arrange
    let input = "x / 2.0";
    let env = Environment::from_tuples(&[("x", 10.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 5.0);

    Ok(())
}

#[test]
fn test_parse_parentheses() -> Result<()> {
    // Arrange
    let input = "(x + 1.0)";
    let env = Environment::from_tuples(&[("x", 5.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 6.0);

    Ok(())
}

#[test]
fn test_parse_with_whitespace() -> Result<()> {
    // Arrange
    let input = "  x  ";
    let env = Environment::from_tuples(&[("x", 7.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 7.0);

    Ok(())
}

#[test]
fn test_parse_empty_string() -> Result<()> {
    // Arrange
    let input = "";

    // Act
    let result = parse(input);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_parse_invalid_syntax() -> Result<()> {
    // Arrange
    let input = "x +";

    // Act
    let result = parse(input);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_parse_unparsed_input_remaining() -> Result<()> {
    // Arrange
    let input = "x 5.0";

    // Act
    let result = parse(input);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_parse_complex_arithmetic_expression() -> Result<()> {
    // Arrange
    let input = "2.0 * x + 3.0";
    let env = Environment::from_tuples(&[("x", 5.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 13.0);

    Ok(())
}

#[test]
fn test_parse_nested_parentheses() -> Result<()> {
    // Arrange
    let input = "((x + 1.0) * 2.0)";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 8.0);

    Ok(())
}

#[test]
fn test_parse_division_by_constant() -> Result<()> {
    // Arrange
    let input = "10.0 / 4.0";
    let env = Environment::empty();

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 2.5);

    Ok(())
}

#[test]
fn test_parse_monomial_zero_exponent() -> Result<()> {
    // Arrange
    let input = "x^0";
    let env = Environment::from_tuples(&[("x", 100.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 1.0);

    Ok(())
}

#[test]
fn test_parse_large_coefficient() -> Result<()> {
    // Arrange
    let input = "1000.5x";
    let env = Environment::from_tuples(&[("x", 2.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 2001.0);

    Ok(())
}

#[test]
fn test_parse_fractional_coefficient() -> Result<()> {
    // Arrange
    let input = "0.5x";
    let env = Environment::from_tuples(&[("x", 10.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 5.0);

    Ok(())
}

#[test]
fn test_parse_multiple_variables() -> Result<()> {
    // Arrange
    let input = "x + y";
    let env = Environment::from_tuples(&[("x", 3.0), ("y", 7.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 10.0);

    Ok(())
}

#[test]
fn test_parse_variable_multiplication() -> Result<()> {
    // Arrange
    let input = "x * y";
    let env = Environment::from_tuples(&[("x", 4.0), ("y", 5.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 20.0);

    Ok(())
}

#[test]
fn test_parse_uppercase_variable() -> Result<()> {
    // Arrange
    let input = "X";
    let env = Environment::from_tuples(&[("X", 15.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 15.0);

    Ok(())
}

#[test]
fn test_parse_mixed_case_variable() -> Result<()> {
    // Arrange
    let input = "MyVar";
    let env = Environment::from_tuples(&[("MyVar", 25.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 25.0);

    Ok(())
}

#[test]
fn test_parse_subtraction_resulting_negative() -> Result<()> {
    // Arrange
    let input = "x - 10.0";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, -7.0);

    Ok(())
}

#[test]
fn test_parse_high_exponent() -> Result<()> {
    // Arrange
    let input = "x^5";
    let env = Environment::from_tuples(&[("x", 2.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 32.0);

    Ok(())
}

#[test]
fn test_parse_parentheses_with_multiplication() -> Result<()> {
    // Arrange
    let input = "(x + 2.0) * 3.0";
    let env = Environment::from_tuples(&[("x", 4.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 18.0);

    Ok(())
}

#[test]
fn test_parse_missing_variable_in_environment() -> Result<()> {
    // Arrange
    let input = "x + y";
    let env = Environment::from_tuples(&[("x", 3.0)]);

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_parse_natural_computation_order() -> Result<()> {
    // Arrange
    let input = "1 + 2 * 3";
    let env = Environment::empty();

    // Act
    let eq = parse(input).map_err(|e| anyhow!("{}", e))?;
    let result = eq.evaluate(&env).map_err(|e| anyhow!("{:?}", e))?;

    // Assert
    assert_relative_eq!(result, 7.0);

    Ok(())
}
