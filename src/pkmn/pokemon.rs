use rustemon::model::pokemon::Pokemon;

use crate::input;
use super::types::Types;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use indicatif::ProgressBar;
use rustemon::client::RustemonClient;

pub struct MyPokemon {
    name: String,
    types: Types,
    base_exp: i64,
    height: i64,
    weight: i64,
}

async fn list_len(client: &RustemonClient) -> Result<i64> {
    let pokemon_page = rustemon::pokemon::pokemon::get_page(client).await?;
    Ok(match pokemon_page.count {
        Some(n) => n,
        None => bail!("failed to get length of list of all pokemon"),
    })
}

pub async fn names_list(client: &RustemonClient) -> Result<Vec<String>> {
    let len = list_len(client).await?;
    let mut list_of_pokemon: Vec<String> = Vec::with_capacity(len.try_into()?);
    let mut offset: i64 = 0;
    let limit = 100;
    let bar = ProgressBar::new(len.try_into()?);

    while offset < len {
        let current_page =
            rustemon::pokemon::pokemon::get_page_with_param(offset, limit, client).await?;

        for pokemon in current_page.results.unwrap() {
            list_of_pokemon.push(pokemon.name.ok_or_else(|| anyhow!("unnamed pokemon"))?);
        }
        offset += limit;
        bar.inc(limit.try_into()?);
    }
    // (#1) replace the dashes with spaces for easier input (see #2)
    list_of_pokemon = list_of_pokemon
        .iter()
        .map(|s| s.replace('-', " "))
        .collect();
    Ok(list_of_pokemon)
}

// TODO: make generic
impl MyPokemon {
    pub async fn from_list_with_select(
        client: &RustemonClient,
        list: &[String],
    ) -> Result<MyPokemon> {
        let pkmn_index = input::select_index(list)?;
        // (#2) replace the spaces with dashes for getting info from the pokeapi (see #1)
        let pkmn_name = list[pkmn_index].replace(' ', "-");
        let my_pokemon: MyPokemon = rustemon::pokemon::pokemon::get_by_name(&pkmn_name, client)
            .await?
            .into();
        Ok(my_pokemon)
    }
}

impl From<Pokemon> for MyPokemon {
    fn from(pkmn: Pokemon) -> Self {
        Self {
            name: match pkmn.name {
                Some(name) => name,
                None => panic!(),
            },
            types: match pkmn.types {
                Some(types) => types.into(),
                None => panic!(),
            },
            base_exp: match pkmn.base_experience {
                Some(exp) => exp,
                None => panic!(),
            },
            height: match pkmn.height {
                Some(height) => height,
                None => panic!(),
            },
            weight: match pkmn.weight {
                Some(weight) => weight,
                None => panic!(),
            },
        }
    }
}

impl std::fmt::Display for MyPokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {name}
type: {types}
base_exp: {base_exp}
height: {height}
weight: {weight}",
            name = self.name,
            types = self.types,
            base_exp = self.base_exp,
            height = self.height,
            weight = self.weight
        )
    }
}
