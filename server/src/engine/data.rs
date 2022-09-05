use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}, error::Error};

use super::structs::{BasePokemon, PokeType, MoveType, DataFieldNotFoundError, GameState, Move};

pub(super) type Pokedex = Arc<HashMap<u8,Arc<BasePokemon>>>;
pub(super) type Movedex = Arc<HashMap<u8,Arc<Move>>>;
pub(super) type Typedex = Arc<HashMap<u8,Arc<PokeType>>>;
pub(super) type WordList = Arc<RwLock<Vec<String>>>;
//Special Snowflake
pub(super) type Games = Arc<Mutex<HashMap<String, Arc<RwLock<GameState>>>>>;

#[derive(Clone)]
pub struct Data{
    pub pokedex: Pokedex,
    pub movedex: Movedex,
    pub typedex: Typedex,
    pub games: Games,
    pub wordlist: WordList,
}

pub(super) fn build_pokemon(pokemon_json: serde_json::Value, typedex: Typedex, movedex: Movedex) -> Result<BasePokemon, Box<dyn Error>> {
    let pokemon = BasePokemon { 
        index: pokemon_json["index"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        pokedex: pokemon_json["pokedex"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        name: pokemon_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        base_hp: pokemon_json["base_stats"]["hp"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?, 
        base_attack: pokemon_json["base_stats"]["attack"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?, 
        base_defense: pokemon_json["base_stats"]["defense"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        base_speed: pokemon_json["base_stats"]["speed"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,
        base_special: pokemon_json["base_stats"]["special"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?,

        type1: typedex.get(&pokemon_json["types_id"][0].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError)?.clone(), 
        type2: if pokemon_json["types_id"][1] != pokemon_json["types_id"][0] {Some(typedex.get(&pokemon_json["types_id"][1].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError)?.clone())} else {None},
        learned_moves: populate_learnable_movedex(movedex.clone(), &pokemon_json["learnable_moves"])?,
        taught_moves: populate_movedex(movedex.clone(), pokemon_json["teachable_moves"].as_array().ok_or_else(|| DataFieldNotFoundError)?.iter().map(|x| x.as_i64().unwrap() as u8).collect())?,
        default_moves: populate_movedex(movedex.clone(), pokemon_json["attacks_lvl_1"].as_array().ok_or_else(|| DataFieldNotFoundError)?.iter().map(|x| x.as_i64().unwrap() as u8).filter(|x| x > &0).collect())?,
    };
    return Ok(pokemon);
}

fn populate_movedex(movedex: Movedex, moves: Vec<u8>) -> Result<Arc<Vec<Arc<Move>>>, Box<dyn Error>> {
    let mut teachable_movedex: Vec<Arc<Move>> = Vec::new();
    for mv in moves {
        teachable_movedex.push(movedex.get(&mv).ok_or_else(|| DataFieldNotFoundError)?.clone())
    }

    return Ok(Arc::new(teachable_movedex));
}

fn populate_learnable_movedex(movedex: Movedex, moves: & serde_json::Value) -> Result<Movedex, Box<dyn Error>> {
    let mut learnable_moves: HashMap<u8, Arc<Move>> = HashMap::new();

    for mv in moves.as_array().unwrap() {
        learnable_moves.insert(mv["level"].as_i64().ok_or_else(|| DataFieldNotFoundError).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError.into()))?, movedex.get(&mv["move"].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError)?.clone());
    }

    return Ok(Movedex::new(learnable_moves));
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
