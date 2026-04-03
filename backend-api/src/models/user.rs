use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, age: {}", self.name, self.age)
    }
}
