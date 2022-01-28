use std::fs::File;
use engine::structs::{Pokemon, PokeType, MoveType};
use rand::Rng;
use tokio::{io as tokio_io, task};
use std::error::Error;
use std::io::{self, Write};

mod engine;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut pokedex: Vec<Pokemon> = Vec::new();
    let mut typedex: Vec<PokeType> = Vec::new();
    let mut rng = rand::thread_rng();
    let engine_conf = File::open("../data/engine.json").expect("Unable to read file");
    let pokedex_file = File::open("../data/pokedex.json").expect("unable to open pokedex");
    let engine_json: serde_json::Value = serde_json::from_reader(engine_conf).expect("JSON was not well-formatted");
    let pokedex_json: serde_json::Value = serde_json::from_reader(pokedex_file).expect("JSON was not well-formatted");
    let level: i32 = 100;

    for poketypes in engine_json["types"].as_array().unwrap() {
        if let Some(new_type) = engine::build_type(poketypes) {
            typedex.push(new_type);
        }
    }

    for pokemon_json in pokedex_json.as_array().unwrap() {
        if let Some(new_mon) = engine::build_pokemon(pokemon_json,pokemon_json["name"].as_str().unwrap(),level,&typedex,rng.gen_range(0..u16::MAX) as i32) {
            pokedex.push(new_mon);
        }
    }
    //println!("Types: {:#?}", &typedex_vec.into_iter().find(|x| x.index == 23));
    assert_eq!(pokedex.len(), 151, "Pokedex length should be {} but {} was found", 151, pokedex.len());

}
