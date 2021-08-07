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
    // pub expire_date: Date
}

#[cfg(test)]
mod testChrono {
    use chrono::{DateTime, Local, NaiveDateTime, Utc, FixedOffset, LocalResult};

    #[test]
    pub fn test_timestamp() {
        // let timestamp = 1572580800;
        let nano: i64 = 1628348358004 % 1000;
        let hour = 3600;
        let d = DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(1628348358004/1000, nano as u32), FixedOffset::east(8 * hour));
        panic!("{:?}", d);
        // TODO: this is how to use TimeStamp and convert to TW time
    }
}
