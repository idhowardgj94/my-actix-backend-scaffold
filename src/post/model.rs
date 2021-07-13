use serde::{Deserialize, Serialize};
use mysql::prelude::FromRow;
use mysql::{Row, FromRowError, from_value, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRequest {
    pub id: Option<i32>,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PostData {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub post_date: String,
    pub create_time: String,
    pub update_time: String,
    pub is_public: i32,
}

// Review: is that good?
impl PostData {
    pub fn add_tags(&mut self, tags: &mut Vec<String>) {
        self.tags.append(tags);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostListPages {
    pub status: String,
    pub pages: u32,
    pub page: u32,
    pub data: Vec<PostData>
}
