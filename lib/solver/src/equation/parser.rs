use nom::{
    AsChar, IResult, Or, Parser as _,
    branch::alt,
    bytes::take_while1,
    character::{char, is_bin_digit, one_of},
    combinator::{map, opt, recognize},
    error::ErrorKind,
    multi::{many, many0, many1},
    number::recognize_float,
    sequence::preceded,
};

use crate::equation::{
    Equation,
    arithmetic::{ArithmeticEquation, Operator},
    constant::ConstantEquation,
    monomial::MonomialEquation,
};

fn number(input: &str) -> IResult<&str, f32> {
    let (input, f) = recognize_float().parse(input)?;
    let v: f32 = f.parse().expect("should be parsable");

    Ok((input, v))
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUM: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Parse a variable name
fn variable(input: &str) -> IResult<&str, String> {
    // variable name as
    let name = (ALPHANUM.to_string() + "_").to_owned();
    let char = one_of(name.as_str());
    let mut parser = recognize((one_of(ALPHA), many0(char)));

    let (input, v): (&str, &str) = parser.parse(input)?;
    Ok((input, v.to_string()))
}

/// Parse an operator
fn high_op(input: &str) -> IResult<&str, Operator> {
    map(one_of("*/"), |v| match v {
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
        _ => unreachable!("do not come here!"),
    })
    .parse(input)
}

/// Parse an operator
fn low_op(input: &str) -> IResult<&str, Operator> {
    map(one_of("-+"), |v| match v {
        '-' => Operator::Subtract,
        '+' => Operator::Add,
        _ => unreachable!("do not come here!"),
    })
    .parse(input)
}

fn signed_digit(input: &str) -> IResult<&str, i32> {
    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, digit) = take_while1(|c: char| c.is_dec_digit()).parse(input)?;

    let mut value: i32 = digit.parse().expect("should be parsable");
    if let Some(s) = sign {
        if s == '-' {
            value = -value;
        }
    }

    Ok((input, value))
}

/// Parse a monomial equation
fn monomial(input: &str) -> IResult<&str, Equation> {
    let coeff = opt(number);
    let exponent = map(opt((char('^'), signed_digit)), |v| match v {
        Some(v) => Some(v.1),
        None => None,
    });

    let (input, (coeff, var, exp)) = (coeff, variable, exponent).parse(input)?;

    Ok((
        input,
        MonomialEquation::new(coeff.unwrap_or(1.0), &var, exp.unwrap_or(1_i32)).into(),
    ))
}

fn constant(input: &str) -> IResult<&str, Equation> {
    let (input, value) = number(input)?;

    Ok((input, value.into()))
}

/// parse (...) equation
fn paren_equation(input: &str) -> IResult<&str, Equation> {
    let (input, _) = char('(').parse(input)?;
    let (input, eq) = equation(input)?;
    let (input, _) = char(')').parse(input)?;

    Ok((input, eq.into()))
}

/// Parse equation from string.
pub fn equation(input: &str) -> IResult<&str, Equation> {
    alt((
        paren_equation,
        high_arithmetic,
        arithmetic,
        monomial,
        constant,
    ))
    .parse(input)
}

/// Parse an arithmetic equation
fn high_arithmetic(input: &str) -> IResult<&str, Equation> {
    let (input, (left, operator, right)) = (equation, high_op, equation).parse(input)?;

    Ok((
        input,
        ArithmeticEquation::new(operator, &vec![left, right])
            .expect("should be success")
            .into(),
    ))
}

/// Parse an arithmetic equation
fn arithmetic(input: &str) -> IResult<&str, Equation> {
    let (input, (left, operator, right)) = (equation, low_op, equation).parse(input)?;

    Ok((
        input,
        ArithmeticEquation::new(operator, &vec![left, right])
            .expect("should be success")
            .into(),
    ))
}
