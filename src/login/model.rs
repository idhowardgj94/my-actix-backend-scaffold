use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfile {
    pub name: String,
}