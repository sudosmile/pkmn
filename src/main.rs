use anyhow::Result;
use futures::future::join_all;
use http_cache::CacheMode;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use online::check;
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
            .display()
            .to_string();
        RustemonClient::new_path_unchecked(cache_path, CacheMode::Default, None)
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    info!("get command line arguments");
    let matches = setup::app();

    setup::logging_init(*matches.get_one::<u8>("debug").unwrap_or(&0u8))?;
    info!("init logging");

    if check(None).await.is_err() {
        warn!("No internet connection, Pages not found in Cache will not be obtainable")
    }

    info!("get list of pokemon names from rustemon");
    let pokemons_list = pokemon::names_list().await?;

    // TODO: implement some way to view cache building progress (with channels?)
    if let Some(true) = matches.get_one::<bool>("build") {
        info!("generating cache");
        println!("Generating Cache...");
        join_all(pokemons_list.iter().map(|p| {
            debug!("getting {}", p);
            rustemon::pokemon::pokemon::get_by_name(p, &CLIENT)
        }))
        .await;
        info!("Done building cache");
        return Ok(());
    }

    let choice: MyPokemon = {
        // different input method depending on flags
        if let Some(name) = matches.get_one::<String>("NAME") {
            info!("find closest matching pokemon name");
            MyPokemon::closest_match_from_list(&pokemons_list, name).await?
        } else {
            info!("let user choose a pokemon from the list");
            MyPokemon::from_list_with_select(&pokemons_list).await?
        }
    };

    // print the info of the chosen pokemon
    println!("{}", choice);
    Ok(())
}
