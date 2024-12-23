use clap::Parser;

use crate::{args::AppArgs, db::init_db};
use anyhow::Result;

mod args;
mod db;

fn init_logging() -> Result<()> {
    Ok(())
}

///Get the path of the datastore,
/// a sqlite file. Takes the parsed
/// args as an arg so it can check if a config file
/// was passed in as an argument on the cli.
fn get_db_path(args: &AppArgs) -> Result<&str> {
    Ok(".rusted_feelings.db")
}
//TODO Git repo
//Accept arguments
//Create argument model;
fn main() -> Result<()> {
    init_logging()?;
    let args = AppArgs::parse();
    let path = get_db_path(&args)?;
    let db = init_db(path)?;
    match args.command {
        args::Command::AddMood { mood, description } => db::create_mood_if_not_exists(mood, &db),
        args::Command::ScoreMood {
            score,
            mood,
            days_back,
        } => todo!(),
        args::Command::ListMoods => todo!(),
    }
    Ok(())
}
