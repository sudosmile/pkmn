pub mod moves;
pub mod pokemon;
mod types;

pub enum PkmnObject {
    Pokemon(pokemon::MyPokemon),
    Move(moves::MyMove),
}

impl From<pokemon::MyPokemon> for PkmnObject {
    fn from(p: pokemon::MyPokemon) -> Self {
        Self::Pokemon(p)
    }
}

impl From<moves::MyMove> for PkmnObject {
    fn from(m: moves::MyMove) -> Self {
        Self::Move(m)
    }
}

impl std::fmt::Display for PkmnObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PkmnObject::Pokemon(pokemon) => format!("{}", pokemon),
                PkmnObject::Move(move_) => format!("{}", move_),
            }
        )
    }
}
