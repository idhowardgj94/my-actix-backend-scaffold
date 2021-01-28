use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub post_date: Option<String>,
    pub content: String,
    pub is_pubic: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTag {
    pub post_id: i32,
    pub tag_id: i32
}

