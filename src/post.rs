use actix_web::{HttpResponse, web};
use actix_web::web::Json;
use model::PostRequest;
use crate::commons::database_type::DatabaseType;
mod service;
mod model;

pub async fn post_insert_post(db: web::Data<mysql::Pool>, body: Json<PostRequest>)
    -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    let res = service::insert_post(DatabaseType::Mysql(conn), body.0);

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