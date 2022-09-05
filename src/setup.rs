use anyhow::Result;
use clap::{arg, command, ArgAction, ArgMatches, Command};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub fn app() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("name")
                .about("Get info by pokemon name (i.e: charizard-gmax) [hyphen separated]")
                .arg(arg!(<NAME> "Name of the pokemon"))
                .arg_required_else_help(true),
        )
        .get_matches()
}

#[cfg(not(debug_assertions))]
pub fn logging_init() -> Result<()> {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}

#[cfg(debug_assertions)]
pub fn logging_init() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}
