use std::fs::File;
use engine::structs::{Pokemon, PokeType, MoveType};
use rand::Rng;

mod engine;

fn build_pokemon<'a>(pokemon_json: &'a serde_json::Value, nickname: &'a str, level: i32, types: &'a Vec<PokeType<'a>>, ivs: i32) -> Option<Pokemon<'a>> { 
    let pokemon = Pokemon { 
        index: pokemon_json["index"].as_i64().map(|x| x as i32)?,
        pokedex: pokemon_json["pokedex"].as_i64().map(|x| x as i32)?,
        name: pokemon_json["name"].as_str()?, 
        nickname: &nickname,
        level,
        hp: pokemon_json["base_stats"]["hp"].as_i64().map(|x| x as i32)?, 
        attack: pokemon_json["base_stats"]["attack"].as_i64().map(|x| x as i32)?, 
        defense: pokemon_json["base_stats"]["defense"].as_i64().map(|x| x as i32)?, 
        speed: pokemon_json["base_stats"]["speed"].as_i64().map(|x| x as i32)?, 
        special: pokemon_json["base_stats"]["special"].as_i64().map(|x| x as i32)?, 

        hp_ev: 0,
        attack_ev: 0,
        defense_ev: 0,
        speed_ev: 0,
        special_ev: 0,

        type1: types.into_iter().find(|type1| type1.index == pokemon_json["types_id"][0].as_i64().map(|x| x as i32).unwrap())?, 
        type2: if pokemon_json["types_id"][1] != pokemon_json["types_id"][0] {types.into_iter().find(|type2| type2.index == pokemon_json["types_id"][1].as_i64().map(|x| x as i32).unwrap())} else {None},
        ivs,
        current_hp: 0, 
        status: engine::structs::PersistantStatus::Healthy,
    };
    return Some(pokemon);
}

fn build_type<'a>(poketype_json: &'a serde_json::Value) -> Option<PokeType<'a>>{
    let poketype = PokeType {
        index: poketype_json["id"].as_i64().map(|x| x as i32)?,
        name: poketype_json["name"].as_str()?,
        category: if poketype_json["category"].as_str()?.eq("physical") {MoveType::Physical} else if poketype_json["category"].as_str()?.eq("special") {MoveType::Special} else {return None},
        strong: poketype_json["strong"].as_array()?.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
        weak: poketype_json["weak"].as_array()?.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
        no_effect: poketype_json["no_effect"].as_array()?.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
    };
    return Some(poketype);
}

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
        if let Some(new_type) = build_type(poketypes) {
        
            typedex.push(new_type);
        }
    }

    for pokemon_json in pokedex_json.as_array().unwrap() {
        if let Some(new_mon) = build_pokemon(pokemon_json,pokemon_json["name"].as_str().unwrap(),level,&typedex,rng.gen_range(0..u16::MAX) as i32) {
        
            pokedex.push(new_mon);
        }
    }
    //println!("Types: {:#?}", &typedex_vec.into_iter().find(|x| x.index == 23));
    assert_eq!(pokedex.len(), 151, "Pokedex length should be {} but {} was found", 151, pokedex.len());

}
