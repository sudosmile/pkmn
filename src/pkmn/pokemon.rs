use rustemon::model::pokemon::Pokemon;

use super::types::Types;
use crate::input;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use indicatif::ProgressBar;
use rustemon::client::RustemonClient;

#[allow(dead_code)]
pub struct MyPokemon {
    id: i64,
    order: i64,
    name: String,
    types: Types,
    base_exp: i64,
    height: i64,
    weight: i64,
}

pub async fn names_list(client: &RustemonClient) -> Result<Vec<String>> {
    let len = {
        let pokemon_page = rustemon::pokemon::pokemon::get_page(client).await?;
        match pokemon_page.count {
            Some(n) => n,
            None => bail!("failed to get length of list of all pokemon"),
        }
    };
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

impl MyPokemon {
    pub async fn from_list_with_select(
        client: &RustemonClient,
        list: &[String],
    ) -> Result<MyPokemon> {
        let pkmn_name = input::fuzzy_select(list)?.replace(' ', "-");
        // (#2) replace the spaces with dashes for getting info from the pokeapi (see #1)
        let my_pokemon: MyPokemon = rustemon::pokemon::pokemon::get_by_name(&pkmn_name, client)
            .await?
            .try_into()?;
        Ok(my_pokemon)
    }
}

impl TryFrom<Pokemon> for MyPokemon {
    type Error = anyhow::Error;

    fn try_from(pkmn: Pokemon) -> Result<Self, Self::Error> {
        let name = match pkmn.name {
            Some(name) => name,
            None => bail!("failed to parse pokemon struct"),
        };
        let types: Types = match pkmn.types {
            Some(types) => types.into(),
            None => bail!("failed to parse pokemon struct"),
        };
        let base_exp = match pkmn.base_experience {
            Some(exp) => exp,
            None => bail!("failed to parse pokemon struct"),
        };
        let height = match pkmn.height {
            Some(height) => height,
            None => bail!("failed to parse pokemon struct"),
        };
        let weight = match pkmn.weight {
            Some(weight) => weight,
            None => bail!("failed to parse pokemon struct"),
        };
        let id = match pkmn.id {
            Some(id) => id,
            None => bail!("failed to parse pokemon struct"),
        };
        let order = match pkmn.order {
            Some(order) => order,
            None => bail!("failed to parse pokemon struct"),
        };
        Ok(Self {
            id,
            order,
            name,
            types,
            base_exp,
            height,
            weight,
        })
    }
}

impl std::fmt::Display for MyPokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {id}
name: {name}
type: {types}
base_exp: {base_exp}
height: {height}
weight: {weight}",
            id = self.id,
            name = self.name,
            types = self.types,
            base_exp = self.base_exp,
            height = self.height,
            weight = self.weight
        )
    }
}
