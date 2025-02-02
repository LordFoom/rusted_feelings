use charming::{component::Title, Chart};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use indexmap::IndexMap;
use rust_decimal::Decimal;

use crate::db::Score;
use anyhow::Result;

//use charminggcc
///Create chart object
pub fn construct_chart(scores: &mut Vec<Score>) -> Result<Chart> {
    //we want to split the scores into buckets, where each bucket is a date
    //should come in sorted, but we make double sure
    scores.sort_by_key(|s| s.create_date);
    let mut curr_date = Utc::now().date_naive();
    let mut graph_buckets: IndexMap<NaiveDate, Vec<Decimal>> = IndexMap::new();
    for score in scores {
        let key_date = if score.create_date.date() != curr_date {
            curr_date = score.create_date.date().clone();
            score.create_date.date()
        } else {
            curr_date
        };
        if let Some(graph_score) = graph_buckets.get_mut(&key_date) {
            graph_score.push(score.score);
        } else {
            let mut score_vec = Vec::new();
            score_vec.push(score.score);
            graph_buckets.insert(key_date, score_vec);
        }
    }

    let chart = Chart::new().title(Title::new().top("Score Chart"));

    Ok(chart)
}
