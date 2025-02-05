use super::parser::Parser;
use super::tokenizer::Tokenizer;
use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalcError {
    #[error("Invalid character: '{0}' at position {1}")]
    InvalidChar(char, usize),

    #[error("Division by zero in expression: {0}")]
    DivisionByZero(String),

    #[error("Parser error: {0}")]
    ParserError(&'static str),

    #[error("Evaluating error: {0}")]
    EvalError(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: char,
        operand: Box<Expr>,
    },
}

pub fn calculate(expr: &str) -> Result<f64> {
    let mut tokens =
        Tokenizer::tokenize(expr).map_err(|e| anyhow!("Failed to parse expression: {}", e))?;
    let ast =
        Parser::parse(&mut tokens).map_err(|e| anyhow!("Failed to parse expression: {}", e))?;
    eval(&ast).map_err(|e| anyhow!("Failed to evaluate expression: {}", e))
}

fn eval(expr: &Expr) -> Result<f64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::UnaryOp { op, operand } => {
            let val = eval(operand)?;
            match op {
                '+' => Ok(val),  //一元加法
                '-' => Ok(-val), //一元减法
                _ => Err(CalcError::EvalError(format!("Invalid operator: {}", op))),
            }
        }
        Expr::BinaryOp { op, left, right } => {
            let (left_val, right_val) = (eval(left)?, eval(right)?);
            match op {
                '+' => Ok(left_val + right_val),
                '-' => Ok(left_val - right_val),
                '*' => Ok(left_val * right_val),
                '/' if right_val == 0.0 => {
                    Err(CalcError::DivisionByZero("Division by zero".to_string()))
                }
                '/' => Ok(left_val / right_val),
                _ => Err(CalcError::EvalError(format!("Invalid operator: {}", op))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        assert_eq!(calculate("3+2").unwrap(), 5.0);
    }

    #[test]
    fn test_expression_with_spaces() {
        assert_eq!(calculate("3 + 2 * 4").unwrap(), 11.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(calculate("(3+2)*4").unwrap(), 20.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate("6/2").unwrap(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calculate("6/0");
        assert!(result.is_err());
        // 使用 assert_eq 来比较错误信息
        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "Failed to evaluate expression: Division by zero in expression: Division by zero"
            );
        } else {
            panic!("Expected an error, but got a result: {:?}", result);
        }
    }

    #[test]
    fn test_invalid_character() {
        let result = calculate("3$2");
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                format!(
                    "Failed to parse expression: Invalid character: '{}' at position {}",
                    '$', 2
                )
            );
        } else {
            panic!("Expected an error, but got a result: {:?}", result);
        }
    }

    #[test]
    fn test_missing_parenthesis() {
        let result = calculate("(3+2");
        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "Failed to parse expression: Parser error: Expected closing parenthesis"
            );
        } else {
            panic!("Expected an error, but got a result: {:?}", result);
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(calculate("10 + 10").unwrap(), 20.0)
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(calculate("-10").unwrap(), -10.0);
    }
    #[test]
    fn test_unary_plus() {
        assert_eq!(calculate("+10").unwrap(), 10.0);
    }

    #[test]
    fn test_unary_with_binary() {
        assert_eq!(calculate("-2 * 3").unwrap(), -6.0);
        assert_eq!(calculate("-(2 + 3)").unwrap(), -5.0);
    }

    #[test]
    fn test_multiple_unary() {
        assert_eq!(calculate("--10").unwrap(), 10.0);
        assert_eq!(calculate("+-10").unwrap(), -10.0);
    }
}
