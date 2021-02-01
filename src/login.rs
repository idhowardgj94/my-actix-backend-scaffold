use mysql::prelude::*;
use mysql::*;
use bcrypt::{DEFAULT_COST, hash, verify};
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Json;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use crate::util::user_bTreeMap;
use crate::jwt::sign_for_login;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

// TODO Sqlite for test.
pub enum DatabaseType {
    Mysql(mysql::PooledConn),
    Sqlite(rusqlite::Connection),
    None
}

pub fn login(db_pool: DatabaseType, user: &User) -> bool {
    let r = match db_pool {
        DatabaseType::None  => {
            let password = hash("idhowardgj94", DEFAULT_COST).unwrap();
            Some( User { name: "idhowardgj94".to_string(), password })
        },
        DatabaseType::Sqlite(sqlite) => Some( User { name: "idhowardgj94".to_string(), password: "idhowardgj94".to_string() }),
        DatabaseType::Mysql(mut conn) => {
            conn.exec_first(r"SELECT name, password FROM users WHERE name=:name", params! {
                "name" => user.name.clone()
            }).unwrap().map(|(name, password)| {
                    User { name, password }
            })
        }
    };
    match r {
        Some(u) => {
            let password = u.password;
            match verify(&user.password, &password) {
                Ok(bool) => bool,
                Err(e) => {
                    false
                }
            }
        },
        None => {
            false
        }
    }
}

// route
// TODO error handling
pub async fn login_post(db: web::Data<mysql::Pool>, body: Json<User>) -> std::io::Result<HttpResponse> {
    let u = body.into_inner();
    let conn = db.get_conn().unwrap();
    let bool = login(DatabaseType::Mysql(conn), &u);
    match bool {
        true => {
            Ok(HttpResponse::Ok()
                .set_header("Authorization",  format!("{} {}", "Bearer", sign_for_login(user_bTreeMap(&u))))
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

#[cfg(test)]
mod test_login {
    use super::*;
    use rusqlite::Connection;
    use mockall::*;
    use mockall::predicate::*;
    use bcrypt::{DEFAULT_COST, hash, verify};
    mod embed {
        use refinery::embed_migrations;
        embed_migrations!("test_migrations");
    }
    fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        let r = self::embed::migrations::runner().run(&mut conn);
        let hashed = hash("idhowardgj94", DEFAULT_COST)
            .expect("something went wrong with hash");
        // conn.execute("INSERT INTO users (name, password) VALUES (:name, :password)",
        //                        params! { "name" => "howardgj94", "password" => hashed } ).unwrap();
        conn
    }

    #[test]
    fn login_test() {
        let conn = setup();
        assert!(login( DatabaseType::None, &User {
            name: "idhowardgj94".to_string(),
            password: "idhowardgj94".to_string()
        }));
    }
}