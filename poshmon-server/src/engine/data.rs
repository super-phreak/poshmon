use std::{collections::HashMap, sync::{Arc, RwLock}, error::Error};

use poshmon_lib::engine::gen1::{BasePokemon, PokeType, MoveType, GameState, PokeMove, graphics::Sprite, EvolutionInfo};

use super::structs::DataFieldNotFoundError;

pub(super) type Pokedex = Arc<HashMap<u8,Arc<BasePokemon>>>;
pub(super) type Movedex = Arc<HashMap<u8,Arc<PokeMove>>>;
pub(super) type Typedex = Arc<HashMap<u8,Arc<PokeType>>>;
pub(super) type WordList = Arc<RwLock<Vec<String>>>;
//Special Snowflake
pub(super) type Games = Arc<RwLock<HashMap<String, Arc<RwLock<GameState>>>>>;

#[derive(Clone)]
pub struct Data{
    pub pokedex: Pokedex,
    pub movedex: Movedex,
    pub typedex: Typedex,
    pub games: Games,
    pub wordlist: WordList,
    pub debug: Arc<RwLock<HashMap<String, String>>>
}

pub(super) fn build_pokemon(pokemon_json: serde_json::Value, typedex: Typedex, movedex: Movedex) -> Result<BasePokemon, Box<dyn Error>> {
    let pokemon = BasePokemon { 
        index: pokemon_json["index"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("index")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("index").into()))?,
        pokedex: pokemon_json["pokedex"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("pokedex")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex").into()))?,
        name: pokemon_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError ::new("name")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("name").into()))?,
        base_hp: pokemon_json["base_stats"]["hp"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("hp")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("hp").into()))?, 
        base_attack: pokemon_json["base_stats"]["attack"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("attack")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("attack").into()))?, 
        base_defense: pokemon_json["base_stats"]["defense"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("defense")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("defense").into()))?,
        base_speed: pokemon_json["base_stats"]["speed"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("speed")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("speed").into()))?,
        base_special: pokemon_json["base_stats"]["special"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("special")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("special").into()))?,

        type1: typedex.get(&pokemon_json["types_id"][0].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError::new("type1"))?.clone(), 
        type2: if pokemon_json["types_id"][1] != pokemon_json["types_id"][0] {Some(typedex.get(&pokemon_json["types_id"][1].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError::new("type2"))?.clone())} else {None},
        learned_moves: populate_learnable_movedex(movedex.clone(), &pokemon_json["learnable_moves"])?,
        taught_moves: populate_movedex(movedex.clone(), pokemon_json["teachable_moves"].as_array().ok_or_else(|| DataFieldNotFoundError::new("teachable_moves"))?.iter().map(|x| x.as_i64().unwrap() as u8).collect())?,
        default_moves: populate_movedex(movedex.clone(), pokemon_json["attacks_lvl_1"].as_array().ok_or_else(|| DataFieldNotFoundError::new("attacks_lvl_1"))?.iter().map(|x| x.as_i64().unwrap() as u8).filter(|x| x > &0).collect())?,
        catch_rate: pokemon_json["catch_rate"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("catch_rate")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("catch_rate").into()))?,
        front_sprite: create_sprite(pokemon_json["front_sprite"].clone())?,
        back_sprite: create_sprite(pokemon_json["back_sprite"].clone())?,
        pokedex_entry: pokemon_json["pokedex_entry"]["text"].as_str().ok_or_else(|| DataFieldNotFoundError::new("pokedex_text")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex_text").into()))?,
        species: pokemon_json["pokedex_entry"]["species"].as_str().ok_or_else(|| DataFieldNotFoundError::new("pokedex_species")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex_species").into()))?,
        height: get_height(pokemon_json["pokedex_entry"]["height"].clone())?,
        weight: pokemon_json["pokedex_entry"]["weight"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("pokedex_weight")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex_weight").into()))?,
        evolution_info: Arc::new(get_evolution_info(pokemon_json["evo_info"].clone())?),
    };
    return Ok(pokemon);
}

fn populate_movedex(movedex: Movedex, moves: Vec<u8>) -> Result<Arc<Vec<Arc<PokeMove>>>, Box<dyn Error>> {
    let mut teachable_movedex: Vec<Arc<PokeMove>> = Vec::new();
    for mv in moves {
        teachable_movedex.push(movedex.get(&mv).ok_or_else(|| DataFieldNotFoundError::new("teachable_moves"))?.clone())
    }

    return Ok(Arc::new(teachable_movedex));
}

fn create_sprite(sprite_json: serde_json::Value) -> Result<Sprite, DataFieldNotFoundError> {
    let width: i32 = sprite_json["width"].as_i64().ok_or_else( move || DataFieldNotFoundError::new("width")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("width")))?;
    let height: i32 = sprite_json["height"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("height")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("height")))?;
    let data: String = sprite_json["data"].as_str().ok_or_else(|| DataFieldNotFoundError::new("data"))?.to_string();
    Ok(Sprite::new(width, height, data, "".to_string()))
}

