use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Json;
use bcrypt::{DEFAULT_COST, hash, verify};
use json::JsonValue;
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};

use crate::commons::database_type::DatabaseType;
use crate::jwt::sign_for_login;
use crate::login::model::User;
use crate::util::user_b_tree_map;

pub mod service;
pub mod model;

// route
// TODO error handling
pub async fn login_post(db: web::Data<mysql::Pool>, body: Json<User>) -> std::io::Result<HttpResponse> {
    let u = body.into_inner();
    let conn = db.get_conn().unwrap();
    let bool = service::login(DatabaseType::Mysql(conn), &u);
    match bool {
        true => {
            Ok(HttpResponse::Ok()
                .set_header("Authorization",  format!("{} {}", "Bearer", sign_for_login(user_b_tree_map(&u))))
                .content_type("application/json")
                .body(json::object! { "status" => "success", "data" => "test" }.dump()))
        },
        false => {
            Ok(HttpResponse::Ok().content_type("application/json").body(json::object! {"status" => "fail"}.dump()))
        }
    }
}

// TODO 1, 取得資料 2, timeout
pub async fn fetch_user() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(json::object! {
        "status" => "login"
    }.dump())
}
