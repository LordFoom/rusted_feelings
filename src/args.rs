use clap::{Parser, Subcommand};
use rust_decimal::Decimal;
///Representation of the command line arguments
#[derive(Parser, Debug)]
#[command(
    version = "0.1",
    about = "Mood tracker",
    long_about = "Keep track of your moods with this cli app. Give it a score and maybe tag it with feelings"
)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    AddMood {
        mood: String,
        description: Option<String>,
    },
    ///Rate your mood
    ScoreMood {
        score: Decimal,
        ///This must match an already registered mood
        mood: String,
        ///Can specify offset of days into the past to record yesterday, ereyesterday, etc
        days_back: Option<usize>,
    },
    ///List all moods in the datastore
    ListMoods,
    ///TODO show the scores
    ShowScores,
}
