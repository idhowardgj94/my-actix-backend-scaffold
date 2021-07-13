use chrono::{DateTime, Local, SecondsFormat};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::post::model::{PostRequest, PostData};
use log::debug;
use log::info;
use mysql::time::Date;
use std::borrow::{Cow, BorrowMut};
use std::cell::{RefCell, RefMut};
use crate::post::post_repository::{PostRepository};
use std::rc::Rc;

#[allow(unused_must_use)]
pub fn insert_post(mut conn:  PooledConn, p: PostRequest) -> mysql::Result<()> {
    let mut tx = conn.start_transaction(TxOpts::default())?;
    let mut post_repo = PostRepository::new(&mut tx);
    let local: DateTime<Local> = Local::now();
    post_repo.insert_post(
        &p.title,
        &local.to_rfc3339_opts(SecondsFormat::Secs, true),
        &p.content,
        p.status)?;

    let post_id = post_repo.last_insert_id().unwrap();
    // insert tag
    for t in &p.tags {
        let tag_id = match post_repo.get_id_by_tag_name( t)? {
            None => {
                post_repo.create_tag(t)?;
                let tag_id = post_repo.last_insert_id().unwrap();
                tag_id
            },
            Some(r) => {
                let tag_id: u64 = r.get(0).unwrap();
                tag_id
            }
        };
        post_repo.insert_post_tag(post_id, tag_id);
    };

    tx.commit();
    Ok(())
}

/// update post
pub fn update_post(mut conn:  PooledConn,  id: u64 ,p: PostRequest) -> mysql::Result<()> {

    let mut tx = conn.start_transaction(TxOpts::default())?;
    let mut post_repo = PostRepository::new(&mut tx);
    post_repo.update_post(&p, id).unwrap();
    post_repo.delete_post_tag(id).unwrap();
    // insert tag
    for t in &p.tags {
        let tag_id = match post_repo.get_first_tag_by_name(t) {
            None => {
                post_repo.create_tag(t).unwrap();
                post_repo.last_insert_id().unwrap()
            },
            Some(r) => {
                r.get(0).unwrap()
            }
        };
        post_repo.insert_post_tag(id, tag_id);
    };
    tx.commit();
    Ok(())
}

pub fn get_blog_by_id(mut conn: PooledConn, id: u64) -> Option<PostData> {
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    let mut post_repo = PostRepository::new( &mut tx);
    let res = post_repo.get(id);
    let response = if let Some(mut it) = res {
        let id = it.id;
        let mut tags = post_repo.get_tags_by_post_id(it.id as u64).unwrap();
        it.add_tags(&mut tags);
        Some(it)
    } else {
        panic!("parsing error");
    };
    tx.commit();
    response
}

pub fn trigger_public_by_id(mut conn: PooledConn, n: u64)  {
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    let mut post_repo = PostRepository::new(&mut tx);
    post_repo.set_post_public_by_id(n);
}

// TODO: redesign code
pub fn select_post_list(db_pool: DatabaseType, mut page: u32, is_public: i32) -> Option<(u32, Vec<PostData>)> {
    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            // calculate pages
            let count: u32 = conn.query_first("SELECT count(id) FROM posts").unwrap().unwrap();
            let pages = (count / 10)  + 1;
            if page > pages {
                return None;
            }

            let mut response: Vec<PostData> = Vec::new();
            let mut posts_tmp: Vec<Row> = Vec::new();
            // query_exec not the same
            // exec_iter return Binary, which will deserialized as Value (enum),
            // and need to implement FromValue ( I guess)
            // use this workaround for now.
            // every query must exec
            let res = match is_public {
                -1 => {
                    let stmt = "SELECT id, title, content, is_public, create_time, update_time, post_date FROM posts ORDER BY post_date DESC LIMIT 10 OFFSET :page";
                    conn.exec_iter(stmt, params! {
                        "page" => page * 10
                    }).unwrap()
                },
                1 | _ => {
                    let stmt = "SELECT id, title, content, is_public, create_time, update_time, post_date FROM posts WHERE is_public = :is_public ORDER BY post_date DESC LIMIT 10 OFFSET :page";
                    conn.exec_iter(stmt, params! {
                        "is_public" => 1,
                        "page" => page * 10
                    }).unwrap()
                }
            };


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
                    create_time: it.get::<Value, _>(4).unwrap().as_sql(true).trim_matches('\'').to_string(),
                    update_time: it.get::<Value, _>(5).unwrap().as_sql(true).trim_matches('\'').to_string(),
                    post_date: it.get::<Value, _>(6).unwrap().as_sql(true).trim_matches('\'').to_string(),
                    is_public: it.get(3).unwrap()
                })
            }
            Some((pages, response))
        }
        _ => None
    }
}