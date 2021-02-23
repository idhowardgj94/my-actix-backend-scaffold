use std::collections::{BTreeMap};
use crate::login::model::User;
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

#[derive(Serialize, Deserialize)]
pub struct DataResponse<'a, Data> {
    pub(crate) status: &'a str,
    pub(crate) data: Data
}

// pub fn new_response<Data>(status: String, data: Data) {
//
// }