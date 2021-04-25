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

impl FromRow for PostData {
    fn from_row(row: Row) -> Self where
        Self: Sized, {
        Self {
            id: from_value::<i32>(row.get(0).unwrap()),
            title: from_value::<String>(Value::Bytes(row.get(1).unwrap())),
            content: from_value::<String>(row.get(2).unwrap()),
            tags: Vec::new(),
            post_date: from_value::<String>(row.get(4).unwrap()),
            create_time: from_value::<String>(row.get(5).unwrap()),
            update_time: from_value::<String>(row.get(6).unwrap()),
            is_public: from_value(row.get(3).unwrap()),
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, FromRowError> where
        Self: Sized {
        unimplemented!()
    }
}

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
