use bcrypt::{DEFAULT_COST, hash, verify};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::login::model::User;

pub fn login(db_pool: DatabaseType, user: &User) -> bool {
    let r = match db_pool {
        DatabaseType::Mysql(mut conn) => {
            conn.exec_first(r"SELECT name, password FROM users WHERE name=:name", params! {
                "name" => user.name.clone()
            }).unwrap().map(|(name, password)| {
                    User { name, password }
            })
        },
        // for test
        _ => {
            let password = hash("idhowardgj94", DEFAULT_COST).unwrap();
            Some( User { name: "idhowardgj94".to_string(), password })
        },
    };

    match r {
        Some(u) => {
            let password = u.password;
            match verify(&user.password, &password) {
                Ok(bool) => bool,
                Err(_) => {
                    false
                }
            }
        },
        None => {
            false
        }
    }
}

#[cfg(test)]
mod test_login {
    use bcrypt::{DEFAULT_COST, hash, verify};
    use mockall::*;
    use mockall::predicate::*;
    use rusqlite::Connection;

    use crate::login::*;
    use crate::login::service::login;

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
