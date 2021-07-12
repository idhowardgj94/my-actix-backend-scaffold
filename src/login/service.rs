use bcrypt::{DEFAULT_COST, hash, verify};
use mysql::*;
use mysql::prelude::*;
use crate::commons::database_type::DatabaseType;
use crate::login::model::{User, UserProfile};
use actix_identity::Identity;
use crate::jwt::sign_for_login;
use std::collections::BTreeMap;
use log::*;
use crate::login::user_repository::UserRepository;

pub fn login(mut user_repo: UserRepository, user: &User) -> Option<UserProfile> {
    if let Some(u) = user_repo.get_by_name(user.name.clone()) {
        let password = u.password;
        if let Ok(bool) = verify(&user.password, &password) {
            let t = BTreeMap::from(user);
            let sign = sign_for_login(t);
            info!("{}", &sign);
            user_repo.update_login_hash(&sign, &u.name);

            return Some(UserProfile {
                name: u.name.clone()
            })
        }
    }
    None
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
}
