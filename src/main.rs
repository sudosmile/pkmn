#![allow(dead_code)]
use anyhow::Result;
use clap::{arg, command, ArgAction, ArgMatches, Command};
use log::{error, info, warn};
use rustemon::client::RustemonClient;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

mod input;
mod pkmn;

use crate::pkmn::pokemon;
use crate::pkmn::pokemon::MyPokemon;

fn app() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("name")
                .about("Get info by pokemon name (i.e: charizard-gmax) [hyphen separated]")
                .arg(arg!(<NAME> "Name of the pokemon"))
                .arg(
                    arg!(-d --direct "Directly query pokeapi with the name")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg_required_else_help(true),
        )
        .get_matches()
}

#[cfg(not(debug_assertions))]
fn logging_init() -> Result<()> {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}

#[cfg(debug_assertions)]
fn logging_init() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<()> {
    logging_init()?;
    info!("init logging");

    info!("get command line arguments");
    let matches = app();

    info!("initiate Rustemon Client (pokeapi wrapper)");
    let client = RustemonClient::default();

    info!("get list of pokemon names from rustemon");
    let pokemons_list = pokemon::names_list(&client).await?;

    info!("parse command line arguments");
    match matches.subcommand() {
        // name subcommand match
        Some(("name", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>("NAME")
                .expect("could not parse pokemon name");
            // if direct flag is set, query pokeapi directly
            if let Some(true) = sub_matches.get_one::<bool>("direct") {
                info!("requesting exact pokemon from pokeapi");
                let choice = match MyPokemon::from_name(&client, name).await {
                    Ok(choice) => choice,
                    Err(e) => {
                        error!("no pokemon with name '{}' found", name);
                        return Err(e);
                    }
                };
                println!("{}", choice);
            // no direct flag set, find closest matching pokemon name
            // (uses levenshtein distance)
            } else {
                info!("find closest matching pokemon name");
                let choice =
                    MyPokemon::closest_match_from_list(&client, &pokemons_list, name).await?;
                println!("{}", choice);
            }
            return Ok(());
        }
        // unreachable path
        Some(_) => {
            warn!("unreachable path reached -- somehow");
            unreachable!("exhausted list of subcommands and subcommand_required prevents `None`")
        }
        // no subcommand -- continue
        None => info!("no subcommand provided -- continuing"),
    }

    info!("let user choose a pokemon from the list");
    let choice = MyPokemon::from_list_with_select(&client, &pokemons_list).await?;

    println!("{}", choice);
    Ok(())
}
