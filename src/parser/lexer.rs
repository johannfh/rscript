use std::num::{ParseFloatError, ParseIntError};

use derive_more::{Display, Error, From};
use logos::Logos;

pub type Result = std::result::Result<Token, LexerError>;

#[derive(Debug, Clone, PartialEq, Default, From, Display, Error)]
pub enum LexerError {
    #[display("{_0}")]
    ParseIntError(ParseIntError),
    #[display("{_0}")]
    ParseFloatError(ParseFloatError),
    #[default]
    Other,
}

#[derive(Logos, Debug, PartialEq, Clone)]
// skip whitespace
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexerError)]
pub enum Token {
    // -- Keywords --
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("type")]
    Type,
    #[token("struct")]
    Struct,
    #[token("fn")]
    Fn,
    #[token("while")]
    While,
    #[token("loop")]
    Loop,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,

    // -- Operators --
    #[token("+")]
    /// `+`
    Plus,
    #[token("-")]
    /// `-`
    Minus,
    #[token("*")]
    /// `*`
    Star,
    #[token("/")]
    /// `/`
    Slash,
    #[token("=")]
    /// `=`
    Assign,
    #[token("==")]
    /// `==`
    Equals,
    #[token("!=")]
    /// `!=`
    NotEquals,
    #[token("<")]
    /// `<`
    LessThan,
    #[token(">")]
    /// `>`
    GreaterThan,
    #[token("&&")]
    /// `&&`
    And,
    #[token("||")]
    /// `||`
    Or,

    // -- Delimiters --
    /// `(`
    #[token("(")]
    LParen,
    #[token(")")]
    /// `)`
    RParen,
    #[token("{")]
    /// `{`
    LBrace,
    #[token("}")]
    /// `}`
    RBrace,
    #[token("[")]
    /// `[`
    LBracket,
    #[token("]")]
    /// `]`
    RBracket,
    #[token(";")]
    /// `;`
    Semicolon,
    #[token(":")]
    /// `:`
    Colon,
    #[token(",")]
    /// `,`
    Comma,
    #[token(".")]
    /// `.`
    Period,
    #[token("->")]
    /// `->`
    RightArrow,

    // -- Identifier --
    #[regex("([a-zA-Z_][a-zA-Z0-9_]*)", |lex| lex.slice().to_string())]
    Identifier(String),

    // -- Literals --
    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>())]
    IntegerLiteral(i64),

    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f64>())]
    FloatLiteral(f64),

    // RegExp:
    // Anything inside quotes:
    // | Except for " and \
    // OR
    // | \ followed by a single character
    // TODO: Maybe the heavy-lifting should be done in [`parse_string_literal`] for better error
    // handling and more tolerant/flexible string matching. I.e. still matching incorrect escape
    // sequences but showing an error for them. (Incorrect escape sequence => still a String)
    #[regex("\"([^\"\\\\]|\\\\.)*\"", |lex| parse_string_literal(lex.slice()))]
    String(String),
}

fn parse_string_literal(lexed_slice: &str) -> Option<String> {
    // TODO: Parse escape sequences into real string
    return Some(lexed_slice.to_string());
}
