use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl From<&User> for BTreeMap<String, String> {
    fn from(u: &User) -> Self {
        let mut b = BTreeMap::new();
        b.insert(String::from("name"), u.name.clone());
        b.insert(String::from("password"), u.password.clone());
        b
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfile {
    pub name: String,
}

