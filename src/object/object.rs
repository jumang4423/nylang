use super::super::ast;
use super::super::enve;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i32),
    Boolean(bool),
    Array(Vec<Object>),
    Null,
    ReturnValue(Box<Object>),
    Closure {
        parameters: Vec<ast::ast::Expression>,
        body: ast::ast::Statement,
        env: enve::enve::Environment,
    },
}

// Implement the Display trait for Object
impl Object {
    pub fn bool_check(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            Object::Null => false,
            _ => true,
        }
    }
}

// object each other is the truly same or not detection closure
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(x), Object::String(y)) => x == y,
            (Object::Integer(x), Object::Integer(y)) => x == y,
            (Object::Boolean(x), Object::Boolean(y)) => x == y,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s)          => write!(f, "{}", s),
            Object::Integer(value)     => write!(f, "{}", value),
            Object::Boolean(value)        => write!(f, "{}", Object::String(if *value { '👍'.to_string() } else { '👎'.to_string() })),
            Object::Null               => write!(f, "null"),
            Object::ReturnValue(value) => write!(f, "{}", value),
            _ => write!(f, "illegal"),
        }
    }
}