use charming::{component::Title, series::Line, Chart, ImageRenderer};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use indexmap::IndexMap;
use log::debug;
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
    debug!("Charting {} scores", scores.len());
    for score in scores {
        let key_date = if score.create_date.date() != curr_date {
            curr_date = score.create_date.date().clone();
            score.create_date.date()
        } else {
            curr_date
        };

        debug!("Current date, our column: {}", key_date);
        if let Some(graph_score) = graph_buckets.get_mut(&key_date) {
            graph_score.push(score.score);
        } else {
            let mut score_vec = Vec::new();
            score_vec.push(score.score);
            graph_buckets.insert(key_date, score_vec);
        }
    }
    let graph_data = graph_buckets
        .into_iter()
        .map(|(key, val)| {
            let length = Decimal::new(val.len() as i64, 2);
            let total: Decimal = val.into_iter().fold(Decimal::from(0), |acc, d| acc + d);
            let avg = total / length;
            (key.to_string(), avg.to_string())
        })
        .collect::<Vec<(String, String)>>();
    debug!("This is the final GraphData: {:?}", graph_data);

    let chart = Chart::new()
        .title(Title::new().top("Score Chart"))
        .series(Line::new().name("Mood trend").data(graph_data));

    Ok(chart)
}

///Draw a chart to
pub fn draw_chart(file: &str, chart: &Chart) -> Result<()> {
    let mut renderer = ImageRenderer::new(640, 480);
    renderer.save(chart, file)?;
    Ok(())
}
