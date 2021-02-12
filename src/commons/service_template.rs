
fn db_template() {
    let r = match db_pool {
        //` for test
        DatabaseType::None  => {
            let password = hash("idhowardgj94", DEFAULT_COST).unwrap();
            Some( User { name: "idhowardgj94".to_string(), password })
        },
        // for test
        DatabaseType::Sqlite(sqlite) => Some( User { name: "idhowardgj94".to_string(), password: "idhowardgj94".to_string() }),
        DatabaseType::Mysql(mut conn) => {
            // eqecute sql
            conn.exec_first(r"SELECT name, password FROM users WHERE name=:name", params! {
                "name" => user.name.clone()
            }).unwrap().map(|(name, password)| {
                // map to struct
                User { name, password }
            })
        }
    };
}