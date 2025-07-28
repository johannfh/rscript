use derive_more::{Display, Error, From};
use logos::{Lexer, Logos};

use self::{
    ast::{
        BinaryOp, BinaryOperator, Expression, FloatLiteral, FunctionDeclaration, Identifier,
        IntegerLiteral, NamedFieldDeclaration, Parameter, Program, ReturnStatement, Statement,
        StructDeclaration, TupleFieldDeclaration, VariableDeclaration,
    },
    lexer::{LexerError, Token},
};
use crate::core::span::{Span, Spanned};

mod ast;
mod lexer;
mod format;

#[derive(Debug, From, PartialEq, Display, Error)]
pub enum ParserError {
    #[display("{_0}")]
    LexerError(LexerError),
    #[display(
        "unexpected token, expected: {}, found: {:?}, span: {:?}",
        expected,
        found,
        span
    )]
    UnexpectedToken {
        expected: String,
        found: Option<Token>,
        span: Span,
    },
    #[display("unexpected end of file")]
    UnexpectedEof,
}

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current: Option<(Token, Span)>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        trace!("Creating new lexer");
        let lexer = Lexer::new(input);
        for token in lexer.clone() {
            trace!("Token: {:?}", token);
        }
        trace!("Creating new parser");
        Parser {
            lexer,
            current: None,
        }
    }

    /// Advances the [`Parser`] to the next [`Token`] and updates the `current` value.
    fn advance(&mut self) -> Result<(), LexerError> {
        if let Some(token) = self.lexer.next() {
            let token = token?;
            let span = self.current_span();
            self.current = Some((token, span));
        } else {
            self.current = None;
        }

        Ok(())
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref().map(|v| &v.0)
    }

    fn consume(&mut self, expected: Token) -> Result<Span, ParserError> {
        if let Some((token, span)) = self.current.clone() {
            if token == expected {
                self.advance()?;
                Ok(span.clone())
            } else {
                Err(ParserError::UnexpectedToken {
                    expected: format!("{:?}", expected),
                    found: Some(token),
                    span: span,
                })
            }
        } else {
            Err(ParserError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: None,
                span: self.current_span(),
            })
        }
    }

    fn consume_identifier(&mut self) -> Result<Identifier, ParserError> {
        if let Some((Token::Identifier(name), span)) = self.current.as_ref().cloned() {
            self.advance()?;
            Ok(Identifier { name, span })
        } else {
            Err(ParserError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self.peek().cloned(),
                span: self.current_span(),
            })
        }
    }

    pub fn parse(mut self) -> Result<Program, ParserError> {
        trace!("Parsing program");
        let program_start_span = self.current_span().start;
        let mut statements = Vec::new();
        self.advance()?;
        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }

        let program_end_span = self.current_span().end;

        info!("Successfully parsed program");
        Ok(Program {
            statements,
            span: Span {
                start: program_start_span,
                end: program_end_span,
            },
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        trace!("Parsing statement");
        match self.peek() {
            Some(Token::Let) => self.parse_variable_declaration().map(Into::into),
            Some(Token::Fn) => self.parse_function_declaration().map(Into::into),
            Some(Token::Struct) => self.parse_struct_declaration().map(Into::into),
            Some(Token::Return) => self.parse_return_statement().map(Into::into),
            Some(other) => todo!("parse: {:#?}", other),
            None => Err(ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: None,
                span: self.current_span(),
            }),
        }
    }

    fn current_span(&self) -> Span {
        self.lexer.span().into()
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ParserError> {
        trace!("Parsing variable declaration");
        let start_span = self.consume(Token::Let)?.start;
        let identifier = self.consume_identifier()?;
        // TODO: Parse Type if Token::Colon
        let _ = self.consume(Token::Assign)?;
        let initializer = self.parse_expression()?;
        let end_span = self.consume(Token::Semicolon)?.end;
        Ok(VariableDeclaration {
            identifier,
            initializer,
            span: Span {
                start: start_span,
                end: end_span,
            },
        })
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParserError> {
        trace!("Parsing function declaration");
        let start_span = self.consume(Token::Fn)?.start;
        let identifier = self.consume_identifier()?;
        let _ = self.consume(Token::LParen)?;

        // -- Parse Parameters --
        let mut parameters = Vec::new();
        loop {
            if let Some(Token::Identifier(name)) = self.peek() {
                let identifier = self.consume_identifier()?;
                let _ = self.consume(Token::Colon)?;
                let declared_type = self.consume_identifier()?;
                let span = Span {
                    start: identifier.span.start,
                    end: declared_type.span.end,
                };
                parameters.push(Parameter {
                    identifier,
                    declared_type,
                    span,
                });
            } else {
                break;
            }
        }

        let _ = self.consume(Token::RParen)?;

        let _ = self.consume(Token::RightArrow)?;

        // -- Parse Return Type --
        let return_type = self.consume_identifier()?;

        let _ = self.consume(Token::LBrace)?;

        // -- Parse Body --
        let mut body = Vec::new();

        while self.peek() != Some(&Token::RBrace) {
            body.push(self.parse_statement()?);
        }

        let end_span = self.consume(Token::RBrace)?.end;

        let span = Span {
            start: start_span,
            end: end_span,
        };

        Ok(FunctionDeclaration {
            identifier,
            parameters,
            return_type,
            body,
            span,
        })
    }

    fn parse_struct_declaration(&mut self) -> Result<StructDeclaration, ParserError> {
        trace!("Parsing struct declaration");
        let start_span = self.consume(Token::Struct)?.start;

        // -- Parse Identifier --
        let identifier = self.consume_identifier()?;

        // -- Parse Fields --
        match self.current.as_ref().cloned() {
            // -- Tuple Fields --
            Some((Token::LParen, _)) => {
                trace!("Matched tuple struct");
                let mut fields = Vec::new();
                loop {
                    self.advance()?;
                    match self.current.as_ref().cloned() {
                        // -- End of Fields --
                        Some((Token::RParen, _)) => break,
                        // -- Comma -> Next Field --
                        Some((Token::Comma, _)) => {
                            trace!("Found comma, expecting next field");
                            continue;
                        }
                        // -- Next Field --
                        Some((Token::Identifier(name), span)) => {
                            let declared_type = Identifier { name, span };
                            trace!("Found tuple field: {:?}", declared_type);
                            fields.push(TupleFieldDeclaration {
                                declared_type,
                                span,
                            });
                        }
                        Some((other, span)) => {
                            return Err(ParserError::UnexpectedToken {
                                expected: "`)` or identifier".to_string(),
                                found: Some(other),
                                span,
                            });
                        }
                        None => return Err(ParserError::UnexpectedEof),
                    }
                }

                self.consume(Token::RParen)?;

                let end_span = self.consume(Token::Semicolon)?.end;

                let span = Span {
                    start: start_span,
                    end: end_span,
                };

                Ok(StructDeclaration::TupleStruct {
                    identifier,
                    fields,
                    span,
                })
            }
            // -- Named Fields --
            Some((Token::LBrace, _)) => {
                trace!("Matched named fields struct");
                let mut fields = Vec::new();
                let end_span = loop {
                    self.advance()?;
                    match self.current.as_ref().cloned() {
                        // -- End of Fields --
                        Some((Token::RBrace, span)) => break span.end,
                        // -- Next Field --
                        Some((Token::Identifier(_), span)) => {
                            let identifier = self.consume_identifier()?;
                            self.consume(Token::Colon)?;
                            let declared_type = self.consume_identifier()?;
                            fields.push(NamedFieldDeclaration {
                                identifier,
                                declared_type,
                                span,
                            });
                        }
                        Some((other, span)) => {
                            return Err(ParserError::UnexpectedToken {
                                expected: "`)` or identifier".to_string(),
                                found: Some(other),
                                span,
                            });
                        }
                        None => return Err(ParserError::UnexpectedEof),
                    }
                };

                self.consume(Token::RBrace)?;

                let span = Span {
                    start: start_span,
                    end: end_span,
                };

                Ok(StructDeclaration::NamedStruct {
                    identifier,
                    fields,
                    span,
                })
            }
            // -- Unit Struct --
            Some((Token::Semicolon, span)) => {
                trace!("Matched unit struct");
                let end_span = span.end;

                let span = Span {
                    start: start_span,
                    end: end_span,
                };

                self.advance()?;
                Ok(StructDeclaration::UnitStruct { identifier, span })
            }
            Some((other, span)) => Err(ParserError::UnexpectedToken {
                expected: "`(` or `{` or `;`".to_string(),
                found: Some(other),
                span: span,
            }),
            None => Err(ParserError::UnexpectedEof),
        }
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserError> {
        trace!("Parsing return statement");
        let span = self.consume(Token::Return)?;

        if self.peek() == Some(&Token::Semicolon) {
            Ok(ReturnStatement { value: None, span })
        } else {
            let expression = self.parse_expression()?;
            let span = span.combine(expression.span());
            let _ = self.consume(Token::Semicolon)?;

            Ok(ReturnStatement {
                value: Some(expression),
                span,
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        trace!("Parsing expression");
        let first: Expression = match self.peek() {
            Some(&Token::IntegerLiteral(_)) => self.parse_integer_literal().map(Into::into)?,
            Some(&Token::FloatLiteral(_)) => self.parse_float_literal().map(Into::into)?,
            Some(&Token::Identifier(_)) => self.consume_identifier().map(Into::into)?,
            other => todo!("got expression: {:?}", other),
        };

        let is_binary_op = matches!(
            self.current.as_ref().map(|v| &v.0),
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Star) | Some(Token::Slash)
        );

        if is_binary_op {
            trace!("Parsing binary operation");
            match self.current.as_ref() {
                Some((Token::Plus, _)) => {
                    trace!("Parsing plus operation");
                    let _ = self.consume(Token::Plus)?;
                    let second = self.parse_expression()?;
                    let span = first.span().combine(second.span());
                    return Ok(BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(first),
                        right: Box::new(second),
                        span,
                        inferred_type: None,
                    }
                    .into());
                }
                Some((Token::Star, _)) => {
                    trace!("Parsing star operation");
                    let _ = self.consume(Token::Star)?;
                    let second = self.parse_expression()?;
                    let span = first.span().combine(second.span());
                    return Ok(BinaryOp {
                        operator: BinaryOperator::Multiply,
                        left: Box::new(first),
                        right: Box::new(second),
                        span,
                        inferred_type: None,
                    }
                    .into());
                }
                other => todo!("binary operation {:?} not implemented yet", other),
            }
        }
        Ok(first)
    }

    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, ParserError> {
        trace!("Parsing integer literal");
        match self.current.as_ref() {
            Some(&(Token::IntegerLiteral(value), span)) => {
                self.advance()?;
                Ok(IntegerLiteral { value, span })
            }
            other => todo!("unexpected token for integer literal: {:?}", other),
        }
    }

    fn parse_float_literal(&mut self) -> Result<FloatLiteral, ParserError> {
        trace!("Parsing float literal");
        match self.current.as_ref() {
            Some(&(Token::FloatLiteral(value), span)) => {
                self.advance()?;
                Ok(FloatLiteral { value, span })
            }
            other => todo!("unexpected token for float literal: {:?}", other),
        }
    }
}
