use std::{collections::HashMap, sync::{Arc, Mutex}, error::Error, rc::Rc};

use super::structs::{BasePokemon, PokeType, MoveType, DataFieldNotFoundError, GameState, Move};

pub(super) type Pokedex = Arc<HashMap<u8,BasePokemon>>;
pub(super) type Movedex = Arc<HashMap<u8,Arc<Move>>>;
pub(super) type Typedex = Arc<HashMap<u8,Arc<PokeType>>>;
//Special Snowflake
pub(super) type Games = Arc<Mutex<HashMap<String, GameState>>>;

#[derive(Clone)]
pub struct Data{
    pub pokedex: Pokedex,
    pub movedex: Movedex,
    pub typedex: Typedex,
    pub games: Games,
}

pub(super) fn build_pokemon(pokemon_json: serde_json::Value, typedex: Typedex) -> Option<BasePokemon> {
    let pokemon = BasePokemon { 
        index: pokemon_json["index"].as_i64().map(|x| x as u8)?,
        pokedex: pokemon_json["pokedex"].as_i64().map(|x| x as u8)?,
        name: pokemon_json["name"].as_str()?.to_string(), 
        base_hp: pokemon_json["base_stats"]["hp"].as_i64().map(|x| x as i32)?, 
        base_attack: pokemon_json["base_stats"]["attack"].as_i64().map(|x| x as i32)?, 
        base_defense: pokemon_json["base_stats"]["defense"].as_i64().map(|x| x as i32)?, 
        base_speed: pokemon_json["base_stats"]["speed"].as_i64().map(|x| x as i32)?, 
        base_special: pokemon_json["base_stats"]["special"].as_i64().map(|x| x as i32)?, 

        type1: typedex.get(&pokemon_json["types_id"][0].as_i64().map(|x| x as u8)?)?.clone(), 
        type2: if pokemon_json["types_id"][1] != pokemon_json["types_id"][0] {Some(typedex.get(&pokemon_json["types_id"][1].as_i64().map(|x| x as u8)?)?.clone())} else {None},
    };
    return Some(pokemon);
}

pub(super) fn build_type(poketype_json: serde_json::Value) -> Result<PokeType, Box<dyn Error>>{
    return Ok(PokeType {
        index: poketype_json["id"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        name: poketype_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into())).to_owned()?,
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

pub(super) fn build_moves(moves_json: serde_json::Value, typedex: Typedex) -> Result<Move, Box<dyn Error>> {
    return Ok(Move {
        id: moves_json["id"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        name: moves_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        effect: moves_json["effect"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        power: moves_json["power"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        move_type: typedex.get(&moves_json["type_id"].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError)?.clone(), 
        accuracy: moves_json["accuracy"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        pp: moves_json["pp"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
    });
}
