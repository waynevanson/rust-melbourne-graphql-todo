use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub name: String,
    pub created_by_user_id: u32,
    pub compeleted_by_user_id: Option<u32>,
}

#[derive(Debug, Default)]
pub struct Database {
    pub persons: HashMap<u32, Person>,
}

impl Database {
    pub fn new(users: Vec<Person>) -> Self {
        Self {
            persons: users.into_iter().map(|user| (user.id, user)).collect(),
        }
    }
}
