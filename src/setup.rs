use anyhow::Result;
use clap::{arg, command, ArgAction, ArgMatches};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub fn app() -> ArgMatches {
    command!()
        .arg(
            arg!(
                -d --debug "Turn debugging information on (each use means more info)"
            )
            .action(ArgAction::Count)
        )
        .arg(arg!(-b --build "fill the cache for future use").action(ArgAction::SetTrue))
        .arg(arg!(<NAME> "Name of the pokemon").required(false))
        .get_matches()
}

pub fn logging_init(level: u8) -> Result<()> {
    TermLogger::init(
        match level {
            0 => LevelFilter::Off,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        },
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}
