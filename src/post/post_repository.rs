use mysql::*;
use mysql::prelude::*;
use mysql::chrono::SecondsFormat;
use crate::db::schema::Post;
use std::borrow::{Borrow, BorrowMut};
use mockall::Any;
use std::rc::Rc;
use std::cell::RefCell;
use crate::post::model::{PostRequest, PostData};
use actix_web::web::post;

const INSERT_POST: &str = "INSERT INTO posts (title, post_date, content, is_public) VALUES (?, ?, ?, ?)";
const GET_ID_BY_TAG_NAME: &str = "SELECT id FROM tags WHERE tag_name=?";
const INSERT_POST_TAG: &str = "INSERT INTO post_tag (post_id, tag_id) VALUES (?, ?)";
const UPDATE_POST_QUERY: &str = "UPDATE posts SET title=?, content=?, is_public=? WHERE id=?";
const CREATE_TAG_NAME: &str= "INSERT INTO tags (tag_name) values (?)";

const DELETE_POST_TAG: &str = "DELETE FROM post_tag WHERE post_id=?";
const QUERY_TAGS: &str = "SELECT id FROM tags WHERE tag_name=?";
const GET_BY_ID: &str = "SELECT id, title, content, is_public, create_time, update_time, post_date  FROM posts WHERE id = ?";
const GET_TAGS_BY_POST_ID: &str = r"SELECT t.tag_name FROM tags t JOIN post_tag p ON p.tag_id = t.id WHERE p.post_id = ?";
const TRIGGER_PUBLIC_BY_ID: &str = r"UPDATE posts SET is_public = IF(is_public = 0, 1, 0) WHERE id=?";
pub struct PostRepository<'a, 't> {
    db: &'a mut Transaction<'t>,
}

impl <'a, 't> PostRepository<'a, 't> {
    pub fn new(db: &'a mut Transaction<'t>) -> Self {
        PostRepository {
            db
        }
    }

    pub fn last_insert_id(&mut self) -> Option<u64> {
        self.db.last_insert_id()
    }
    pub fn insert_post(&mut self, title: &str, post_date: &str, content: &str, is_public: i32)
                   -> mysql::Result<()> {
        self.db.exec_drop(INSERT_POST,
                         (title, post_date, content, is_public))
    }

    pub fn get_id_by_tag_name(&mut self, tag_name: &str) -> mysql::Result<Option<Row>> {
        self.db.exec_first(GET_ID_BY_TAG_NAME, (tag_name, ))
    }

    pub fn create_tag(&mut self, t: &str) -> mysql::Result<()> {
        self.db.exec_drop(CREATE_TAG_NAME, (t,))
    }

    pub fn insert_post_tag(&mut self,  post_id: u64, tag_id: u64) -> mysql::Result<()> {
        self.db.exec_drop(INSERT_POST_TAG, (post_id, tag_id,))
    }

    pub fn update_post(&mut self, p: &PostRequest, id: u64) -> mysql::Result<()> {
        self.db.exec_drop(UPDATE_POST_QUERY,
                          (&p.title, &p.content, p.status, id))
    }

    pub fn delete_post_tag(&mut self, id: u64) -> mysql::Result<()> {
        self.db.exec_drop(DELETE_POST_TAG, (id,))
    }

    pub fn get_first_tag_by_name(&mut self, t: &String) -> Option<Row> {
       self.db.exec_first(QUERY_TAGS, (t,)).unwrap()
    }

    pub fn get_tags_by_post_id(&mut self, post_id: u64) -> Option<Vec<String>> {
        self
            .db
            .exec_map(GET_TAGS_BY_POST_ID, (post_id, ), |(name, )| -> String { name })
            .ok()
    }

    pub fn get(&mut self, id: u64) -> Option<PostData> {
        self.db.exec_map(GET_BY_ID, (id, ), |row: Row| {
            PostData {
                id: from_value::<i32>(row.get(0).unwrap()),
                title: from_value::<String>(row.get(1).unwrap()),
                content: from_value::<String>(row.get(2).unwrap()),
                tags: Vec::new(),
                post_date: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
                create_time: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
                update_time: row.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
                is_public: from_value(row.get(3).unwrap()),
            }
        }).unwrap().pop()
    }

    pub fn set_post_public_by_id(&mut self, id: u64) {
        self.db.exec_drop(TRIGGER_PUBLIC_BY_ID, (id, )).unwrap();
    }

}
