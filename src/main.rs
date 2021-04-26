use yaml_rust::{YamlLoader};
use mysql::*;
use log::LevelFilter;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use blog_back::db::migration::*;
use actix_web::middleware::Logger;
use log::*;
use blog_back::post;
use blog_back::login;
use blog_back::env;
use actix_identity::{IdentityService, CookieIdentityPolicy};
use actix_web::cookie::Cookie;


#[actix_web::main]
async fn main() -> Result<()> {
    // load env file.
    let env = env::get_from("env.yaml");

    simple_logging::log_to_file("server.log", LevelFilter::Debug).unwrap();
    // db migration
    let pool = Pool::new(format!("{}://{}:{}@{}/{}", env.db_type, env.user, env.password, env.db_url, env.db))?;
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
            .wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 32])
                .name("lishin-id")
                .secure(true)))
            .app_data(db_pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .configure(login::route)
            .configure(post::route)
            .default_service(web::route().to(not_found))
    })
        .bind(env.server_url)?
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

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body(json::object! {
        "status" => "not found",
        "msg" => "please check the api doc and try again",
    }.dump())
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