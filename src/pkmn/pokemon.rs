use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use indicatif::ProgressBar;
use rustemon::model::pokemon::Pokemon;
use triple_accel::rdamerau_exp;

use crate::input;

use super::types::MyTypeVec;

#[allow(dead_code)]
pub struct MyPokemon {
    id: i64,
    order: i64,
    name: String,
    types: MyTypeVec,
    base_exp: i64,
    height: i64,
    weight: i64,
}

pub async fn names_list() -> Result<Vec<String>> {
    // TODO: use async better to make this faster
    let client = &crate::CLIENT;
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
    Ok(list_of_pokemon)
}

impl MyPokemon {
    pub async fn from_list_with_select(list: &[String]) -> Result<MyPokemon> {
        let client = &crate::CLIENT;
        let pkmn_name = input::fuzzy_select(list)?;
        // (#2) replace the spaces with dashes for getting info from the pokeapi (see #1)
        let my_pokemon: MyPokemon = rustemon::pokemon::pokemon::get_by_name(pkmn_name, client)
            .await?
            .try_into()?;
        Ok(my_pokemon)
    }

    pub async fn closest_match_from_list(list: &[String], name: &str) -> Result<MyPokemon> {
        let client = &crate::CLIENT;
        let mut lowest_distance = 999;
        let mut closest_name: &str = "";
        for i in list {
            // use damereau-levenshtein algorithm to calculate distance between the strings
            let distance = rdamerau_exp(name.as_bytes(), i.as_str().as_bytes());
            if distance < lowest_distance {
                lowest_distance = distance;
                closest_name = i;
            };
        }
        let my_pokemon: MyPokemon = rustemon::pokemon::pokemon::get_by_name(closest_name, client)
            .await?
            .try_into()?;
        Ok(my_pokemon)
    }

    pub async fn from_name(name: &str) -> Result<MyPokemon> {
        let client = &crate::CLIENT;
        let my_pokemon: MyPokemon = rustemon::pokemon::pokemon::get_by_name(name, client)
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
        let types: MyTypeVec = match pkmn.types {
            Some(types) => types.try_into()?,
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
base_exp: {base_exp}
height: {height}
weight: {weight}
{types}",
            id = self.id,
            name = self.name,
            types = self.types,
            base_exp = self.base_exp,
            height = self.height,
            weight = self.weight
        )
    }
}
