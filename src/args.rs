use chrono::NaiveDate;
use clap::{parser, Args, Parser};
use rust_decimal::Decimal;
///Representation of the command line arguments
#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Mood tracker",
    long_about = "Keep track of your moods with this cli app. Give it a score and maybe tag it with feelings"
)]
pub struct AppArgs {
    pub score: Option<Decimal>,
    ///What tags you want associated
    #[arg(short, long)]
    pub tags: Vec<String>,
    ///Spew debug info
    #[arg(short, long)]
    pub verbose: bool,
    ///Show a table of scores
    #[arg(short, long)]
    pub list: bool,
    ///Start date for listing/charting
    #[arg(short, long)]
    pub start: Option<NaiveDate>,
    ///End date for listing/charting
    #[arg(short, long)]
    pub end: Option<NaiveDate>,
    ///generate a chart
    #[arg(short, long)]
    pub chart: bool,
    ///Filter tags, or no_filter
    #[command(flatten)]
    pub filtered: Filtered,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct Filtered {
    ///Tag filters for listing/charting scores by tag
    #[arg(short, long)]
    pub filters: Vec<String>,
    ///Only list scores without tags
    #[arg(short, long)]
    pub no_filter: bool,
}
