use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web::cookie::{Cookie, SameSite};
// use actix_web::http::Cookie;
use actix_web::web::Json;
use log::debug;

use crate::commons::database_type::DatabaseType;
use crate::jwt::sign_for_login;
use crate::login::model::{User, UserProfile};
use crate::util::{user_b_tree_map, user_profile_b_tree_map};
use actix_identity::Identity;
use time::Duration;
use std::convert::identity;
use std::borrow::Borrow;

pub mod service;
pub mod model;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/login").route(web::post().to(login_post)))
        .service(web::resource("/api/logout").route(web::post().to(logout)))
        .service(
            web::resource("/api/me")
                .route(web::post().to(me))
        );
}
// route
// TODO error handling
pub async fn login_post(db: web::Data<mysql::Pool>, body: Json<User>, id: Identity) -> std::io::Result<HttpResponse> {

    let u = body.into_inner();
    let conn = db.get_conn().unwrap();
    let result = service::login(DatabaseType::Mysql(conn), &u);
    match result {
        Some(userprofile) => {
            id.remember(   serde_json::to_string(&userprofile).unwrap());
            Ok(HttpResponse::Ok()
                .set_header("Authorization", format!("{} {}", "Bearer", sign_for_login(user_b_tree_map(&u))))
                .content_type("application/json")
                .body(json::object! { "status" => "success" }.dump()))
        }
        None => {
            id.forget();
            Ok(HttpResponse::Ok().content_type("application/json").body(json::object! {"status" => "fail"}.dump()))
       }
    }
}

/// POST /me
pub async fn me(req: HttpRequest, id: Identity) -> impl Responder {
    match id.identity() {
        None => HttpResponse::Ok().content_type("application/json").body(json::object! {
                "status" => "not_login",
                "token" => ""
            }.dump()),
        Some(u) => {
            let s = serde_json::from_str::<UserProfile>(u.borrow()).unwrap();
            let token = sign_for_login(user_profile_b_tree_map(&s));
            HttpResponse::Ok().content_type("application/json").body(json::object! {
                    "status" => "login",
                    "token" => token
                }.dump())
        }
    }
}

/// POST logout
pub async fn logout(req: HttpRequest, id: Identity) -> impl Responder {
    let mut auth_cookie: Cookie = req.cookie("auth").unwrap();
    HttpResponse::Ok().content_type("application/json")
        .del_cookie(&auth_cookie)
        .body(json::object! {
            "status" => "success",
        }.dump())
}