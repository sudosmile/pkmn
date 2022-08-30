use anyhow::bail;
use anyhow::Result;
use rustemon::model::pokemon::PokemonType;
use rustemon::model::pokemon::Type;
use rustemon::model::pokemon::TypeRelations;
use rustemon::model::resource::NamedApiResource;
use tokio::runtime::Handle;

use crate::CLIENT;

#[derive(PartialEq, Eq)]
pub struct MyTypeRelations {
    no_damage_from: MyTypeNameVec,
    half_damage_from: MyTypeNameVec,
    double_damage_from: MyTypeNameVec,
}

#[derive(PartialEq, Eq)]
pub struct MyType {
    name: String,
    damage_relations: MyTypeRelations,
}

#[derive(PartialEq, Eq)]
pub struct MyTypeNameVec {
    arr: Vec<String>,
}

#[derive(PartialEq, Eq)]
pub struct MyTypeVec {
    arr: Vec<MyType>,
}

impl std::fmt::Display for MyTypeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: calculate multipliers for multitype pkmn
        for i in &self.arr {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type: {}
    takes x0 dmg from: {}
    takes x0.5 dmg from: {}
    takes x2 dmg from: {}",
            self.name, 
            self.damage_relations.no_damage_from,
            self.damage_relations.half_damage_from,
            self.damage_relations.double_damage_from,
        )
    }
}

impl std::fmt::Display for MyTypeNameVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.arr {
            write!(f, "[{}]", i)?;
        }
        Ok(())
    }
}

#[allow(unused_variables)]
fn named_to_ressource<T>(named: NamedApiResource<T>) -> T
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    let handle = Handle::current();
    let enterguard = handle.enter();
    futures::executor::block_on(named.follow(&CLIENT)).unwrap()
}

impl From<Vec<PokemonType>> for MyTypeVec {
    fn from(v: Vec<PokemonType>) -> Self {
        let mut new: Vec<MyType> = vec![];
        for i in v {
            new.push(i.try_into().unwrap())
        }
        Self { arr: new }
    }
}

impl TryFrom<PokemonType> for MyType {
    type Error = anyhow::Error;

    fn try_from(value: PokemonType) -> Result<Self, Self::Error> {
        let type_: Type = named_to_ressource(match value.type_ {
            Some(named) => named,
            None => bail!("conversion failed"),
        });
        let name = type_.name.unwrap();
        let damage_relations = type_.damage_relations.unwrap().try_into()?;
        Ok(Self {
            name,
            damage_relations,
        })
    }
}

#[allow(unused_must_use)]
impl From<Vec<NamedApiResource<Type>>> for MyTypeNameVec {
    fn from(vec: Vec<NamedApiResource<Type>>) -> Self {
        let vecoftypename: Vec<String> = vec
            .iter()
            .map(|type_| -> String {
                let handle = Handle::current();
                // this is unused :(
                handle.enter();
                let type_ = futures::executor::block_on(type_.follow(&CLIENT)).unwrap();
                type_.name.unwrap()
            })
            .collect();
        Self { arr: vecoftypename }
    }
}

impl TryFrom<TypeRelations> for MyTypeRelations {
    type Error = anyhow::Error;

    fn try_from(value: TypeRelations) -> Result<Self, Self::Error> {
        let no_damage_from: MyTypeNameVec = value.no_damage_from.unwrap().into();
        let half_damage_from: MyTypeNameVec = value.half_damage_from.unwrap().into();
        let double_damage_from: MyTypeNameVec = value.double_damage_from.unwrap().into();
        Ok(Self {
            no_damage_from,
            half_damage_from,
            double_damage_from,
        })
    }
}
