use super::super::object;
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Debug)]
pub struct Environment {
  store: HashMap<String, object::object::Object>, // {hoge: ???, ...}
  _virtual: Option<RefCell<Box<Environment>>>,
}

impl Environment {
  pub fn new() -> Self {
    Environment {
      store: HashMap::new(),
      _virtual: None,
    }
  }

  pub fn get(&self, key: &str) -> Option<object::object::Object> {
    match self.store.get(key) {
      Some(obj) => Some(obj.clone()),
      None => match &self._virtual {
        Some(env_store) => env_store.borrow().get(key), // used RefCell
        None => None,
      },
    }
  }

  pub fn ve(__virtual: Environment) -> Environment {
    Environment {
      store: HashMap::new(),
      _virtual: Some(RefCell::new(Box::new(__virtual))),
    }
  }

  pub fn set(&mut self, key: String, value: object::object::Object) {
    self.store.insert(key, value);
  }
}
