use crate::commons::database_type::DatabaseType;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use crate::login::model::User;
use mysql::prelude::*;
use mysql::*;
use crate::db::model::MySqLDatabase;

const GET_USER_BY_NAME: &str = r"SELECT name, password FROM users WHERE name=:name";
const UPDATE_LOGIN_HASH: &str = r"update users set login_hash=:hash where name =:name";


pub struct UserRepository<'a, 't>  {
    db: MySqLDatabase<'a, 't>,
}

impl <'a, 't>UserRepository<'a, 't> {
    pub fn new(db: MySqLDatabase<'a, 't>) -> Self {
        UserRepository {
            db
        }
    }

    pub fn get_by_name(&mut self, name: String) -> Option<User>{
        match self.db {
            MySqLDatabase::Transaction(ref mut db) => {
                db.exec_first(GET_USER_BY_NAME, params! {
                "name" => name
                }).unwrap().map(|(name, password)| {
                    User { name, password }
                })
            }
            MySqLDatabase::Connection(ref mut db) => {
                db.exec_first(GET_USER_BY_NAME, params! {
                "name" => name
                }).unwrap().map(|(name, password)| {
                    User { name, password }
                })
            }
        }
    }

    pub fn update_login_hash(&mut self, sign: &str, name: &str) {
        match self.db {
            MySqLDatabase::Transaction(ref mut db) => {
                db.exec_drop(UPDATE_LOGIN_HASH,  params! {
                                "hash" => sign,
                                "name" => name
                            });
            }
            MySqLDatabase::Connection(ref mut db) => {
                db.exec_drop(UPDATE_LOGIN_HASH,  params! {
                                "hash" => sign,
                                "name" => name
                            });
            }
        }
    }
}


