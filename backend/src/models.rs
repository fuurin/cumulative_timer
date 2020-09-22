use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}