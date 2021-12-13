use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::digit1,
    character::complete::multispace0, multi::many0, multi::separated_list1, sequence::delimited,
    sequence::preceded, sequence::separated_pair, sequence::terminated, sequence::tuple, IResult,
};

use crate::ast::AExp;
use crate::ast::BExp;
use crate::ast::Block;
use crate::ast::Pgm;
use crate::ast::Stmt;

fn parenth(input: &str) -> IResult<&str, AExp> {
    delimited(
        multispace0,
        delimited(tag("("), aexpr, tag(")")),
        multispace0,
    )(input)
}

fn var(input: &str) -> IResult<&str, AExp> {
    let (input, var) = delimited(multispace0, alpha1, multispace0)(input)?;
    Ok((input, AExp::Id(var.to_string())))
}
fn int(input: &str) -> IResult<&str, AExp> {
    // let (input, dig) = nom::number::complete::be_i32(input)?;
    // Ok((input, AExp::Int(dig)))
    let (input, num) = delimited(multispace0, digit1, multispace0)(input)?;
    Ok((input, AExp::Int(num.parse().unwrap())))
}

fn not_bin(input: &str) -> IResult<&str, AExp> {
    (alt((parenth, int, var)))(input)
}
fn div_extra(input: &str) -> IResult<&str, Vec<AExp>> {
    many0(preceded(tag("/"), not_bin))(input)
}
fn div(input: &str) -> IResult<&str, AExp> {
    // let (input, exp1) = alt(parenth, todo!());
    let (input, (init, extra)) = tuple((not_bin, div_extra))(input)?;
    Ok((
        input,
        extra
            .into_iter()
            .fold(init, |acc, x| AExp::Divide(Box::new(acc), Box::new(x))),
    ))
}

// plus ::= plus + div_exp | div_exp
// plus ::= div_exp plus'
// plus' ::= (+div_exp) plus' | nothing

fn plus_extra(input: &str) -> IResult<&str, Vec<AExp>> {
    many0(preceded(tag("+"), div))(input)
}
fn plus(input: &str) -> IResult<&str, AExp> {
    let (input, (init, extra)) = tuple((div, plus_extra))(input)?;
    Ok((
        input,
        extra
            .into_iter()
            .fold(init, |acc, x| AExp::Plus(Box::new(acc), Box::new(x))),
    ))
}
fn aexpr(input: &str) -> IResult<&str, AExp> {
    plus(input)
}

fn ltexp(input: &str) -> IResult<&str, BExp> {
    let (input, (left, right)) = separated_pair(aexpr, tag("<="), aexpr)(input)?;
    Ok((input, (BExp::LessThanEq(Box::new(left), Box::new(right)))))
}
fn and_extra(input: &str) -> IResult<&str, Vec<BExp>> {
    many0(preceded(tag("&&"), ltexp))(input)
}

// bexp ::= bexp && ltexp | ltexp | ( bexp ) | ! bexp | bool
// plus ::= div_exp plus'
// plus' ::= (+div_exp) plus' | nothing
//
//

fn parse_true(input: &str) -> IResult<&str, BExp> {
    let (input, _b) = delimited(multispace0, tag("true"), multispace0)(input)?;
    Ok((input, BExp::Bool(true)))
}

fn parse_false(input: &str) -> IResult<&str, BExp> {
    let (input, _) = delimited(multispace0, tag("false"), multispace0)(input)?;
    Ok((input, BExp::Bool(false)))
}
fn bool_const(input: &str) -> IResult<&str, BExp> {
    alt((parse_true, parse_false))(input)
}
fn not(input: &str) -> IResult<&str, BExp> {
    let (input, _) = delimited(multispace0, tag("!"), multispace0)(input)?;
    let (input, b) = bexp(input)?;
    Ok((input, BExp::Negation(Box::new(b))))
}
fn bexp_parens(input: &str) -> IResult<&str, BExp> {
    delimited(
        tag("("),
        delimited(multispace0, bexp, multispace0),
        tag(")"),
    )(input)
}

fn bexp(input: &str) -> IResult<&str, BExp> {
    let (input, (init, extra)) =
        tuple((alt((ltexp, bexp_parens, not, bool_const)), and_extra))(input)?;
    Ok((
        input,
        extra
            .into_iter()
            .fold(init, |acc, x| BExp::And(Box::new(acc), Box::new(x))),
    ))
}

fn open_bracket(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag("{"), multispace0)(input)
}

fn closed_bracket(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag("}"), multispace0)(input)
}

fn seq_list(input: &str) -> IResult<&str, Option<Stmt>> {
    let (input, s) = many0(stmt)(input)?;
    let new_stmt = s
        .into_iter()
        .reduce(|acc, x| Stmt::Sequence(Box::new(acc), Box::new(x)));
    Ok((input, new_stmt))
}
fn block(input: &str) -> IResult<&str, Block> {
    let (input, s) = delimited(open_bracket, seq_list, closed_bracket)(input)?;

    let new_block = match s {
        Some(x) => Block::BlockStmt(Box::new(x)),
        None => Block::EmptyBlock,
    };

    Ok((input, (new_block)))
}

