use anyhow::Result;
use clap::{arg, ArgAction, ArgMatches, command};
use pkmn::PkmnObject;

mod input;
mod pkmn;

use crate::input::Selection;
use crate::pkmn::{moves, pokemon};

fn app() -> ArgMatches {
    command!()
        .arg(
            arg!(-t --test "test flag (no effect)")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<()> {
    let matches = app();

    let client = rustemon::client::RustemonClient::default();

    // ask user to chose between the different types of accessible data
    let response: PkmnObject = match input::get_selection() {
        Ok(Selection::Pokemon) => {
            let pokemons_list = pokemon::names_list(&client).await?;
            pokemon::MyPokemon::from_list_with_select(&client, &pokemons_list)
                .await?
                .into()
        }
        Ok(Selection::Move) => {
            let moves_list = moves::names_list(&client).await?;
            moves::MyMove::from_list_with_select(&client, &moves_list)
                .await?
                .into()
        }
        Err(e) => return Err(e),
    };

    println!("{}", response);
    Ok(())
}
