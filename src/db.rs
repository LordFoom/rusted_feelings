use chrono::NaiveDate;
use color_eyre::Result;
use rusqlite::{params, Connection};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct AppDb {
    pub path: String,
    pub conn: Connection,
}
pub struct Score {
    pub id: usize,
    pub score: Decimal,
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
        "CREATE TABLE IF NOT EXISTS score (id integer primary key asc, create_date default current_timestamp, score REAL)";
    conn.execute(score_tbl_sql, [])?;

    let tag_tbl_sql =
        "CREATE TABLE IF NOT EXISTS tag (id integer primary key asc, score_id integer, name TEXT)";
    conn.execute(tag_tbl_sql, [])?;

    Ok(())
}

pub fn add_score_and_tags(score: &Decimal, tags: &Vec<String>, conn: &Connection) -> Result<()> {
    let score_id = add_score(score, conn)?;
    if !tags.is_empty() {
        add_tags(tags, score_id, conn)?;
    };
    Ok(())
}

///Insert a score and return the db id
pub fn add_score(score: &Decimal, conn: &Connection) -> Result<i64> {
    conn.execute(
        "INSERT INTO score(score) VALUES(?)",
        params![score.to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

///Add tags associated with score id
pub fn add_tags(tags: &Vec<String>, score_id: i64, conn: &Connection) -> Result<()> {
    for tag in tags {
        conn.execute(
            "INSERT INTO tag (name, score_id) VALUES (?,?) ",
            params![tag, score_id],
        )?;
    }
    Ok(())
}

///get a list of scores and associated tags
pub fn list_scores(
    conn: &Connection,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
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
    let mut stmt = conn.prepare(&sql)?;
    let mut tag_stmt = conn.prepare(&tag_sql)?;
    let score_rows = stmt.query_map([], |row| {
        let mut score = Score {
            id: row.get(0)?,
            score: dec!(row.get(1)?),
            create_date: row.get(2)?,
            tags: Vec::new(),
        };

        let tags_iter = tag_stmt.query_map([score.id], |tag_row| Ok(row.get::<_, String>(0)?))?;
        let mut tags = Vec::new();
        for tag_row in tags_iter.into_iter() {
            //match tag_row {
            //    Ok(tag) => tags.push(tag),
            //    Err(why) => return Err(why),
            //}
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
    use rusqlite::Connection;
    use rust_decimal_macros::dec;

    use super::{add_score_and_tags, init_db_tables, list_scores};

    #[test]
    fn test_init_db() {
        let conn = get_test_conn();
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

    ///Get connection to in-memory db and ensure base tables exist
    fn get_test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db_tables(&conn).unwrap();
        conn
    }

    #[test]
    pub fn test_add_score_and_tags() {
        let conn = get_test_conn();
        let dec = dec!(7.9);
        let tags = vec![
            "test".to_string(),
            "the".to_string(),
            "tagger".to_string(),
            "before".to_string(),
            "the".to_string(),
            "tagger".to_string(),
            "tests".to_string(),
            "you".to_string(),
        ];
        add_score_and_tags(&dec, &tags, &conn);

        let scores = list_scores(&conn, None, None).unwrap();
        assert!(scores.len() == 1);
        let test_score = scores.get(0).unwrap();
        let test_dec = test_score.score;
        assert_eq!(dec, test_dec)
    }
}
