use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleRequestDAO {
    pub title: String,
    post_date: String,
    content: String,
    is_public: i32,
    tags: Option<Vec<String>>
}