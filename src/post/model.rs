use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRequest {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub status: i32,
}
