use std::error::Error;

use nom::{
    AsChar, IResult, Parser,
    branch::alt,
    bytes::take_while1,
    character::{char, complete::multispace0, one_of},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::many0,
    number::recognize_float,
    sequence::delimited,
};

use crate::equation::{
    Equation,
    arithmetic::{ArithmeticEquation, Operator},
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

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

fn signed_digit(input: &str) -> IResult<&str, i32> {
    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, digit) = take_while1(|c: char| c.is_dec_digit()).parse(input)?;

    let mut value: i32 = digit.parse().expect("should be parsable");
    if let Some(s) = sign
        && s == '-'
    {
        value = -value;
    }

    Ok((input, value))
}

/// Parse a monomial equation
fn monomial(input: &str) -> IResult<&str, Equation> {
    let coeff = opt(number);
    let exponent = map(opt((char('^'), signed_digit)), |v| v.map(|v| v.1));

    let (input, (coeff, var, exp)) = (coeff, ws(variable), exponent).parse(input)?;

    Ok((
        input,
        MonomialEquation::new(coeff.unwrap_or(1.0), &var, exp.unwrap_or(1_i32)).into(),
    ))
}

fn constant(input: &str) -> IResult<&str, Equation> {
    let (input, value) = ws(number).parse(input)?;

    Ok((input, value.into()))
}

/// parse (...) equation
fn paren_equation(input: &str) -> IResult<&str, Equation> {
    let lparen = ws(char('('));
    let rparen = ws(char(')'));
    let mut eq = delimited(lparen, equation, rparen);

    let (input, eq) = eq.parse(input)?;

    Ok((input, eq))
}

/// Parse equation from string.
fn equation(input: &str) -> IResult<&str, Equation> {
    alt((
        paren_equation,
        high_arithmetic,
        low_arithmetic,
        monomial,
        constant,
    ))
    .parse(input)
}

/// Parse an arithmetic equation with high priority
fn high_arithmetic(input: &str) -> IResult<&str, Equation> {
    let (input, (left, operator, right)) = (equation, ws(high_op), equation).parse(input)?;

    Ok((
        input,
        ArithmeticEquation::new(operator, &[left, right])
            .expect("should be success")
            .into(),
    ))
}

/// Parse an arithmetic equation with preceding priority
fn low_arithmetic(input: &str) -> IResult<&str, Equation> {
    let (input, (left, operator, right)) = (equation, ws(low_op), equation).parse(input)?;

    Ok((
        input,
        ArithmeticEquation::new(operator, &[left, right])
            .expect("should be success")
            .into(),
    ))
}

/// Parse an equation from input string
///
/// # Arguments
/// * `input` - A string slice that holds the equation
///
/// # Returns
/// * `Result<Equation, Box<dyn Error>>` - Parsed Equation or an error
pub fn parse(input: &str) -> Result<Equation, Box<dyn Error>> {
    let (rest, eq) = equation(input).map_err(|e| format!("Parse error: {:?}", e))?;

    if !rest.trim().is_empty() {
        return Err(format!("Unparsed input remaining: {}", rest).into());
    }

    Ok(eq)
}
