use chrono::{DateTime, Local};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::post::model::{PostRequest, PostListData};
use log::debug;

#[allow(unused_must_use)]
pub fn insert_post(db_pool: DatabaseType, p: PostRequest)-> mysql::Result<()> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            let local: DateTime<Local> = Local::now();
            let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("INSERT INTO posts (title, post_date, content, is_public) VALUES (?, ?, ?, ?)",
                         (p.title, local.to_rfc3339(), p.content, p.status))?;
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

pub fn select_post_list(db_pool: DatabaseType, page: u32) -> Option<(u32, Vec<PostListData>)> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            // calculate pages
            let count: u32 = conn.query_first("SELECT count(id) FROM posts").unwrap().unwrap();
            debug!("{}", count);
            let pages = count / 10 + 1;
            debug!("{}", pages);
            if page > pages {
                return None;
            }

            let mut response: Vec<PostListData> = Vec::new();
            let mut posts_tmp: Vec<Row> = Vec::new();
            // query_iter
            // query_exec not the same
            let res= conn.query_iter(
                format!("SELECT id, title, content, is_public, create_time, update_time FROM posts LIMIT 10 OFFSET {}",
                                     (page - 1) * 10)).unwrap();
            for rit in res {
                let it = rit.unwrap();
                posts_tmp.push(it);
            }

            for it in posts_tmp {
                debug!("{:?}", it);
                debug!("{:?}", it.get::<String, _>(1).unwrap());
                let t_result = conn.exec_iter("SELECT t.tag_name FROM tags t JOIN post_tag p ON p.tag_id = t.id WHERE p.post_id = ?",
                                                       (it.get::<u32, _>(0).unwrap(),)).unwrap();
                response.push( PostListData {
                    id: it.get(0).unwrap(),
                    title: it.get(1).unwrap(),
                    content: it.get(2).unwrap(),
                    tags: t_result.into_iter().map(|it| it.unwrap().get(0).unwrap()).collect(),
                    create_time: it.get::<String, _>(4).unwrap(),
                    update_time: it.get::<String, _>(5).unwrap(),
                    is_public: it.get(3).unwrap()
                })
            }
            Some((pages, response))
        }
        _ => None
    }
}