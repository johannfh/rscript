use std::collections::HashMap;

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

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        let scope = self
            .variables
            .last_mut()
            .expect("There should always be at least one scope initialized");
        scope.insert(name, value);
    }
}
