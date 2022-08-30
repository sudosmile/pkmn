use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn fuzzy_select<T>(list: &[T]) -> Result<&T> 
where T: std::fmt::Display {
    let input = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("=>")
        .default(0)
        .items(list)
        .interact()?;
    Ok(&list[input])
}
