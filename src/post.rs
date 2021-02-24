use actix_web::{HttpResponse, web};
use actix_web::web::{Json, Path};
use log::*;
use serde::{Deserialize, Serialize};
use model::{PostListPages, PostRequest};
use crate::commons::database_type::DatabaseType;
use crate::post::model::PostData;
use crate::post::service::{select_post_list, get_blog_by_id, trigger_public_by_id};
use crate::util::DataResponse;

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

/// GET /api/blog/{id}
pub async fn get_blog(db: web::Data<mysql::Pool>, id: Path<i32>) -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    match get_blog_by_id(DatabaseType::Mysql(conn), id.0) {
        Some(d) => {
            // let data = serde_json::to_string(&d).unwrap();
            Ok(HttpResponse::Ok().content_type("application/json").json(DataResponse {
                status: "success",
                data: d
            }))
        },
        None => Ok(HttpResponse::Ok().content_type("application").body(json::object! {
            "status" => "not_found"
        }.dump()))
    }
}

/// POST /api/post/public/{n}
pub async fn trigger_public(db: web::Data<mysql::Pool>, n: Path<u32>) -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    trigger_public_by_id(DatabaseType::Mysql(conn), n.0);
    Ok(HttpResponse::Ok().content_type("application/json").body(json::object! {
        "status" => "success"
    }.dump()))
}

/// GET /api/posts/{page}
pub async fn get_post_list(db: web::Data<mysql::Pool>, info: Path<u32>)
                              -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    let result = select_post_list(DatabaseType::Mysql(conn), info.0, -1);

    match result {
        Some((pages, data)) => Ok(
            HttpResponse::Ok().json(PostListPages {
                status: "success".to_string(),
                pages,
                page: info.0,
                data
            })),
        _ => Ok(HttpResponse::Ok().content_type("application/json").body(
            json::object! {
                "status" => "error",
                "msg" => "error when fetch data"
        }.dump()))
    }
}

/// GET /api/posts/public/{page}
pub async fn get_public_post_list(db: web::Data<mysql::Pool>, info: Path<u32>)
                           -> std::io::Result<HttpResponse> {
    let conn = db.get_conn().unwrap();
    let result = select_post_list(DatabaseType::Mysql(conn), info.0, 1);

    match result {
        Some((pages, data)) => Ok(
            HttpResponse::Ok().json(PostListPages {
                status: "success".to_string(),
                pages,
                page: info.0,
                data
            })),
        _ => Ok(HttpResponse::Ok().content_type("application/json").body(
            json::object! {
                "status" => "error",
                "msg" => "error when fetch data"
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