use async_graphql::*;
use std::string::ToString;

#[derive(Debug, SimpleObject, Clone)]
pub struct Name {
    pub first: String,
    pub last: String,
}

impl From<&database::Person> for Name {
    fn from(value: &database::Person) -> Self {
        Self {
            first: value.first_name.to_string(),
            last: value.last_name.to_string(),
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: u32,
    pub name: Name,
}

impl From<&database::Person> for User {
    fn from(value: &database::Person) -> Self {
        Self {
            id: value.id,
            name: value.into(),
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct Relationship {
    bond: String,
    user: User,
}

impl From<(String, User)> for Relationship {
    fn from((bond, user): (String, User)) -> Self {
        Self { bond, user }
    }
}
