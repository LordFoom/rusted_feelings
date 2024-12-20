use anyhow::{Ok, Result};
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
    init_db_tables(&db.conn);
    Ok(db)
}

pub fn init_db_tables(conn: &Connection) -> Result<()> {
    let score_tbl_sql =
        "CREATE TABLE IF NOT EXISTS score (id integer primary key asc, mood_id integer, score REAL)";
    conn.execute(score_tbl_sql, [])?;
    let mood_table_sql =
        "CREATE TABLE IF NOT EXISTS mood (id integer primary key asc, name TEXT NOT NULL UNIQUE)";
    conn.execute(mood_table_sql, [])?;

    let tag_tbl_sql =
        "CREATE TABLE IF NOT EXISTS tag (id integer primary key asc, score_id integer, name TEXT)";
    conn.execute(tag_tbl_sql, [])?;

    Ok(())
}

///Create the argument with "name" in thee sqlite db
pub fn create_mood_if_not_exists(mood_name: &str, db: &AppDb) -> Result<bool> {
    let insert_mood_sql = "INSERT OR IGNORE INTO mood (name) VALUES (?)";
    let res = db.conn.execute(&insert_mood_sql, [mood_name])?;
    Ok(res == 1)
}

pub fn insert_mood_data_point() -> Result<()> {
    Ok(())
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
            SELECT 1 FROM sqlite_master where type='table' and name=?)";
        conn.query_row(sql, params, f)"#;

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
        let db = &AppDb::from_conn(conn);
        let mood_created = create_mood_if_not_exists("peace", &db).unwrap();
        assert!(mood_created);
        let mood_created_twice = create_mood_if_not_exists("peace", &db).unwrap();
        assert!(!mood_created_twice);
    }
}
