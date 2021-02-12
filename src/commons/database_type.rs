// TODO Sqlite for test.
pub enum DatabaseType {
    Mysql(mysql::PooledConn),
    Sqlite(rusqlite::Connection),
    None
}
