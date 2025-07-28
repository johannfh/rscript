use std::io::{self, Write};

use termcolor::{Color, ColorSpec, WriteColor};

use crate::ast::{
    BinaryOp, BooleanLiteral, Expression, FloatLiteral, FunctionDeclaration, Identifier, IntegerLiteral, NamedFieldDeclaration, Parameter, Program, Statement, StringLiteral, StructDeclaration, TupleFieldDeclaration, VariableDeclaration
};

fn bracket_theme<W>(stdout: &mut W) -> io::Result<()>
where
    W: Write + WriteColor,
{
    const BRACKET_COLOR: Color = Color::Rgb(80, 80, 255);
    stdout.reset()?;
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(BRACKET_COLOR))
            .set_italic(false)
            .set_bold(true),
    )
}

fn node_theme<W>(stdout: &mut W) -> io::Result<()>
where
    W: Write + WriteColor,
{
    const NODE_COLOR: Color = Color::Rgb(200, 120, 120);
    stdout.reset()?;
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(NODE_COLOR))
            .set_italic(false)
            .set_bold(true)
            .set_underline(true),
    )
}

fn span_theme<W>(stdout: &mut W) -> io::Result<()>
where
    W: Write + WriteColor,
{
    const SPAN_COLOR: Color = Color::Cyan;
    stdout.reset()?;
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(SPAN_COLOR))
            .set_italic(true)
            .set_bold(false),
    )
}

fn property_theme<W>(stdout: &mut W) -> io::Result<()>
where
    W: Write + WriteColor,
{
    const PROPERTY_COLOR: Color = Color::Rgb(255, 150, 0);
    stdout.reset()?;
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(PROPERTY_COLOR))
            .set_italic(true)
            .set_bold(false),
    )
}

pub trait Format {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor;
}

impl Format for Program {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "Program")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()?;
        for statement in &self.statements {
            statement.format(stdout, indent, level + 1)?;
        }
        Ok(())
    }
}

impl Format for Statement {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        match self {
            Statement::VariableDeclaration(v) => v.format(stdout, indent, level)?,
            Statement::FunctionDeclaration(v) => v.format(stdout, indent, level)?,
            Statement::StructDeclaration(v) => v.format(stdout, indent, level)?,
            Statement::ExpressionStatement(v) => todo!(),
            Statement::ReturnStatement(v) => todo!(),
            Statement::BreakStatement(v) => todo!(),
        };
        Ok(())
    }
}

impl Format for VariableDeclaration {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "VariableDeclaration")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()?;
        // Print the variable declaration details
        self.identifier.format(stdout, indent, level + 1)?;
        self.initializer.format(stdout, indent, level + 1)?;
        Ok(())
    }
}

impl Format for FunctionDeclaration {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        let prefix_plus = " ".repeat(indent * (level + 1));
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "FunctionDeclaration")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        // Print the function declaration details
        // e.g., identifier, parameters, return type
        self.identifier.format(stdout, indent, level + 1)?;
        for parameter in &self.parameters {
            parameter.format(stdout, indent, level + 1)?;
        }
        self.return_type.format(stdout, indent, level + 1)?;
        Ok(())
    }
}

impl Format for Parameter {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "Parameter")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()?;
        // Print the parameter details
        self.identifier.format(stdout, indent, level + 1)?;
        self.declared_type.format(stdout, indent, level + 1)?;
        Ok(())
    }
}

