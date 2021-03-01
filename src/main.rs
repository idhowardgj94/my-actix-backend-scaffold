use yaml_rust::{YamlLoader};
use std::fs;
use mysql::*;
use log::LevelFilter;
use actix_web::{web, App, HttpServer, Responder, guard};
use blog_back::db::migration::*;
use blog_back::login::{login_post, me, logout};
use blog_back::router::not_found;
use actix_web::middleware::Logger;
use log::*;
use blog_back::auth_middleware::validator;
use blog_back::post::{post_insert_post, get_post_list, get_blog, trigger_public, get_public_post_list};
use chrono::{DateTime, Local, SecondsFormat};


#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name:  Option<String>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    // load env file.
    let env = YamlLoader::load_from_str(&fs::read_to_string("env.yaml").unwrap());
    let server_url;
    let db_type;
    let user;
    let password;
    let db;
    let db_url;
    if let Ok(v) = env {
        let doc = v[0].to_owned();
        server_url = doc["server"]["url"].clone().into_string().unwrap();
        db_type = doc["db"]["type"].clone().into_string().unwrap();
        db_url = doc["db"]["url"].clone().into_string().unwrap();
        db = doc["db"]["db"].clone().into_string().unwrap();
        user = doc["db"]["user"].clone().into_string().unwrap();
        password = doc["db"]["password"].clone().into_string().unwrap();
    } else {
        panic!("please check if your env.yaml exist and valid");
    }
    
    simple_logging::log_to_file("server.log", LevelFilter::Debug);
    // db migration
    let pool = Pool::new(format!("{}://{}:{}@{}/{}", db_type, user, password, db_url, db))?;
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
        .bind(server_url)?
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

#[cfg(test)]
mod test_yaml {
    use super::*;
    #[test]
    pub fn test_env_valid() {
        let env = YamlLoader::load_from_str(&fs::read_to_string("env.yaml").unwrap());
        if let Ok(v) = env {
            let doc = &v[0];
            assert_eq!(doc["server"]["url"].as_str().unwrap(), "0.0.0.0:3000")
        }
    }
}