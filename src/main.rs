use anyhow::{bail, Result};

mod input;
mod pkmn;

use input::Selection;
use pkmn::{pokemon, moves};


#[tokio::main]
async fn main() -> Result<()> {
    let client = rustemon::client::RustemonClient::default();

    match input::get_selection() {
        Ok(Selection::Pokemon) => {
            let pokemons_list = pokemon::names_list(&client).await?;
            let pokemon = pokemon::MyPokemon::from_list_with_select(&client, &pokemons_list).await?;
            println!("{}", pokemon);
        }
        Ok(Selection::Move) => {
            let moves_list = moves::names_list(&client).await?;
            let move_ = moves::MyMove::from_list_with_select(&client, &moves_list).await?;
            println!("{}", move_);
        }
        Err(_) => bail!("Selection resulted in Error"),
    }
    Ok(())
}
