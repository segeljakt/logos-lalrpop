use logos::Lexer as LogosLexer;
use logos::Logos;
use std::num::ParseIntError;
use std::ops::Range;

mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

use parser::ExprParser;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    #[error]
    Error,
    #[token("+")]
    Plus,
    #[regex(r"-?([0-9]+)")]
    Number,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Plus,
    Number(i32),
}

pub type Span = Range<usize>;

pub enum Error {
    InvalidToken(Span),
    InvalidInt(Span, ParseIntError),
}

struct Lexer<'i> {
    logos: LogosLexer<'i, LogosToken>,
    errors: Vec<Error>,
}

impl Iterator for Lexer<'_> {
    type Item = (usize, Token, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let token = loop {
            let token = self.logos.next()?;
            match self.convert(token) {
                Ok(token) => break token,
                Err(err) => self.errors.push(err),
            }
        };
        let span = self.logos.span();
        return Some((span.start, token, span.end));
    }
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            errors: Vec::new(),
            logos: LogosLexer::new(input),
        }
    }

    /// Convert a LogosToken to a Token.
    fn convert(&mut self, token: LogosToken) -> Result<Token, Error> {
        match token {
            LogosToken::Error => Err(Error::InvalidToken(self.logos.span())),
            LogosToken::Plus => Ok(Token::Plus),
            LogosToken::Number => match self.logos.slice().parse() {
                Ok(v) => Ok(Token::Number(v)),
                Err(e) => Err(Error::InvalidInt(self.logos.span(), e)),
            },
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("1+2+3+4+100");
    let mut errors = Vec::new();
    let expr = ExprParser::new().parse(&mut errors, &mut lexer).unwrap();
    errors.append(&mut lexer.errors);

    println!("{:?}", expr);
}
