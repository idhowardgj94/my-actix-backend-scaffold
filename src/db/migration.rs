
pub mod embed {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[cfg(test)]
mod migration_test {
    use super::*;
    use rusqlite::Connection;
    mod embed {
        use refinery::embed_migrations;
        embed_migrations!("test_migrations");
    }

    #[test]
    fn test_migration_work() {
        let mut conn = Connection::open_in_memory().unwrap();
        let r = self::embed::migrations::runner().run(&mut conn);
        if let Ok(res) = r  {
            println!("{:?}", res);
            assert!(true);
        } else {
            assert!(false);
        }
    }
}