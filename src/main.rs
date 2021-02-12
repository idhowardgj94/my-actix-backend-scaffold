use mysql::*;
use mysql::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder, guard, HttpResponse};
use blog_back::db::migration::*;
use blog_back::login::{login_post, fetch_user};
use std::sync::Arc;
use blog_back::router::not_found;
use actix_web::middleware::Logger;
use log::*;
use serde::{Deserialize, Serialize};
use blog_back::auth_middleware::validator;
use actix_web::web::{Json, scope};
use actix_web::dev::ServiceResponse;
use blog_back::post::post_insert_post;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name:  Option<String>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    // db migration
    let pool = Pool::new("mysql://root:example@127.0.0.1:3306/blog")?;
    let mut conn = pool.get_conn();
    let db_pool = web::Data::new(pool);

    if let Ok(mut c) = conn {
        info!("good, {:?}", c);
        let r = embed::migrations::runner().run(&mut c);
        if let Err(e) = r {
            error!("{:?}", e);
        }
    } else {
        panic!("{:?}", conn);
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::post().to(login_post)))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/fetch")
                            .guard(guard::fn_guard(validator))
                            .route(web::get().to(fetch_user)))
                    .service(
                        web::resource("/post")
                            .guard(guard::fn_guard(validator))
                            .route(web::post().to(post_insert_post))
                    )
            )
            .default_service(web::route().to(not_found))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await?;
    Ok(())
}

async fn index() -> impl Responder {
    format!("Hello, World, Hello, Howard")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(true, true);
    }
}