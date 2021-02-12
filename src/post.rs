use serde::{Serialize, Deserialize};
use crate::commons::database_type::DatabaseType;
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Json;
use json::JsonValue;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRequest {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub status: i32,
}

pub fn insert_post(db_pool: DatabaseType, p: PostRequest)-> Result<()> {
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


pub async fn post_insert_post(db: web::Data<mysql::Pool>, body: Json<PostRequest>)
    -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    let res = insert_post(DatabaseType::Mysql(conn), body.0);

    match res {
        Ok(()) => Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json::object! {
                        "status" => "success" }.dump())),
        _ => Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json::object! {
                        "status" => "error"
                    }.dump()))
    }

}

#[cfg(test)]
pub mod test_time {
    use chrono::prelude::*;
    #[test]
    pub fn test_datetime() {
        let local: DateTime<Local> = Local::now();
        println!("{:?}", local);
        println!("{}", local.to_rfc2822());
        println!("{}", local.to_rfc3339());
        assert!(false);
    }
}