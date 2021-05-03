pub enum DatabaseType {
    Mysql(mysql::PooledConn),
    Sqlite(rusqlite::Connection),
    None
}
