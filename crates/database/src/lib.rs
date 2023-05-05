use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub user_id: u32,
    pub bond: Bond,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bond {
    Friend,
    Father,
    Mother,
    Daughter,
    Son,
    Neice,
    Nephew,
    GrandMother,
    GrandFather,
    Husband,
    Wife,
    DeFactoPartner,
}

#[derive(Debug, Default)]
pub struct Database {
    pub users: HashMap<u32, User>,
    pub todos: HashMap<u32, Todo>,
    pub relationships: HashMap<u32, Relationship>,
}

impl Database {
    pub fn new(users: Vec<User>, relationships: Vec<Relationship>, todos: Vec<Todo>) -> Self {
        Self {
            users: users.into_iter().map(|user| (user.id, user)).collect(),
            relationships: relationships
                .into_iter()
                .map(|relationship| (relationship.user_id, relationship))
                .collect(),
            todos: todos.into_iter().map(|todo| (todo.id, todo)).collect(),
        }
    }
}
