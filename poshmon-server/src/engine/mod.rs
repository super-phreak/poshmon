use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::{Arc, RwLock};
use poshmon_lib::engine::gen1::{BasePokemon, PokeMove, PokeType};

use self::data::{Data, Pokedex, Typedex, build_type, build_pokemon, Games, Movedex, build_moves, WordList};

pub mod structs;
pub mod data;

pub fn init_engine(data: HashMap<&str, serde_json::Value>, words_file: File) -> Data {
    let mut pokedex: HashMap<u8, Arc<BasePokemon>> = HashMap::new();
    let mut movedex: HashMap<u8, Arc<PokeMove>> = HashMap::new();
    let mut typedex: HashMap<u8, Arc<PokeType>> = HashMap::new();
    let games = Games::new(RwLock::new(HashMap::new()));
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
                Ok(new_mon) => _ = pokedex.insert(new_mon.index, Arc::new(new_mon)),
                Err(e) => println!("{} was the error", e),
            }
        }
    }

    let pokedex = Pokedex::new(pokedex);

    let debug: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));


    return Data { pokedex, movedex, typedex, games, wordlist, debug };
}







