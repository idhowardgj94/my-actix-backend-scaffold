use std::collections::{BTreeMap};
use crate::login::model::User;
use log::*;

// a helper for covert user to map
pub fn user_b_tree_map(u: &User) -> BTreeMap< String, String> {
    let mut t = BTreeMap::new();
    t.insert(String::from("name"), u.name.clone());
    info!("{:?}", t);
    t
}