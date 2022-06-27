use std::{collections::HashMap, sync::{Arc, Mutex}, error::Error};

use super::structs::{BasePokemon, PokeType, MoveType, DataFieldNotFoundError};

pub(super) type Pokedex = Arc<Mutex<HashMap<u8,BasePokemon>>>;
pub(super)type Typedex = Arc<Mutex<HashMap<u8,PokeType>>>;

#[derive(Clone)]
pub struct Data{
    pub pokedex: Pokedex,
    pub typedex: Typedex,
}

pub(super) fn build_pokemon<'a>(pokemon_json: &'a serde_json::Value, typedex: Typedex) -> Option<BasePokemon> {
    if let Ok(types) = typedex.try_lock() {
        let pokemon = BasePokemon { 
            index: pokemon_json["index"].as_i64().map(|x| x as u8)?,
            pokedex: pokemon_json["pokedex"].as_i64().map(|x| x as u8)?,
            name: pokemon_json["name"].as_str()?.to_string(), 
            base_hp: pokemon_json["base_stats"]["hp"].as_i64().map(|x| x as i32)?, 
            base_attack: pokemon_json["base_stats"]["attack"].as_i64().map(|x| x as i32)?, 
            base_defense: pokemon_json["base_stats"]["defense"].as_i64().map(|x| x as i32)?, 
            base_speed: pokemon_json["base_stats"]["speed"].as_i64().map(|x| x as i32)?, 
            base_special: pokemon_json["base_stats"]["special"].as_i64().map(|x| x as i32)?, 

            type1: types.get(&pokemon_json["types_id"][0].as_i64().map(|x| x as u8).unwrap())?.clone(), 
            type2: if pokemon_json["types_id"][1] != pokemon_json["types_id"][0] {Some(types.get(&pokemon_json["types_id"][1].as_i64().map(|x| x as u8).unwrap())?.clone())} else {None},
        };
        return Some(pokemon);
    };
    return None;
}

pub(super) fn build_type<'a>(poketype_json: &'a serde_json::Value) -> Result<PokeType, Box<dyn Error>>{
    return Ok(PokeType {
        index: poketype_json["id"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        name: poketype_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        category: match poketype_json["category"].as_str().ok_or_else(|| DataFieldNotFoundError)? {
            "physical" => MoveType::Physical,
            "special" => MoveType::Special,
            _ => return Err(DataFieldNotFoundError.into()),
        },
        strong: poketype_json["strong"].as_array().ok_or_else(|| DataFieldNotFoundError)?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
        weak: poketype_json["weak"].as_array().ok_or_else(|| DataFieldNotFoundError)?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
        no_effect: poketype_json["no_effect"].as_array().ok_or_else(|| DataFieldNotFoundError)?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
    });
}
