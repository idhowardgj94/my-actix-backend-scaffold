use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web::cookie::{Cookie, SameSite};
// use actix_web::http::Cookie;
use actix_web::web::Json;
use log::debug;

use crate::commons::database_type::DatabaseType;
use crate::jwt::sign_for_login;
use crate::login::model::User;
use crate::util::user_b_tree_map;

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
pub async fn login_post(db: web::Data<mysql::Pool>, body: Json<User>) -> std::io::Result<HttpResponse> {
    let u = body.into_inner();
    let conn = db.get_conn().unwrap();
    let bool = service::login(DatabaseType::Mysql(conn), &u);
    match bool {
        true => {
            let cookie = Cookie::build("auth", sign_for_login(user_b_tree_map(&u)))
                .secure(false)
                .http_only(true)
                .path("/")
                .same_site(SameSite::Lax)
                .finish();

            Ok(HttpResponse::Ok()
                .set_header("Authorization", format!("{} {}", "Bearer", sign_for_login(user_b_tree_map(&u))))
                .content_type("application/json")
                .cookie(cookie)
                .body(json::object! { "status" => "success" }.dump()))
        }
        false => {
            Ok(HttpResponse::Ok().content_type("application/json").body(json::object! {"status" => "fail"}.dump()))
        }
    }
}

/// POST /me
pub async fn me(req: HttpRequest) -> impl Responder {
    let q = req.headers().get("Authorization");
    match q {
        Some(k) => {
            let k: Vec<&str> = k.to_str().unwrap().split(" ").collect();
            let token = *(k.get(1).unwrap_or_else(|| &""));
            // TODO refactor, 應該不用這個 if else 才對…吧
            if !token.is_empty() {
                HttpResponse::Ok().content_type("application/json").body(json::object! {
                    "status" => "login",
                    "token" => token
                }.dump())
            } else {
                HttpResponse::Ok().content_type("application/json").body(json::object! {
                    "status" => "not_login",
                    "token" => ""
                }.dump())
            }
        }
        None => HttpResponse::Ok().content_type("application/json").body(json::object! {
                "status" => "not_login",
                "token" => ""
            }.dump())
    }
}

/// POST logout
pub async fn logout(req: HttpRequest) -> impl Responder {
    let mut auth_cookie: Cookie = req.cookie("auth").unwrap();
    HttpResponse::Ok().content_type("application/json")
        .del_cookie(&auth_cookie)
        .body(json::object! {
            "status" => "success",
        }.dump())
}