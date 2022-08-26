use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub enum Selection {
    Pokemon,
    Move,
}

pub fn get_selection() -> Result<Selection> {
    let list = ["Pokemon", "Move"];
    let input = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("=>")
        .default(0)
        .items(&list[..])
        .interact()?;
    Ok(match list[input] {
        "Pokemon" => Selection::Pokemon,
        "Move" => Selection::Move,
        _ => bail!("wrong input on selection"),
    })
}

pub fn select_index(list: &[String]) -> Result<usize> {
    let input = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("=>")
        .default(0)
        .items(list)
        .interact()?;
    Ok(input)
}
