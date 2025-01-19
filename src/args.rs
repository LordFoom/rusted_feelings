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
    #[arg(short, long)]
    pub tags: Vec<String>,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long)]
    pub start: Option<NaiveDate>,
    #[arg(short, long)]
    pub end: Option<NaiveDate>,
}
