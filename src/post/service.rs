use chrono::{DateTime, Local, SecondsFormat};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::post::model::{PostRequest, PostData};
use log::debug;
use log::info;
use mysql::time::Date;

#[allow(unused_must_use)]
pub fn insert_post(db_pool: DatabaseType, p: PostRequest)-> mysql::Result<()> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            let local: DateTime<Local> = Local::now();
            let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("INSERT INTO posts (title, post_date, content, is_public) VALUES (?, ?, ?, ?)",
                         (p.title, local.to_rfc3339_opts(SecondsFormat::Secs, true), p.content, p.status))?;
            let post_id = tx.last_insert_id().unwrap();
            // insert tag
            for t in &p.tags {
                let res: Option<Row> = tx.exec_first("SELECT id FROM tags WHERE tag_name=?", (t,)).unwrap();
                let tag_id = match res {
                    None => {
                        tx.exec_drop("INSERT INTO tags (tag_name) values (?)", (t,));
                        let tag_id = tx.last_insert_id().unwrap();
                        tag_id
                    },
                    Some(r) => {
                        let tag_id: u64 = r.get(0).unwrap();
                        tag_id
                    }
                };
                tx.exec_drop("INSERT INTO post_tag (post_id, tag_id) VALUES (?, ?)", (post_id, tag_id,));
            };
            tx.commit();
            Ok(())
        }
        _ => Ok(())
    }
}

/// update post
pub fn update_post(db_pool: DatabaseType, id: i32 ,p: PostRequest) -> mysql::Result<()> {
    const UPDATE_POST_QUERY: &str = "UPDATE posts SET title=?, content=?, is_public=? WHERE id=?";
    const DELETE_POST_TAG: &str = "DELETE FROM post_tag WHERE post_id=?";
    const QUERY_TAGS: &str = "SELECT id FROM tags WHERE tag_name=?";
    const INSERT_TAG: &str = "INSERT INTO tags (tag_name) values (?)";
    const INSERT_POST_TAG: &str = "INSERT INTO post_tag (post_id, tag_id) VALUES (?, ?)";
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop(UPDATE_POST_QUERY,
                         (p.title, p.content, p.status, id))?;
            tx.exec_drop(DELETE_POST_TAG, (id,))?;
            // insert tag
            for t in &p.tags {
                let res: Option<Row> = tx.exec_first(QUERY_TAGS, (t,)).unwrap();
                let tag_id = match res {
                    None => {
                        tx.exec_drop(INSERT_TAG, (t,));
                        let tag_id = tx.last_insert_id().unwrap();
                        tag_id
                    },
                    Some(r) => {
                        let tag_id: u64 = r.get(0).unwrap();
                        tag_id
                    }
                };
                tx.exec_drop(INSERT_POST_TAG, (id, tag_id,));
            };
            tx.commit();
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}

pub fn get_blog_by_id(db_pool: DatabaseType, id: i32) -> Option<PostData> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            // XXX exec_first 方法，在使用 get (from FromValue Trait ) 時，在create_time 解析成 String 時會有問題…
            let res = conn.query_first::<Row, _>(format! ("SELECT id, title, content, is_public, create_time, update_time, post_date  FROM posts WHERE id = {}", (id))).unwrap();

            let response = match res {
                Some(it) => {
                    let t_result = conn.exec_iter("SELECT t.tag_name FROM tags t JOIN post_tag p ON p.tag_id = t.id WHERE p.post_id = ?",
                                                  (it.get::<u32, _>(0).unwrap(),)).unwrap();
                    debug!("{:?}", it);
                    Some(PostData {
                        id: it.get(0).unwrap(),
                        title: it.get(1).unwrap(),
                        content: it.get::<String, _>(2).unwrap(),
                        tags: t_result.into_iter().map(|it| it.unwrap().get(0).unwrap()).collect(),
                        create_time: it.get::<String, _>(4).unwrap(),
                        update_time: it.get::<String, _>(5).unwrap(),
                        post_date: it.get::<String, _>(6).unwrap(),
                        is_public: it.get(3).unwrap()
                    })
                },
                None => None
            };
            response
        },
        _ => None
    }
}

pub fn trigger_public_by_id(db_pool: DatabaseType, n: u32)  {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            conn.exec_drop("UPDATE posts SET is_public = IF(is_public = 0, 1, 0) WHERE id=?", (n,)).unwrap();
        }
        _ => {}
    }

}

pub fn select_post_list(db_pool: DatabaseType, page: u32, is_public: i32) -> Option<(u32, Vec<PostData>)> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            // calculate pages
            let count: u32 = conn.query_first("SELECT count(id) FROM posts").unwrap().unwrap();
            let pages = count / 10 + 1;
            if page > pages {
                return None;
            }

            let mut response: Vec<PostData> = Vec::new();
            let mut posts_tmp: Vec<Row> = Vec::new();
            // query_iter
            // query_exec not the same
            let query = match is_public {
                -1 => format!("SELECT id, title, content, is_public, create_time, update_time, post_date FROM posts ORDER BY post_date DESC LIMIT 10 OFFSET {}",
                              (page - 1) * 10),
                1 | _ => format!("SELECT id, title, content, is_public, create_time, update_time, post_date FROM posts WHERE is_public = {} ORDER BY post_date DESC LIMIT 10 OFFSET {}",
                             1, (page - 1) * 10),
            };
            let res= conn.query_iter(query).unwrap();
            for rit in res {
                let it = rit.unwrap();
                posts_tmp.push(it);
            }

            for it in posts_tmp {
                let t_result = conn.exec_iter("SELECT t.tag_name FROM tags t JOIN post_tag p ON p.tag_id = t.id WHERE p.post_id = ?",
                                                       (it.get::<u32, _>(0).unwrap(),)).unwrap();

                // setting content max size 150.
                let mut content = it.get::<String, _>(2).unwrap();
                if content.len() > 150 {
                    content = content.chars().into_iter().take(150).collect();
                }

                response.push( PostData {
                    id: it.get(0).unwrap(),
                    title: it.get(1).unwrap(),
                    content,
                    tags: t_result.into_iter().map(|it| it.unwrap().get(0).unwrap()).collect(),
                    create_time: it.get::<String, _>(4).unwrap(),
                    update_time: it.get::<String, _>(5).unwrap(),
                    post_date: it.get::<String, _>(6).unwrap(),
                    is_public: it.get(3).unwrap()
                })
            }
            Some((pages, response))
        }
        _ => None
    }
}