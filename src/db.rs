use anyhow::{Ok, Result};
use rusqlite::Connection;

struct AppDb {
    pub path: String,
    pub conn: Connection,
}

impl AppDb {
    pub fn new(path: String, conn: Connection) -> Self {
        Self { path, conn }
    }
}
pub fn init_db(path: &str) -> Result<AppDb> {
    let conn = Connection::open(path)?;
    let db = AppDb::new(path.to_string(), conn);
    init_db_tables(&db.conn);
    Ok(db)
}

pub fn init_db_tables(conn: &Connection) -> Result<()> {
    let score_tbl_sql = "CREATE TABLE IF NOT EXISTS score (primary key id, score REAL)";
    conn.execute(score_tbl_sql, [])?;
    let tag_tbl_sql = "CREATE TABLE IF NOT EXISTS tag (primary key id, name TEXT)";
    conn.execute(tag_tbl_sql, [])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use super::init_db_tables;

    #[test]
    fn test_init_db() {
        let conn = Connection::open_in_memory().unwrap();
        init_db_tables(&conn).unwrap();
        //somehow check that there iss a db
        let table_query = r#"SELECT EXISTS ( 
            SELETCT 1 FROM sqlite_master where type='table' and name=?)";
        conn.query_row(sql, params, f)"#;

        let score_exists: bool = conn
            .query_row(table_query, ["score"], |row| row.get(0))
            .unwrap();
        assert!(score_exists);
        let tag_exists: bool = conn
            .query_row(table_query, ["tag"], |row| row.get(0))
            .unwrap();
        assert!(tag_exists);
    }

    fn get_test_conn() {}
}