impl Format for StructDeclaration {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        match self {
            StructDeclaration::NamedStruct {
                identifier,
                fields,
                span,
            } => {
                write!(stdout, "{}", prefix)?;
                bracket_theme(stdout)?;
                write!(stdout, "[")?;
                node_theme(stdout)?;
                write!(stdout, "StructDeclaration::NamedStruct")?;
                span_theme(stdout)?;
                write!(stdout, " {}", span)?;
                bracket_theme(stdout)?;
                write!(stdout, "]\n")?;
                stdout.reset()?;
                identifier.format(stdout, indent, level + 1)?;
                for field in fields {
                    field.format(stdout, indent, level + 1)?;
                }
            },
            StructDeclaration::TupleStruct {
                identifier,
                fields,
                span,
            } => {
                write!(stdout, "{}", prefix)?;
                bracket_theme(stdout)?;
                write!(stdout, "[")?;
                node_theme(stdout)?;
                write!(stdout, "StructDeclaration::TupleStruct")?;
                span_theme(stdout)?;
                write!(stdout, " {}", span)?;
                bracket_theme(stdout)?;
                write!(stdout, "]\n")?;
                stdout.reset()?;
                identifier.format(stdout, indent, level + 1)?;
                for field in fields {
                    field.format(stdout, indent, level + 1)?;
                }
            },
            StructDeclaration::UnitStruct { identifier, span } => {
                write!(stdout, "{}", prefix)?;
                bracket_theme(stdout)?;
                write!(stdout, "[")?;
                node_theme(stdout)?;
                write!(stdout, "StructDeclaration::UnitStruct")?;
                span_theme(stdout)?;
                write!(stdout, " {}", span)?;
                bracket_theme(stdout)?;
                write!(stdout, "]\n")?;
                stdout.reset()?;
                identifier.format(stdout, indent, level + 1)?;
            }
        };
        Ok(())
    }
}

impl Format for NamedFieldDeclaration {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "NamedFieldDeclaration")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()?;
        // Print the field declaration details
        self.identifier.format(stdout, indent, level + 1)?;
        self.declared_type.format(stdout, indent, level + 1)?;
        //trace!("{:#?}", self);
        Ok(())
    }
}

impl Format for TupleFieldDeclaration {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "TupleFieldDeclaration")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        self.declared_type.format(stdout, indent, level + 1)?;
        stdout.reset()
    }
}

impl Format for Identifier {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "Identifier")?;
        span_theme(stdout)?;
        write!(stdout, " {} ", self.span)?;
        property_theme(stdout)?;
        write!(stdout, "name = \"{}\"", self.name)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()
    }
}

impl Format for Expression {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        match self {
            Expression::BinaryOp(v) => v.format(stdout, indent, level),
            //Expression::FunctionCall(v) => v.format(stdout, indent, level),
            //Expression::BlockExpression(v) => v.format(stdout, indent, level),
            //Expression::IfExpression(v) => v.format(stdout, indent, level),
            Expression::Identifier(v) => v.format(stdout, indent, level),
            Expression::IntegerLiteral(v) => v.format(stdout, indent, level),
            Expression::FloatLiteral(v) => v.format(stdout, indent, level),
            //Expression::StringLiteral(v) => v.format(stdout, indent, level),
            Expression::BooleanLiteral(v) => v.format(stdout, indent, level),
            other => todo!("Implement formatting for {:?}", other),
        }
    }
}

impl Format for BinaryOp {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "BinaryOp")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        property_theme(stdout)?;
        write!(stdout, " operator = \"{}\"", self.operator)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()?;

        // Print the left and right expressions
        self.left.format(stdout, indent, level + 1)?;
        self.right.format(stdout, indent, level + 1)?;

        // Print the operator
        property_theme(stdout)?;

        Ok(())
    }
}

impl Format for IntegerLiteral {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "IntegerLiteral")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        property_theme(stdout)?;
        write!(stdout, " value = {}", self.value)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()
    }
}

impl Format for FloatLiteral {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "FloatLiteral")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        property_theme(stdout)?;
        write!(stdout, " value = {}", self.value)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()
    }
}

impl Format for StringLiteral {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "StringLiteral")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        property_theme(stdout)?;
        write!(stdout, " value = \"{}\"", self.value)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()
    }
}

impl Format for BooleanLiteral {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor,
    {
        let prefix = " ".repeat(indent * level);
        write!(stdout, "{}", prefix)?;
        bracket_theme(stdout)?;
        write!(stdout, "[")?;
        node_theme(stdout)?;
        write!(stdout, "BooleanLiteral")?;
        span_theme(stdout)?;
        write!(stdout, " {}", self.span)?;
        property_theme(stdout)?;
        write!(stdout, " value = {}", self.value)?;
        bracket_theme(stdout)?;
        write!(stdout, "]\n")?;
        stdout.reset()
    }
}