fn semicolon(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag(";"), multispace0)(input)
}
fn assign(input: &str) -> IResult<&str, Stmt> {
    let (input, (v, s)) = terminated(separated_pair(var, tag("="), aexpr), semicolon)(input)?;
    Ok((input, Stmt::Assign(v.to_string(), Box::new(s))))
}
fn ifthenelse(input: &str) -> IResult<&str, Stmt> {
    let (input, (_, b, s1, _, s2)) = tuple((tag("if"), bexp, block, tag("else"), block))(input)?;
    Ok((
        input,
        Stmt::IfThenElse(Box::new(b), Box::new(s1), Box::new(s2)),
    ))
}

fn while_loop(input: &str) -> IResult<&str, Stmt> {
    let (input, (_, b, s)) = tuple((tag("while"), bexp, block))(input)?;
    Ok((input, Stmt::While(Box::new(b), Box::new(s))))
}

fn stmt(input: &str) -> IResult<&str, Stmt> {
    alt((assign, ifthenelse, while_loop))(input)
}

fn pgm(input: &str) -> IResult<&str, Pgm> {
    let (input, (_, vs, _, s)) = tuple((
        tag("int"),
        separated_list1(tag(","), var),
        semicolon,
        seq_list,
    ))(input)?;
    let s = match s {
        Some(s) => s,
        None => Stmt::StmtBlock(Box::new(Block::EmptyBlock)),
    };
    Ok((
        input,
        Pgm::Program(vs.iter().map(|x| x.to_string()).collect(), s),
    ))
}

pub fn parse(input: String) -> Option<Pgm> {
    match delimited(multispace0, pgm, multispace0)(&input) {
        Ok(("", x)) => Some(x),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arith1() {
        match aexpr("1") {
            Ok((_, AExp::Int(1))) => {}
            _ => panic!(),
        };
    }

    #[test]
    fn test_arith2() {
        match aexpr("1 + 2") {
            Ok((_, AExp::Plus(x, y))) => match (*x, *y) {
                (AExp::Int(1), AExp::Int(2)) => {}
                _ => panic!(),
            },
            _ => panic!(),
        };
    }
    #[test]
    fn test_arith3() {
        match aexpr("1 + 2 + 3") {
            Ok((_, AExp::Plus(x, y))) => match (*x, *y) {
                (AExp::Plus(x, y), AExp::Int(3)) => match (*x, *y) {
                    (AExp::Int(1), AExp::Int(2)) => {}
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        };
    }
    #[test]
    fn test_arith4() {
        match aexpr("1 / 3 + (2 + x ) + 3 / 1") {
            Ok((_, AExp::Plus(x, y))) => {
                match *y {
                    AExp::Divide(x, y) => match (*x, *y) {
                        (AExp::Int(3), AExp::Int(1)) => {}
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
                match *x {
                    AExp::Plus(x, y) => {
                        match *y {
                            AExp::Plus(x, y) => match (*x, *y) {
                                (AExp::Int(2), AExp::Id(z)) if z == "x" => {}
                                _ => panic!(),
                            },
                            _ => panic!(),
                        }
                        match *x {
                            AExp::Divide(x, y) => match (*x, *y) {
                                (AExp::Int(1), AExp::Int(3)) => {}
                                _ => panic!(),
                            },
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_stmt2() {
        match seq_list("x = 1 ; x = 1; ") {
            Ok((_, Some(Stmt::Sequence(s1, s2)))) => {
                match *s1 {
                    Stmt::Assign(v, n) => {
                        if v == "x" {
                            match *n {
                                AExp::Int(1) => {}
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                }
                match *s2 {
                    Stmt::Assign(v, n) => {
                        if v == "x" {
                            match *n {
                                AExp::Int(1) => {}
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                }
            }

            _ => panic!(),
        };
    }
    #[test]
    fn test_stmt1() {
        match stmt("x = 1 ; ") {
            Ok((_, Stmt::Assign(v, n))) => {
                if v == "x" {
                    match *n {
                        AExp::Int(1) => {}
                        _ => panic!(),
                    }
                }
            }

            _ => panic!(),
        };
    }
    #[test]
    fn test_pgm1() {
        match parse("int x, y; x = 1 ; ".to_string()) {
            Some(Pgm::Program(v, s)) if v == vec!["x".to_string(), "y".to_string()] => match s {
                Stmt::Assign(v, n) => {
                    if v == "x" {
                        match *n {
                            AExp::Int(1) => {}
                            _ => panic!(),
                        }
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        };
    }
}
