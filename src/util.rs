use std::collections::{BTreeMap};
use crate::login::User;
use log::*;

// a helper for covert user to map
pub fn user_bTreeMap(u: &User) -> BTreeMap< String, String> {
    let mut t = BTreeMap::new();
    t.insert(String::from("name"), u.name.clone());
    info!("{:?}", t);
    t
}