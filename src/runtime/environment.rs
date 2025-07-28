use std::collections::HashMap;

use derive_more::From;

use crate::runtime::Runtime;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Unit,
    StructInstance(StructInstance),
    //TODO: Function definitions
    //FunctionDefinition(FunctionDefinition),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructInstance {
    pub name: String,
    pub fields: Vec<(String, Value)>,
}

//#[derive(Debug, Clone, PartialEq)]
//pub struct FunctionDefinition {
//    pub name: String,
//    pub parameters: Vec<String>,
//    pub body: Vec<Statement>
//}

#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentError {
    VariableNotFound(String),
    VariableAlreadyDeclared(String),
}

/// # Environment
/// The [`Environment`] struct represents the runtime environment for executing scripts.
/// It manages the variables and their scopes, allowing for variable lookup and assignment.
/// It supports nested scopes, enabling variable shadowing and scoping rules similar to those found
/// in many programming languages.
///
/// # Example
///
/// ```
/// use rscript::runtime::environment::Environment;
/// let mut env = Environment::new();
/// env.set_variable("x".to_string(), Value::Int(42));
/// assert_eq!(env.get_variable("x"), Some(&Value::Int(42)));
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        // -- Variables --
        let mut variables = Vec::new();
        // Initialize the top level scope for variables
        variables.push(HashMap::new());

        Environment { variables }
    }

    /// Returns a reference to the current scope.
    fn scope(&self) -> &HashMap<String, Value> {
        self.variables
            .last()
            .expect("There should always be at least one scope initialized")
    }

    /// Returns a mutable reference to the current scope.
    fn scope_mut(&mut self) -> &mut HashMap<String, Value> {
        self.variables
            .last_mut()
            .expect("There should always be at least one scope initialized")
    }

    fn push_scope(&mut self) {
        trace!("Pushing a new scope");
        self.variables.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        trace!(
            "Popping the current scope with {} variables",
            self.scope().len()
        );
        if self.variables.len() > 1 {
            self.variables.pop();
        } else {
            warn!("Attempted to pop the last scope, which is not allowed");
        }
    }

    /// Declares a new variable in the current scope. Supports shadowing of variables.
    pub fn declare_variable(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        trace!("Declaring variable: {}", name);
        self.scope_mut().insert(name, value);
        Ok(())
    }

    /// Gets the value of a variable by its name, searching through all scopes from innermost to
    /// outermost.
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        trace!("Getting variable: {}", name);
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    /// Sets the value of a variable by its name, searching through all scopes from innermost to
    /// outermost. If the variable is not found, it will return an error.
    pub fn set_variable(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        trace!("Setting variable: {} to: {:?}", name, value);
        self.scope_mut().insert(name, value);
        Ok(())
    }
}
