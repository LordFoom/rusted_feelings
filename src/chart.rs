use charming::{
    component::{Legend, Title},
    Chart,
};

use crate::db::Score;

//use charminggcc
///Create chart object
pub fn construct_chart(scores: Vec<Score>) -> Result<Chart> {
    let chart = Chart::new().title(Title::new().top("Score Chart"));
    Ok(chart)
}
