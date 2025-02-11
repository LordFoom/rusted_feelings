use std::str::FromStr;

use chrono::NaiveDate;
use color_eyre::Result;
use log::debug;
use rusqlite::{params, params_from_iter, Connection};
use rust_decimal::Decimal;

use crate::error::AppError;

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
            conn,
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
        "CREATE TABLE IF NOT EXISTS score (id integer primary key asc, create_date default current_timestamp, score TEXT)";
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
///Optionally filtered by tags,start- and end_date
pub fn list_scores(
    conn: &Connection,
    filter_tags: &Vec<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<Score>, AppError> {
    let mut sql = "SELECT id, score, create_date FROM score WHERE 1=1 ".to_string();
    let mut parms = Vec::new();
    if let Some(dt) = start_date {
        sql.push_str(" AND create_date >= ?");
        parms.push(dt);
    };
    if let Some(dt) = end_date {
        sql.push_str(" AND create_date < ?");
        parms.push(dt);
    };
    if !filter_tags.is_empty() {
        let mut tags_string =
            " AND id in (select score_id from tags where tags.name in (?".to_string();

        for _ in 1..filter_tags.len() {
            tags_string.push_str(",?");
        }
        //remove the final character
        tags_string.push_str("))");
        sql.push_str(&tags_string);
    }

    debug!("sql: {}", sql);
    debug!("tags {:?}", filter_tags);
    let tag_sql = "SELECT name from tag where score_id = ? ";
    let mut stmt = conn.prepare(&sql)?;
    let mut tag_stmt = conn.prepare(tag_sql)?;
    //TODO add the dates
    let mut scores = Vec::new();
    let params = params_from_iter(filter_tags);

    let mut rows = stmt.query(params)?;
    while let Some(row) = rows.next()? {
        let score_str: String = row.get(1)?;
        let score_dec = Decimal::from_str(&score_str).map_err(AppError::from)?;
        let mut score = Score {
            id: row.get(0)?,
            score: score_dec,
            create_date: row.get(2)?,
            tags: Vec::new(),
        };

        let mut tags = Vec::new();
        let mut tags_iter = tag_stmt.query([score.id])?;
        while let Some(tag_row) = tags_iter.next()? {
            let tag = tag_row.get(0)?;
            tags.push(tag);
        }
        score.tags = tags;
        scores.push(score);
    }
    //for score_wrap in score_rows {
    //    let score = score_wrap.unwrap();
    //    scores.push(score);
    //}
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
        add_score_and_tags(&dec, &tags, &conn).unwrap();

        let tag_filters = Vec::new();
        let scores = list_scores(&conn, &tag_filters, None, None).unwrap();
        assert!(scores.len() == 1);
        let test_score = scores.get(0).unwrap();
        let test_dec = test_score.score;
        assert_eq!(dec, test_dec)
    }
}
