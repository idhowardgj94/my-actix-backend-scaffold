use mysql::*;
use actix_web::{web, App, HttpServer, Responder, guard};
use blog_back::db::migration::*;
use blog_back::login::{login_post, me, logout};
use blog_back::router::not_found;
use actix_web::middleware::Logger;
use log::*;
use blog_back::auth_middleware::validator;
use blog_back::post::{post_insert_post, get_post_list, get_blog, trigger_public, get_public_post_list};

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
    let conn = pool.get_conn();
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
            .service(web::resource("/api/login").route(web::post().to(login_post)))
            .service(web::resource("/api/logout").route(web::post().to(logout)))
            .service(
                web::resource("/api/me")
                    .route(web::post().to(me))
            )
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/post")
                            .guard(guard::fn_guard(validator))
                            .route(web::post().to(post_insert_post)))
                    .service(
                        web::resource("/posts/{page}")
                            .route(web::get().to(get_post_list)))
                    .service(
                        web::resource("/posts/public/{page}")
                            .route(web::get().to(get_public_post_list)))
                    .service(
                        web::resource("/post/public/{n}")
                            .guard(guard::fn_guard(validator))
                            .route(web::post().to(trigger_public))
                    )
                    .service(
                        web::resource("/blog/{id}")
                            .route(web::get().to(get_blog))
                    )
            )
            .default_service(web::route().to(not_found))
    })
        .bind("0.0.0.0:3001")?
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