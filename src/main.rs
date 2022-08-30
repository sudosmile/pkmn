#![allow(dead_code)]
use anyhow::Result;
use log::{error, info, warn};
use rustemon::client::RustemonClient;
use lazy_static::lazy_static;

mod input;
mod pkmn;
mod setup;

use crate::pkmn::pokemon;
use crate::pkmn::pokemon::MyPokemon;

lazy_static! {
    static ref CLIENT: RustemonClient = {
        info!("init rustemon client");
        RustemonClient::default()
    };
}

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<()> {
    setup::logging_init()?;
    info!("init logging");

    info!("get command line arguments");
    let matches = setup::app();

    info!("get list of pokemon names from rustemon");
    let pokemons_list = pokemon::names_list().await?;

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
                let choice = match MyPokemon::from_name(name).await {
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
                    MyPokemon::closest_match_from_list(&pokemons_list, name).await?;
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
    let choice = MyPokemon::from_list_with_select(&pokemons_list).await?;

    println!("{}", choice);
    Ok(())
}
