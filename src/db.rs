use anyhow::Result;
use rusqlite::Connection;

struct AppDb {
    pub path: String,
    pub conn: Connection,
}
pub fn init_db(path: &str) -> Result<AppDb> {
    let sql = "CREATE TABLE IF NOT EXISTS mood_journal (primary key id, score REAL)";
    let conn = Connection::open(path)?;
    conn.execute(sql, [])?;
    let db = AppDb {
        path: path.to_string(),
        conn,
    };
    Ok(db)
}

mod test {}
