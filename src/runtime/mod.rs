use derive_more::From;
use termcolor::{ColorChoice, StandardStream};

use crate::{
    core::format::Format,
    parser::{
        Parser, ParserError,
        ast::{Expression, Program, Statement},
    },
};

#[derive(Debug, From)]
pub enum RuntimeError {
    ParserError(ParserError),
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
pub struct Runtime {}

impl Runtime {
    /// Creates a new instance of the `Runtime`.
    pub fn new() -> Self {
        Runtime {}
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
        }

        Ok(())
    }
}
