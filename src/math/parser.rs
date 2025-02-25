use super::calculator::{CalcError, Expr, Token};
use std::collections::VecDeque;

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Expr, CalcError> {
        Self::parse_expr(tokens)
    }

    fn parse_expr(tokens: &mut VecDeque<Token>) -> Result<Expr, CalcError> {
        let result = Self::parse_add_sub(tokens)?;
        Ok(result)
    }

    fn parse_add_sub(tokens: &mut VecDeque<Token>) -> Result<Expr, CalcError> {
        let mut left = Self::parse_mul_div(tokens)?;

        while let Some(token) = tokens.front() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match tokens.pop_front().unwrap() {
                        Token::Plus => '+',
                        Token::Minus => '-',
                        _ => unreachable!(),
                    };
                    let right = Self::parse_mul_div(tokens)?;
                    left = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(tokens: &mut VecDeque<Token>) -> Result<Expr, CalcError> {
        let mut left = Self::parse_primary(tokens)?;

        while let Some(token) = tokens.front() {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = match tokens.pop_front().unwrap() {
                        Token::Multiply => '*',
                        Token::Divide => '/',
                        _ => unreachable!(),
                    };
                    let right = Self::parse_primary(tokens)?;
                    left = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_primary(tokens: &mut VecDeque<Token>) -> Result<Expr, CalcError> {
        match tokens.pop_front() {
            Some(Token::Plus) => Ok(Expr::UnaryOp {
                op: '+',
                operand: Box::new(Self::parse_primary(tokens)?),
            }),
            Some(Token::Minus) => Ok(Expr::UnaryOp {
                op: '-',
                operand: Box::new(Self::parse_primary(tokens)?),
            }),
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(Token::LeftParen) => {
                let expr = Self::parse_expr(tokens)?;
                match tokens.pop_front() {
                    Some(Token::RightParen) => Ok(expr),
                    Some(_) => Err(CalcError::ParserError("Expected closing parenthesis")),
                    None => Err(CalcError::ParserError("Unexpected end of input")),
                }
            }
            Some(_) => Err(CalcError::ParserError("Expected number or parenthesis")),
            None => Err(CalcError::ParserError("Unexpected end of input")),
        }
    }
}
