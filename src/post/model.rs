use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRequest {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostData {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub create_time: String,
    pub update_time: String,
    pub is_public: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostListPages {
    pub status: String,
    pub pages: u32,
    pub page: u32,
    pub data: Vec<PostData>
}
