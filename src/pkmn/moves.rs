use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use indicatif::ProgressBar;
use rustemon::client::RustemonClient;

use rustemon::model::moves::Move;

use crate::input;
use super::types::Type;

pub struct MyMove {
    name: String,
    type_: Type,
    accuracy: i64,
    pp: i64,
    priority: i64,
    power: i64,
}

async fn list_len(client: &RustemonClient) -> Result<i64> {
    let pokemon_page = rustemon::moves::move_::get_page(client).await?;
    Ok(match pokemon_page.count {
        Some(n) => n,
        None => bail!("failed to get length of list of all moves"),
    })
}

pub async fn names_list(client: &RustemonClient) -> Result<Vec<String>> {
    let len = list_len(client).await?;
    let mut list_of_moves: Vec<String> = Vec::with_capacity(len.try_into()?);
    let mut offset: i64 = 0;
    let limit = 100;
    let bar = ProgressBar::new(len.try_into()?);

    while offset < len {
        let current_page =
            rustemon::moves::move_::get_page_with_param(offset, limit, client).await?;

        for move_ in current_page.results.unwrap() {
            list_of_moves.push(move_.name.ok_or_else(|| anyhow!("unnamed move"))?);
        }
        offset += limit;
        bar.inc(limit.try_into()?);
    }
    // (#1) replace the dashes with spaces for easier input (see #2)
    list_of_moves = list_of_moves.iter().map(|s| s.replace('-', " ")).collect();
    Ok(list_of_moves)
}

impl MyMove {
    pub async fn from_list_with_select(client: &RustemonClient, list: &[String]) -> Result<MyMove> {
        let pkmn_index = input::select_index(list)?;
        // (#2) replace the spaces with dashes for getting info from the pokeapi (see #1)
        let pkmn_name = list[pkmn_index].replace(' ', "-");
        let my_pokemon: MyMove = rustemon::moves::move_::get_by_name(&pkmn_name, client)
            .await?
            .into();
        Ok(my_pokemon)
    }
}

impl std::fmt::Display for MyMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {name}
type: {type_}
accuracy: {accuracy}
pp: {pp}
power: {power}
priority: {priority}",
            name = self.name,
            type_ = self.type_,
            accuracy = self.accuracy,
            pp = self.pp,
            power = self.power,
            priority = self.priority
        )
    }
}

impl From<Move> for MyMove {
    fn from(mv: Move) -> Self {
        Self {
            name: match mv.name {
                Some(name) => name,
                None => panic!(),
            },
            type_: match mv.type_ {
                Some(types) => types.into(),
                None => panic!(),
            },
            accuracy: match mv.accuracy {
                Some(acc) => acc,
                None => panic!(),
            },
            pp: match mv.pp {
                Some(pp) => pp,
                None => panic!(),
            },
            priority: match mv.priority {
                Some(priority) => priority,
                None => panic!(),
            },
            power: mv.power.unwrap_or(0),
        }
    }
}
