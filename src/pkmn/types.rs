use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use rustemon::model::pokemon::PokemonType;
use rustemon::model::pokemon::Type;
use rustemon::model::pokemon::TypeRelations;
use rustemon::model::resource::NamedApiResource;
use tokio::runtime::Handle;

use crate::CLIENT;

#[derive(PartialEq, Eq, Clone)]
pub struct MyTypeRelations {
    no_damage_from: MyTypeNameVec,
    half_damage_from: MyTypeNameVec,
    double_damage_from: MyTypeNameVec,
}

#[derive(PartialEq, Eq, Clone)]
pub struct MyType {
    name: String,
    damage_relations: MyTypeRelations,
}

#[derive(PartialEq, Eq, Clone)]
pub struct MyTypeNameVec {
    arr: Vec<String>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct MyTypeVec {
    arr: Vec<MyType>,
}

/// helper function that follows the api link to the ressource
/// (an asynchronous task) in a sync function
///
/// # Examples
/// ```
/// let type_named: NamedApiResource<Type>;
/// let type_: Type = named_to_ressource(type_named);
/// ```
///
#[allow(unused_variables)]
fn named_to_ressource<T>(named: &NamedApiResource<T>) -> Result<T>
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    let handle = Handle::current();
    let enterguard = handle.enter();
    Ok(futures::executor::block_on(named.follow(&CLIENT))?)
}

impl std::fmt::Display for MyTypeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl TryFrom<Vec<PokemonType>> for MyTypeVec {
    type Error = anyhow::Error;

    fn try_from(v: Vec<PokemonType>) -> Result<MyTypeVec> {
        let mut new: Vec<MyType> = vec![];
        for type_ in v {
            new.push(type_.try_into()?)
        }
        Ok(Self { arr: new })
    }
}

impl TryFrom<PokemonType> for MyType {
    type Error = anyhow::Error;

    fn try_from(value: PokemonType) -> Result<Self, Self::Error> {
        let original = match value.type_ {
            Some(name) => name,
            None => bail!("conversion failed"),
        };
        let type_: Type = named_to_ressource(&original)?;
        let name = type_.name.ok_or_else(|| anyhow!("unwrapped a None"))?;
        let damage_relations = type_
            .damage_relations
            .ok_or_else(|| anyhow!("unwrapped a None"))?
            .try_into()?;
        Ok(Self {
            name,
            damage_relations,
        })
    }
}

#[allow(unused_must_use)]
impl TryFrom<Vec<NamedApiResource<Type>>> for MyTypeNameVec {
    type Error = anyhow::Error;

    fn try_from(source: Vec<NamedApiResource<Type>>) -> Result<MyTypeNameVec> {
        let mut type_names_vec: Vec<String> = vec![];
        for i in source {
            type_names_vec.push(
                named_to_ressource(&i)?
                    .name
                    .ok_or_else(|| anyhow!("unwrapped a None"))?,
            )
        }
        Ok(Self {
            arr: type_names_vec,
        })
    }
}

impl TryFrom<TypeRelations> for MyTypeRelations {
    type Error = anyhow::Error;

    fn try_from(value: TypeRelations) -> Result<Self, Self::Error> {
        let no_damage_from: MyTypeNameVec = value
            .no_damage_from
            .ok_or_else(|| anyhow!("unwrapped a None"))?
            .try_into()?;
        let half_damage_from: MyTypeNameVec = value
            .half_damage_from
            .ok_or_else(|| anyhow!("unwrapped a None"))?
            .try_into()?;
        let double_damage_from: MyTypeNameVec = value
            .double_damage_from
            .ok_or_else(|| anyhow!("unwrapped a None"))?
            .try_into()?;
        Ok(Self {
            no_damage_from,
            half_damage_from,
            double_damage_from,
        })
    }
}
