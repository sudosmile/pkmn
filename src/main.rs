use std::io;

use anyhow::Result;
use clap::{arg, command, ArgAction, ArgMatches};
use pkmn::pokemon::MyPokemon;
use rustemon::client::RustemonClient;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use log::{info, trace, warn};

mod input;
mod pkmn;

use crate::pkmn::pokemon;

fn app() -> ArgMatches {
    command!()
        .arg(arg!(-t --test "test flag (no effect)").action(ArgAction::SetTrue))
        .get_matches()
}

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    info!("get command line arguments");
    let matches = app();

    info!("initiate PokeApi Client through rustemon");
    let client = RustemonClient::default();
    info!("get list of pokemon from rustemon");
    let pokemons_list = pokemon::names_list(&client).await?;

    info!("let user choose a pokemon from the list");
    let choice = MyPokemon::from_list_with_select(&client, &pokemons_list).await?;

    println!("{}", choice);
    Ok(())
}
