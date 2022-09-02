#![allow(dead_code)]

use anyhow::Result;
use http_cache::CacheMode;
use lazy_static::lazy_static;
use log::{error, info, warn};
use rustemon::client::RustemonClient;

mod input;
mod pkmn;
mod setup;

use crate::pkmn::pokemon;
use crate::pkmn::pokemon::MyPokemon;

lazy_static! {
    static ref CLIENT: RustemonClient = {
        info!("init rustemon client");
        let cache_path = dirs::cache_dir()
            .unwrap()
            .join("pkmn")
            .to_str()
            .unwrap()
            .to_string();
        RustemonClient::new_path_unchecked(cache_path, CacheMode::Default, None)
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    setup::logging_init()?;
    info!("init logging");

    info!("get command line arguments");
    let matches = setup::app();

    info!("get list of pokemon names from rustemon");
    let pokemons_list = pokemon::names_list().await?;

    info!("parse command line arguments");
    let choice: MyPokemon = match matches.subcommand() {
        // name subcommand match
        Some(("name", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>("NAME")
                .expect("could not parse pokemon name");

            if let Some(true) = sub_matches.get_one::<bool>("direct") {
                // if direct flag is set, query pokeapi directly
                info!("requesting exact pokemon from pokeapi");
                match MyPokemon::from_name(name).await {
                    Ok(choice) => choice,
                    Err(e) => {
                        error!("no pokemon with name '{}' found", name);
                        return Err(e);
                    }
                }
            } else {
                // no direct flag set, find closest matching pokemon name
                // (uses levenshtein distance)
                info!("find closest matching pokemon name");
                MyPokemon::closest_match_from_list(&pokemons_list, name).await?
            }
        }
        // unreachable path
        Some(_) => {
            warn!("unreachable path reached -- somehow");
            unreachable!("exhausted list of subcommands and subcommand_required prevents `None`")
        }
        // no subcommand -- continue
        None => {
            info!("let user choose a pokemon from the list");
            let choice = MyPokemon::from_list_with_select(&pokemons_list).await?;
            choice
        }
    };

    // print the info of the chosen pokemon
    println!("{}", choice);
    Ok(())
}