fn get_height(pokedex_json: serde_json::Value) -> Result<u16, DataFieldNotFoundError> {
    let feet: u16 = pokedex_json["feet"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("pokedex_height_feet")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex_height_feet").into()))?;
    let inches: u16 = pokedex_json["inches"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("pokedex_height_inches")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("pokedex_height_inches").into()))?;
    Ok(feet * 12u16 + inches)
}

fn get_evolution_info(evolution_json: serde_json::Value) -> Result<Vec<EvolutionInfo>, DataFieldNotFoundError> {
    let mut info_vec: Vec<EvolutionInfo> = Vec::new();

    for evo_info in evolution_json.as_array().ok_or_else(|| DataFieldNotFoundError ::new("evo_info"))? {
        let method = match evo_info["evo_method"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_method"))? {
            1 => Ok(EvolutionInfo::LevelUp { 
                level: evo_info["evo_level"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_levelup_level")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("evo_levelup_level")))?, 
                index: evo_info["evo_mon_index"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_levelup_index")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("evo_levelup_index")))?,
            }),
            2 => Ok(EvolutionInfo::Item { 
                item_id: evo_info["evo_item_id"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_itemup_item")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("evo_itemup_item")))?, 
                index: evo_info["evo_mon_index"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_itemup_index")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("evo_itemup_index")))?, 
            }),
            3 => Ok(EvolutionInfo::Trade { 
                index: evo_info["evo_mon_index"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("evo_trade_index")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError ::new("evo_index")))?,
            }),
            _ => Err(DataFieldNotFoundError ::new("evo_method_unknown"))
        };
        
        match method {
            Ok(info) => info_vec.push(info),
            Err(e) => return Err(e),
        }
    }

    Ok(info_vec)
}

fn populate_learnable_movedex(movedex: Movedex, moves: & serde_json::Value) -> Result<Movedex, Box<dyn Error>> {
    let mut learnable_moves: HashMap<u8, Arc<PokeMove>> = HashMap::new();

    for mv in moves.as_array().unwrap() {
        learnable_moves.insert(mv["level"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_level")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_level").into()))?, movedex.get(&mv["move"].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError::new("move_id"))?.clone());
    }

    return Ok(Movedex::new(learnable_moves));
}

pub(super) fn build_type(poketype_json: serde_json::Value) -> Result<PokeType, Box<dyn Error>>{
    return Ok(PokeType {
        index: poketype_json["id"].as_i64().ok_or_else(|| DataFieldNotFoundError ::new("type_id")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("type_id").into()))?,
        name: poketype_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError ::new("type_name")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("type_name").into()))?,
        category: match poketype_json["category"].as_str().ok_or_else(|| DataFieldNotFoundError::new("type_category"))? {
            "physical" => MoveType::Physical,
            "special" => MoveType::Special,
            _ => return Err(DataFieldNotFoundError::new("type_category_unknown").into()),
        },
        strong: poketype_json["strong"].as_array().ok_or_else(|| DataFieldNotFoundError::new("type_strong_id"))?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
        weak: poketype_json["weak"].as_array().ok_or_else(|| DataFieldNotFoundError::new("type_weak_id"))?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
        no_effect: poketype_json["no_effect"].as_array().ok_or_else(|| DataFieldNotFoundError::new("type_no_effect_id"))?.iter().map(|x| x.as_i64().unwrap() as u8).collect(),
    });
}

pub(super) fn build_moves(moves_json: serde_json::Value, typedex: Typedex) -> Result<PokeMove, Box<dyn Error>> {
    return Ok(PokeMove {
        id: moves_json["id"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_id")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_id").into()))?,
        name: moves_json["name"].as_str().ok_or_else(|| DataFieldNotFoundError::new("move_name")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_name").into()))?,
        effect: moves_json["effect"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_effect")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_effect").into()))?,
        power: moves_json["power"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_power")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_power").into()))?,
        move_type: typedex.get(&moves_json["type_id"].as_i64().map(|x| x as u8).unwrap()).ok_or_else(|| DataFieldNotFoundError::new("move_type"))?.clone(), 
        accuracy: moves_json["accuracy"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_accuracy")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_accuracy").into()))?,
        pp: moves_json["pp"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_pp")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_pp").into()))?,
        priority: moves_json["priority"].as_i64().ok_or_else(|| DataFieldNotFoundError::new("move_priority")).and_then(|x| x.try_into().map_err(|_| DataFieldNotFoundError::new("move_priority").into()))?,
    });
}