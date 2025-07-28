use derive_more::From;
use termcolor::{ColorChoice, StandardStream};

use crate::{
    core::format::Format,
    parser::{
        Parser, ParserError,
        ast::{Expression, Program, Statement},
    },
    runtime::environment::{Environment, EnvironmentError, Value},
};

mod environment;

#[derive(Debug, From)]
pub enum RuntimeError {
    ParserError(ParserError),
    EnvironmentError(EnvironmentError),
}

/// # Runtime
///
/// Contains the runtime environment for executing scripts.
/// This module is responsible for managing the execution context, including
/// variables, functions, and the execution flow of the script.
///
/// # Example
/// ```
/// use rscript::runtime::Runtime;
/// let runtime = Runtime::new();
/// runtime.execute("print('Hello, World!')");
/// ```
#[derive(Debug, Clone)]
pub struct Runtime {
    environment: Environment,
}

impl Runtime {
    /// Creates a new instance of the `Runtime`.
    pub fn new() -> Self {
        let environment = Environment::new();
        Runtime { environment }
    }

    /// Executes a script in the runtime environment.
    pub fn execute(&mut self, source: &str) -> Result<(), RuntimeError> {
        trace!("Executing script");
        let parser = Parser::new(source);
        let program = parser.parse()?;
        self.execute_program(program)
    }

    pub fn execute_program(&mut self, program: Program) -> Result<(), RuntimeError> {
        trace!("Executing program");
        if log::max_level() >= log::LevelFilter::Debug {
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            program.format(&mut stdout, 4, 0);
        }

        for statement in &program.statements {
            trace!("Executing statement: {:?}", statement);
            match statement {
                Statement::VariableDeclaration(decl) => {
                    trace!("Variable declaration: {:?}", decl.identifier.name);
                    let name = decl.identifier.name.clone();
                    let value = self.evaluate_expression(&decl.initializer)?;
                    self.environment.declare_variable(name, value)?;
                }
                _ => error!("Unhandled statement type: {:?}", statement),
            }
        }

        Ok(())
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, RuntimeError> {
        trace!("Evaluating expression: {:?}", expression);
        match expression {
            Expression::IntegerLiteral(value) => {
                trace!("Integer literal with value: {:?}", value.value);
                Ok(Value::Int(value.value))
            }
            expr => panic!("Unhandled expression type: {:?}", expr),
        }
    }
}
