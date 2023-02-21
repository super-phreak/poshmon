use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::{Mutex, Arc, RwLock};
// use rand::Rng;
use uuid::Uuid;
use self::structs:: {
    Status,
    PokeType,
    Move, 
    Pokemon, 
    StatEnum, 
    PokemonNotFoundError, 
    BasePokemon,
};

use self::data::{Data, Pokedex, Typedex, build_type, build_pokemon, Games, Movedex, build_moves, WordList};

pub mod structs;
pub mod data;



fn get_iv (method: StatEnum, iv: u16) -> i32 {
    match method {
        StatEnum::Attack => return ((iv & 0xF000) >> 12)  as i32,
        StatEnum::Defense => return ((iv & 0x0F00) >> 8) as i32,
        StatEnum::Speed => return ((iv & 0x00F0) >> 4) as i32,
        StatEnum::Special => return (iv & 0x000F) as i32,
        StatEnum::Hp => return (((iv & 0x1000) >> 9) + ((iv & 0x0100) >> 6) + ((iv & 0x0010) >> 3) + ((iv & 0x0001))) as i32,
    }
}

pub fn stat_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
    let statxp: i32 = (statxp as f32).sqrt().ceil() as i32;
    let statxp: i32 = cmp::min(statxp, 255);
    return (((((base+iv) * 2) + (statxp/4))*level)/100)+5;
}

pub fn hp_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
    return stat_calculator(base, iv, statxp, level) + level + 5;
}

pub fn create_pokemon(id: u8, data: Data) -> Result<Pokemon, Box<dyn Error>> {
    let ivs: u16 = 0xffff;
    let level = 100;
    data.pokedex.get(&id).ok_or_else(|| PokemonNotFoundError.into()).and_then(|base_pokemon| {
        Ok(Pokemon {
            base: base_pokemon.clone(),
            trainer_id: 1337,
            nickname: "TestMon".to_string(),
            level: 100,
            xp: 0,
            hp: hp_calculator(base_pokemon.base_hp, get_iv(StatEnum::Hp, ivs), ivs as i32, level),
            attack: stat_calculator(base_pokemon.base_attack, get_iv(StatEnum::Attack, ivs), ivs as i32, level),
            defense: stat_calculator(base_pokemon.base_defense, get_iv(StatEnum::Defense, ivs), ivs as i32, level),
            speed: stat_calculator(base_pokemon.base_speed, get_iv(StatEnum::Speed, ivs), ivs as i32, level),
            special: stat_calculator(base_pokemon.base_special, get_iv(StatEnum::Special, ivs), ivs as i32, level),
            iv: ivs,
            hp_ev: ivs as i32,
            attack_ev: ivs as i32,
            defense_ev: ivs as i32,
            speed_ev: ivs as i32,
            special_ev: ivs as i32,
            move1: base_pokemon.default_moves.get(0).map_or_else(|| None, |v| Some(v.clone())),
            move2: base_pokemon.default_moves.get(1).map_or_else(|| None, |v| Some(v.clone())),
            move3: base_pokemon.default_moves.get(2).map_or_else(|| None, |v| Some(v.clone())),
            move4: base_pokemon.default_moves.get(3).map_or_else(|| None, |v| Some(v.clone())),
            status: Status::Healthy,
            current_hp: hp_calculator(base_pokemon.base_hp, get_iv(StatEnum::Hp, ivs), ivs as i32, level),
            guid: Uuid::new_v4(),
        })
    })
}

pub fn init_engine(data: HashMap<&str, serde_json::Value>, words_file: File) -> Data {
    let mut pokedex: HashMap<u8, Arc<BasePokemon>> = HashMap::new();
    let mut movedex: HashMap<u8, Arc<Move>> = HashMap::new();
    let mut typedex: HashMap<u8, Arc<PokeType>> = HashMap::new();
    let games = Games::new(Mutex::new(HashMap::new()));
    let mut wordlist: Vec<String> = Vec::new();

    if let Some(config) = data.get("conf") {
        for poketypes in config["types"].as_array().unwrap().to_owned() {
            match build_type(poketypes) {
                Ok(new_type) => _ = typedex.insert(new_type.index, Arc::new(new_type)),
                Err(e) => println!("{} was the error", e),
            };
        }
    }

    let reader = BufReader::new(words_file);
    for word in reader.lines() {
        match word {
            Ok(word) => _ = wordlist.push(word),
            Err(e) => println!("{} was the error", e),
        }
    }

    let wordlist = WordList::new(RwLock::new(wordlist));

    let typedex = Typedex::new(typedex);

    if let Some(moves) = data.get("moves") {
        for moves_json in moves.as_array().unwrap().to_owned() {
            match build_moves(moves_json, typedex.clone()) {
                Ok(new_move) => _ = movedex.insert(new_move.id, Arc::new(new_move)),
                Err(e) => println!("{} was the error", e),
            };
        }
    }

    let movedex = Movedex::new(movedex);

    if let Some(pokemon) = data.get("pokemon") {
        for pokemon_json in pokemon.as_array().unwrap().to_owned() {
            match build_pokemon(pokemon_json, typedex.clone(), movedex.clone()) {
                Ok(new_mon) => _ = pokedex.insert(new_mon.pokedex, Arc::new(new_mon)),
                Err(e) => println!("{} was the error", e),
            }
        }
    }

    let pokedex = Pokedex::new(pokedex);


    return Data { pokedex, movedex, typedex, games, wordlist };
}




