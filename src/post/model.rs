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

// TODO that not a good solution. I don't think change the default trait is good idea
// but is still work, cool.
impl FromRow for PostData {
    fn from_row(row: Row) -> Self where
        Self: Sized, {
        Self {
            id: from_value::<i32>(row.get(0).unwrap()),
            title: from_value::<String>(row.get(1).unwrap()),
            content: from_value::<String>(row.get(2).unwrap()),
            tags: Vec::new(),
            post_date: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
            create_time: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
            update_time: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
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
