use rustemon::model::pokemon::PokemonType;
use rustemon::model::resource::NamedApiResource;

pub enum Type {
    Normal,
    Psychic,
    Fire,
    Bug,
    Water,
    Rock,
    Grass,
    Ghost,
    Electric,
    Dark,
    Ice,
    Dragon,
    Fighting,
    Steel,
    Poison,
    Fairy,
    Ground,
    Flying,
}

pub struct Types {
    arr: Vec<Type>,
}

impl From<Vec<PokemonType>> for Types {
    fn from(v: Vec<PokemonType>) -> Self {
        let mut ret = Self {
            arr: Vec::with_capacity(v.len()),
        };
        for i in v {
            ret.arr.push(i.into())
        }
        ret
    }
}

impl From<NamedApiResource<rustemon::model::pokemon::Type>> for Type {
    fn from(
        t: rustemon::model::resource::NamedApiResource<rustemon::model::pokemon::Type>,
    ) -> Self {
        let name = match t.name {
            Some(name) => name,
            None => panic!(),
        };
        name.into()
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "normal" => Self::Normal,
            "psychic" => Self::Psychic,
            "fire" => Self::Fire,
            "bug" => Self::Bug,
            "water" => Self::Water,
            "rock" => Self::Rock,
            "grass" => Self::Grass,
            "ghost" => Self::Ghost,
            "electric" => Self::Electric,
            "dark" => Self::Dark,
            "ice" => Self::Ice,
            "dragon" => Self::Dragon,
            "fighting" => Self::Fighting,
            "steel" => Self::Steel,
            "poison" => Self::Poison,
            "fairy" => Self::Fairy,
            "ground" => Self::Ground,
            "flying" => Self::Flying,
            _ => panic!(),
        }
    }
}

impl From<PokemonType> for Type {
    fn from(t: PokemonType) -> Self {
        let namedapiressource = match t.type_ {
            Some(t) => t,
            None => panic!(),
        };
        let name = match namedapiressource.name {
            Some(name) => name,
            None => panic!(),
        };
        name.into()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Normal => "Normal",
                Type::Psychic => "Psychic",
                Type::Fire => "Fire",
                Type::Bug => "Bug",
                Type::Water => "Water",
                Type::Rock => "Rock",
                Type::Grass => "Grass",
                Type::Ghost => "Ghost",
                Type::Electric => "Electric",
                Type::Dark => "Dark",
                Type::Ice => "Ice",
                Type::Dragon => "Dragon",
                Type::Fighting => "Fighting",
                Type::Steel => "Steel",
                Type::Poison => "Poison",
                Type::Fairy => "Fairy",
                Type::Ground => "Ground",
                Type::Flying => "Flying",
            }
        )
    }
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.arr {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}
