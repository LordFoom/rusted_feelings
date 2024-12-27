use clap::Parser;

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::{args::AppArgs, db::init_db};
use color_eyre::Result;

mod args;
mod db;

fn init_logging(verbose: bool) -> Result<()> {
    color_eyre::install()?;
    // Configure the console appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build();

    // Configure the file appender
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build("./rusted_feelings.log")
        .expect("Failed to create file appender");

    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    // Build the `log4rs` configuration
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )
        .expect("Failed to build log4rs configuration");

    // Initialize the logger
    log4rs::init_config(config).expect("Failed to initialize logging");
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
    let args = AppArgs::parse();
    init_logging(args.verbose)?;
    let path = get_db_path(&args)?;
    let db = init_db(path)?;
    match args.command {
        args::Commands::AddMood { mood, description } => {
            db::create_mood_if_not_exists(&mood, &description, &db)?;
            //info!()
        }
        args::Commands::ScoreMood {
            score,
            mood,
            days_back,
        } => todo!(),
        args::Commands::ListMoods => todo!(),
    }
    Ok(())
}
