use std::collections::{BTreeMap};
use crate::login::model::{User, UserProfile};
use log::*;
use serde::Deserialize;
use serde::Serialize;

// a helper for covert user to map
pub fn user_b_tree_map(u: &User) -> BTreeMap< String, String> {
    let mut t = BTreeMap::new();
    t.insert(String::from("name"), u.name.clone());
    info!("{:?}", t);
    t
}

pub fn sign_from_string(u: &String) -> BTreeMap<String, String> {
    let mut t = BTreeMap::new();
    t.insert(String::from("name"), u.clone());
    return t;
}
#[derive(Serialize, Deserialize)]
pub struct DataResponse<'a, Data> {
    pub(crate) status: &'a str,
    pub(crate) data: Data
}

// pub fn new_response<Data>(status: String, data: Data) {
//
// }