use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::Json;
use log::*;
use crate::commons::database_type::DatabaseType;
use crate::jwt::sign_for_login;
use crate::login::model::{User, UserProfile};
use crate::util::{user_b_tree_map, sign_from_string};
use actix_identity::Identity;
use time::Duration;
use std::convert::identity;
use std::borrow::{Borrow, BorrowMut};
use crate::login::user_repository::UserRepository;

pub mod service;
pub mod model;
mod user_repository;

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
    let result = service::login(UserRepository::new(db.get_conn().unwrap()), &u);

    match result {
        Some(userprofile) => {
            id.remember(userprofile.name.clone());
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
            info!("{}", u);
            let token = sign_for_login(sign_from_string(&u));
            HttpResponse::Ok().content_type("application/json").body(json::object! {
                    "status" => "login",
                    "token" => token
                }.dump())
        }
    }
}

/// POST logout
pub async fn logout(req: HttpRequest, id: Identity) -> impl Responder {
    let mut auth_cookie: Cookie = req.cookie("lishin_id").unwrap();
    id.forget();
    HttpResponse::Ok().content_type("application/json")
        .del_cookie(&auth_cookie)
        .body(json::object! {
            "status" => "success",
        }.dump())
}