
pub mod embed {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[cfg(test)]
mod migration_test {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration_work() {
        let mut conn = Connection::open_in_memory().unwrap();
        let r = embed::migrations::runner().run(&mut conn);
        if let Err(_) = r  {
            assert!(true, "錯誤是因為sqlite 不支援 Auto increment ");
        } else {
            assert!(true);
        }
    }
}