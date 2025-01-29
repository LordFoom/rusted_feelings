use clap::Parser;

use color_eyre::owo_colors::OwoColorize;
use log::info;
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use tabled::builder::Builder;

use crate::{args::AppArgs, db::init_db};
use color_eyre::Result;

mod args;
mod db;
mod error;

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
                .build(level),
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
fn get_db_path(_args: &AppArgs) -> Result<&str> {
    Ok(".rusted_feelings.db")
}
//TODO refactor some methods out, like a pro would
fn main() -> Result<()> {
    let args = AppArgs::parse();
    init_logging(args.verbose)?;
    let path = get_db_path(&args)?;
    let db = init_db(path)?;
    //if list is presnt
    if args.list {
        let scores = db::list_scores(&db.conn, args.start, args.end)?;
        //trying to turn the bottom into a manual row because i cannot derive because of a vec of
        //strings
        //being vexed by header
        let table_rows = scores
            .into_iter()
            .map(|score| (score.create_date, score.score, score.tags));

        let mut table_builder = Builder::default();
        table_builder.push_record(["Date", "Score", "Tags"]);
        for (score_date, score, score_tags) in table_rows {
            table_builder.push_record([
                score_date.to_string(),
                score.to_string(),
                score_tags.join(","),
            ]);
        }

        let table = table_builder.build();
        println!("{table}");
    }

    if let Some(arg_score) = args.score {
        db::add_score_and_tags(&arg_score, &args.tags, &db.conn)?;
        info! {"Added score {} with tags {:?}", arg_score.cyan(), args.tags.yellow()};
    }
    Ok(())
}
