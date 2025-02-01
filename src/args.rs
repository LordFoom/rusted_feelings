use chrono::NaiveDate;
use clap::Parser;
use rust_decimal::Decimal;
///Representation of the command line arguments
#[derive(Parser, Debug)]
#[command(
    version = "0.1",
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
    ///Tag filters for listing/charting by tag
    #[arg(short, long)]
    pub filter: Vec<String>,
}
