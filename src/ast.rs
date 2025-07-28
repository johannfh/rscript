use derive_more::{Display, From};

use crate::span::{Span, Spanned};

#[derive(Debug, PartialEq, Clone, From)]
pub enum Expression {
    BinaryOp(BinaryOp),
    FunctionCall(FunctionCall),
    BlockExpression(BlockExpression),
    IfExpression(IfExpression),
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
}

impl Spanned for Expression {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Expression::BinaryOp(node) => node.span,
            Expression::FunctionCall(node) => node.span,
            Expression::BlockExpression(node) => node.span,
            Expression::IfExpression(node) => node.span,
            Expression::Identifier(node) => node.span,
            Expression::IntegerLiteral(node) => node.span,
            Expression::FloatLiteral(node) => node.span,
            Expression::StringLiteral(node) => node.span,
            Expression::BooleanLiteral(node) => node.span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub value: i64,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FloatLiteral {
    pub value: f64,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOp {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
    pub inferred_type: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    And,
    Or,
    // TODO: More operators, for example:
    // bit operators: << & | ^
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub function_name: Identifier,
    pub arguments: Vec<Expression>,
    pub span: Span,
    pub inferred_type: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
    pub final_expression: Option<Box<Expression>>,
    pub inferred_type: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub then_branch: BlockExpression,
    pub else_branch: Option<BlockExpression>,
    /// Must be consistent between branches
    pub inferred_type: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, From)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructDeclaration),
    ExpressionStatement(ExpressionStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::VariableDeclaration(node) => node.span,
            Statement::FunctionDeclaration(node) => node.span,
            Statement::StructDeclaration(node) => match *node {
                StructDeclaration::NamedStruct { span, .. } => span,
                StructDeclaration::TupleStruct { span, .. } => span,
                StructDeclaration::UnitStruct { span, .. } => span,
            },
            Statement::ExpressionStatement(node) => node.span,
            Statement::ReturnStatement(node) => node.span,
            Statement::BreakStatement(node) => node.span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub initializer: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub identifier: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Identifier,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub identifier: Identifier,
    pub declared_type: Identifier,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructDeclaration {
    NamedStruct {
        identifier: Identifier,
        fields: Vec<NamedFieldDeclaration>,
        span: Span,
    },
    TupleStruct {
        identifier: Identifier,
        fields: Vec<TupleFieldDeclaration>,
        span: Span,
    },
    UnitStruct {
        identifier: Identifier,
        span: Span,
    },
}

impl Spanned for StructDeclaration {
    fn span(&self) -> Span {
        match self {
            StructDeclaration::NamedStruct { span, .. } => *span,
            StructDeclaration::TupleStruct { span, .. } => *span,
            StructDeclaration::UnitStruct { span, .. } => *span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedFieldDeclaration {
    pub identifier: Identifier,
    pub declared_type: Identifier,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleFieldDeclaration {
    pub declared_type: Identifier,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}
