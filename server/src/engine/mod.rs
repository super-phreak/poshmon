use std::cmp;
use rand::Rng;
use self::structs::MoveType;
use self::structs::PersistantStatus;
use self::structs::PokeType;
use self::structs::Pokemon;
use self::structs::Move;

pub mod structs;

fn dmg_calculator(level: i32, power: i32, attack: i32, defense: i32, stab: i32, effective: i32, random: i32) -> i32 {
    return (((((2*level/5)+2) * power * attack / defense) / 50) + 2) * stab * effective * random / 2550000;
}

pub fn stat_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
    let statxp: i32 = (statxp as f32).sqrt().ceil() as i32;
    let statxp: i32 = cmp::min(statxp, 255);
    return (((((base+iv) * 2) + (statxp/4))*level)/100)+5;
}

pub fn hp_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
    return stat_calculator(base, iv, statxp, level) + level + 5;
}

fn get_effective(attack_type: &PokeType, target: &Pokemon) -> i32 {
    let mut effective: i32 = 100;
    if attack_type.strong.contains(&target.type1.index) {
        effective = effective << 1;
    }
    if attack_type.weak.contains(&target.type1.index) {
        effective = effective >> 1;
    }
    if attack_type.no_effect.contains(&target.type1.index) {
        effective = 0;
    }
    if let Some(type2) = target.type2 {
        if attack_type.strong.contains(&type2.index) {
            effective = effective << 1;
        }
        if attack_type.weak.contains(&type2.index) {
            effective = effective >> 1;
        }
        if attack_type.no_effect.contains(&type2.index) {
            effective = 0;
        }
    }
    return effective;
}

fn is_crit(attacker: &Pokemon) -> bool {
    let mut rng = rand::thread_rng();
    let rng = rng.gen_range(0..255);
    println!("{} <= {} ({} / 2)", rng, attacker.speed / 2, attacker.speed);
    return rng <= attacker.speed / 2;
}

pub fn attack(attacker: &Pokemon, defender: &Pokemon, pokemove: &Move) -> (i32,i32,bool) {
    let mut rng = rand::thread_rng();
    let mut stab = if pokemove.move_type == attacker.type1 {150} else {100};
    if let Some(type2) = attacker.type2 {
        stab = if pokemove.move_type == type2 {150} else {stab};
    }
    let effective: i32 = get_effective(&pokemove.move_type, &defender);
    let random = rng.gen_range(217..255);
    let crit = is_crit(&attacker);
    let level = if crit {attacker.level * 2} else {attacker.level};
    println!("level: {}, stab: {}, effective: {}, random: {}", level, stab, effective, random);
    return (dmg_calculator(level, pokemove.power, attacker.attack, defender.defense, stab, effective, random),effective,crit)
}

pub fn build_pokemon<'a>(pokemon_json: &'a serde_json::Value, nickname: &'a str, level: i32, types: &'a Vec<PokeType<'a>>, ivs: i32) -> Option<Pokemon<'a>> { 
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
        status: PersistantStatus::Healthy,
    };
    return Some(pokemon);
}

pub fn build_type<'a>(poketype_json: &'a serde_json::Value) -> Option<PokeType<'a>>{
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
