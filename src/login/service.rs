use bcrypt::{DEFAULT_COST, hash, verify};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::login::model::{User, UserProfile};
use actix_identity::Identity;
use crate::jwt::sign_for_login;
use std::collections::BTreeMap;
use log::*;

pub fn login(db_pool: DatabaseType, user: &User) -> Option<UserProfile> {

    match db_pool {
        DatabaseType::Mysql(mut conn) => {
            let r = conn.exec_first(r"SELECT name, password FROM users WHERE name=:name", params! {
                "name" => user.name.clone()
            }).unwrap().map(|(name, password)| {
                    User { name, password }
            });
            match r {
                Some(u) => {
                    let password = u.password;
                    match verify(&user.password, &password) {
                        Ok(bool) => {
                            // TODO 用JWT 簽合法的hash
                            let t = BTreeMap::from(user);
                            let sign = sign_for_login(t);
                            info!("{}", &sign);
                            conn.exec_drop("update users set login_hash=:hash where name =:name", params! {
                                "hash" => &sign,
                                "name" => u.name.clone()
                            });

                            Some(UserProfile {
                                name: u.name.clone()
                            })
                        },
                        Err(_) => {
                            None
                        }
                    }
                },
                None => {
                    None
                }
            }
        },
        // for test
        _ => {
            let password = hash("idhowardgj94", DEFAULT_COST).unwrap();
            Some( UserProfile { name: "idhowardgj94".to_string() })
        },
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
    use crate::login::model::UserProfile;

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
        assert!(matches! { login( DatabaseType::None, &User {
            name: "idhowardgj94".to_string(),
            password: "idhowardgj94".to_string()
        }) , Option::Some(_) } );
    }
}
