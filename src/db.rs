use color_eyre::Result;
use rusqlite::Connection;

pub struct AppDb {
    pub path: String,
    pub conn: Connection,
}

impl AppDb {
    pub fn new(path: String, conn: Connection) -> Self {
        Self { path, conn }
    }

    pub fn from_conn(conn: Connection) -> Self {
        Self {
            path: "IN-MEMORY".to_string(),
            conn: conn,
        }
    }
}
pub fn init_db(path: &str) -> Result<AppDb> {
    let conn = Connection::open(path)?;
    let db = AppDb::new(path.to_string(), conn);
    init_db_tables(&db.conn)?;
    Ok(db)
}

pub fn init_db_tables(conn: &Connection) -> Result<()> {
    let score_tbl_sql =
        "CREATE TABLE IF NOT EXISTS score (id integer primary key asc, mood_id integer, score REAL)";
    conn.execute(score_tbl_sql, [])?;
    let mood_table_sql =
        "CREATE TABLE IF NOT EXISTS mood (id integer primary key asc, name TEXT NOT NULL UNIQUE, description TEXT)";
    conn.execute(mood_table_sql, [])?;

    let tag_tbl_sql =
        "CREATE TABLE IF NOT EXISTS tag (id integer primary key asc, score_id integer, name TEXT)";
    conn.execute(tag_tbl_sql, [])?;

    Ok(())
}

///Create the argument with "name" in thee sqlite db.
///Returns true if a mood was created
pub fn create_mood_if_not_exists(
    mood_name: &str,
    maybe_mood_description: &Option<String>,
    db: &AppDb,
) -> Result<bool> {
    let mood_description = maybe_mood_description.as_deref().unwrap_or("");
    let insert_mood_sql = "INSERT OR IGNORE INTO mood (name, description) VALUES (?, ?)";
    let res = db
        .conn
        .execute(&insert_mood_sql, [mood_name, mood_description])?;
    Ok(res == 1)
}

pub fn add_mood_score(
    score: rust_decimal::Decimal,
    mood: &str,
    days_back: Option<usize>,
    db: &AppDb,
) -> Result<String> {
    //get the mood to ensure it exists
    let check_mood_sql = "SELECT COUNT(*) FROM mood WHERE name = ?";

    let ok = String::from("OK");
    Ok(ok)
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use crate::db::AppDb;

    use super::{create_mood_if_not_exists, init_db_tables};

    #[test]
    fn test_init_db() {
        let conn = Connection::open_in_memory().unwrap();
        init_db_tables(&conn).unwrap();
        //somehow check that there iss a db
        let table_query = r#"SELECT EXISTS ( 
            SELECT 1 FROM sqlite_master where type='table' and name=?)"#;

        let score_exists: bool = conn
            .query_row(table_query, ["score"], |row| row.get(0))
            .unwrap();
        assert!(score_exists);
        let mood_exists: bool = conn
            .query_row(table_query, ["mood"], |row| row.get(0))
            .unwrap();
        assert!(mood_exists);
        let tag_exists: bool = conn
            .query_row(table_query, ["tag"], |row| row.get(0))
            .unwrap();
        assert!(tag_exists);
    }

    #[test]
    fn test_create_mood_if_not_exists() {
        let conn = Connection::open_in_memory().unwrap();
        init_db_tables(&conn).unwrap();
        let db = &AppDb::from_conn(conn);
        let mood_created = create_mood_if_not_exists("peace", &None, &db).unwrap();
        assert!(mood_created);
        let mood_created_twice = create_mood_if_not_exists("peace", &None, &db).unwrap();
        assert!(!mood_created_twice);
    }
}
