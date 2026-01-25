use std::error::Error;

use anyhow::Result;
use nom::{
    AsChar, Finish, IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace0, one_of},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::many0,
    number::complete::recognize_float,
    sequence::delimited,
};

use crate::equation::{
    Equation,
    arithmetic::{ArithmeticEquation, Operator},
    monomial::MonomialEquation,
};

#[derive(Debug, Clone)]
enum Syntax {
    // direct representation for constant/monomial
    Constant(Equation),
    Monomial(Equation),
    // operator should be construct with a equation
    Op(Operator),
    Paren(Vec<Syntax>),
}

fn number(input: &str) -> IResult<&str, f32> {
    let (input, f) = recognize_float(input)?;
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
fn op(input: &str) -> IResult<&str, Operator> {
    map(one_of("-+*/"), |v| match v {
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
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
fn monomial(input: &str) -> IResult<&str, Syntax> {
    let coeff = opt(number);
    let exponent = map(opt((char('^'), signed_digit)), |v| v.map(|v| v.1));

    let (input, (coeff, var, exp)) = (coeff, ws(variable), exponent).parse(input)?;

    Ok((
        input,
        Syntax::Monomial(
            MonomialEquation::new(coeff.unwrap_or(1.0), &var, exp.unwrap_or(1_i32)).into(),
        ),
    ))
}

fn constant(input: &str) -> IResult<&str, Syntax> {
    let (input, value) = ws(number).parse(input)?;

    Ok((input, Syntax::Constant(value.into())))
}

/// parse (...) equation
fn paren_syntax(input: &str) -> IResult<&str, Syntax> {
    let lparen = ws(char('('));
    let rparen = ws(char(')'));
    let mut eq = delimited(lparen, (equation, many0(equation_with_op)), rparen);

    let (input, (op, ops)) = eq.parse(input)?;
    let ret = vec![vec![op], ops.into_iter().flatten().collect()]
        .into_iter()
        .flatten()
        .collect();

    Ok((input, Syntax::Paren(ret)))
}

/// Parse equation from string.
fn equation(input: &str) -> IResult<&str, Syntax> {
    alt((paren_syntax, monomial, constant)).parse(input)
}

/// Parse equation from string.
fn equation_with_op(input: &str) -> IResult<&str, Vec<Syntax>> {
    let (input, (op, syntax)) = (ws(op), equation).parse(input)?;

    Ok((input, vec![Syntax::Op(op), syntax]))
}

/// Construct an equation with parsed syntaxs.
fn construct_equation(syntax: &[Syntax]) -> Result<Equation> {
    let Some((first, rest)) = syntax.split_first() else {
        return Err(anyhow::anyhow!("Must not empty"));
    };

    fn to_eq(v: &Syntax) -> Equation {
        match v {
            Syntax::Constant(v) => v.clone(),
            Syntax::Monomial(v) => v.clone(),
            Syntax::Paren(v) => construct_equation(v).unwrap(),
            Syntax::Op(_) => unreachable!("This case is parse error"),
        }
    }

    let first: Equation = to_eq(first);

    // short cut to stop infinite recursion
    if rest.is_empty() {
        return Ok(first);
    }

    fn make_tree(syntax: &[Syntax]) -> Equation {
        if syntax.len() == 1 {
            return to_eq(&syntax[0]);
        }
        let mut ordered_ops = syntax
            .iter()
            .enumerate()
            .filter_map(|(i, v)| match v {
                Syntax::Constant(_) => None,
                Syntax::Monomial(_) => None,
                Syntax::Op(operator) => Some((i, operator)),
                Syntax::Paren(_) => None,
            })
            .collect::<Vec<_>>();

        // ordered by less-operator, and greater(right most) index.
        ordered_ops.sort_by(|(idx1, v1), (idx2, v2)| v1.cmp(v2).then(idx1.cmp(idx2).reverse()));

        let (first, rest) = syntax.split_at(ordered_ops[0].0);

        ArithmeticEquation::new(
            *ordered_ops[0].1,
            &[make_tree(first), make_tree(&rest[1..])],
        )
        .expect("Should be convertable")
        .into()
    }

    Ok(make_tree(syntax))
}

/// Parse an equation from input string
///
/// # Arguments
/// * `input` - A string slice that holds the equation
///
/// # Returns
/// * `Result<Equation, Box<dyn Error>>` - Parsed Equation or an error
pub fn parse(input: &str) -> Result<Equation, Box<dyn Error + '_>> {
    let (rest, (syntax, syntaxes)) = (equation, many0(equation_with_op))
        .parse(input)
        .finish()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    if !rest.trim().is_empty() {
        return Err(format!("Unparsed input remaining: {}", rest).into());
    }

    let syntaxes: Vec<Syntax> = vec![vec![syntax], syntaxes.into_iter().flatten().collect()]
        .into_iter()
        .flatten()
        .collect();
    let eq = construct_equation(&syntaxes)?;

    Ok(eq)
}

#[cfg(test)]
mod tests;
