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
    let sql = "CREATE TABLE IF NOT EXISTS mood_journal (primary key id, score REAL)";
    conn.execute(sql, [])?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_init_db() {
        let test_db_file_path = ".test.sqlite.db";
    }
}
