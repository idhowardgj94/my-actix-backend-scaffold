use chrono::{DateTime, Local};
use mysql::{Row, TxOpts};
use mysql::prelude::Queryable;

use crate::commons::database_type::DatabaseType;
use crate::post::model::PostRequest;

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
