use color_eyre::Result;
use rusqlite::{params, Connection};
use rust_decimal::Decimal;

use crate::args::AppArgs;

pub struct AppDb {
    pub path: String,
    pub conn: Connection,
}

pub struct Score {
    pub id: usize,
    pub score: String,
    pub create_date: chrono::NaiveDateTime,
    pub tags: Vec<String>,
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
        "CREATE TABLE IF NOT EXISTS score (id integer primary key asc, mood_id integer, create_date default current_timestamp, score REAL)";
    conn.execute(score_tbl_sql, [])?;

    let tag_tbl_sql =
        "CREATE TABLE IF NOT EXISTS tag (id integer primary key asc, score_id integer, name TEXT)";
    conn.execute(tag_tbl_sql, [])?;

    Ok(())
}

pub fn add_score_and_tags(args: &AppArgs, conn: &Connection) -> Result<()> {
    let score_id = add_score(args.score, conn)?;
    if let tag_values = &args.tags {
        add_tags(tag_values, score_id, conn)?;
    };
    Ok(())
}

///Insert a score and return the db id
pub fn add_score(score: Decimal, conn: &Connection) -> Result<i64> {
    conn.execute(
        "INSERT INTO score(score) VALUES(?)",
        params![score.to_string()],
    )?;
    let score_id = conn.last_insert_rowid();
    Ok(score_id)
}

///Add tags associated with score id
pub fn add_tags(tags: &Vec<String>, score_id: i64, db: &AppDb) -> Result<()> {
    for tag in tags {
        db.conn.execute(
            "INSERT INTO tag (name, score_id) VALUES (?,?) ",
            params![tag, score_id],
        )?;
    }
    Ok(())
}

///get a list of scores and associated tags
pub fn list_scores(
    db: &AppDb,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Vec<Score>> {
    let mut sql = "SELECT id, score, create_date FROM score WHERE 1=1 ".to_string();
    let mut parms = Vec::new();
    if let Some(dt) = start_date {
        sql.push_str("AND create_date >= ?");
        parms.push(dt);
    };
    if let Some(dt) = end_date {
        sql.push_str("AND create_date < ?");
        parms.push(dt);
    };

    let tag_sql = "SELECT name from tag where score_id = ? ";
    let mut stmt = db.conn.prepare(&sql)?;
    let mut tag_stmt = db.conn.prepare(&tag_sql)?;
    let score_rows = stmt.query_map([], |row| {
        let mut score = Score {
            id: row.get(0)?,
            score: row.get(1)?,
            create_date: row.get(2)?,
            tags: Vec::new(),
        };

        let tags_iter = &tag_stmt.query_map([score.id], |tag_row| Ok(row.get(0)?.to_string()));
        let mut tags = Vec::new();
        for tag_row in tags_iter {
            let tag = tag_row.unwrap();
            tags.push(tag);
        }
        score.tags = tags;
        Ok(score)
    })?;
    let mut scores = Vec::new();
    for score_wrap in score_rows {
        let score = score_wrap.unwrap();
        scores.push(score);
    }
    Ok(scores)
}

#[cfg(test)]
mod test {
    use crate::db::AppDb;
    use rusqlite::Connection;
    use rust_decimal_macros::dec;

    use super::init_db_tables;

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
        let tag_exists: bool = conn
            .query_row(table_query, ["tag"], |row| row.get(0))
            .unwrap();
        assert!(tag_exists);
    }

    fn get_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db_tables(&conn).unwrap();
        conn
    }

    #[test]
    pub fn test_add_score_and_tags() {}
}
