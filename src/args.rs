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
    score: Decimal,
    name: String,
    #[arg(short, long)]
    tags: Vec<String>,
}
