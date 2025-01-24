use super::calculator::{CalcError, Token};
use std::collections::VecDeque;

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(expr: &str) -> Result<VecDeque<Token>, CalcError> {
        let mut chars = expr.chars().peekable();
        let mut tokens = VecDeque::new();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    let mut num_str = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let num = num_str
                        .parse::<f64>()
                        .map_err(|_| CalcError::PaserError("Failed to parse number"))?;
                    tokens.push_back(Token::Number(num));
                }
                '+' => {
                    tokens.push_back(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push_back(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push_back(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push_back(Token::Divide);
                    chars.next();
                }
                '(' => {
                    tokens.push_back(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push_back(Token::RightParen);
                    chars.next();
                }
                ' ' => {
                    chars.next();
                }
                _ => return Err(CalcError::InvalidChar(c, chars.count())),
            }
        }

        Ok(tokens)
    }
}
