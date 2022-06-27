use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use rand::Rng;
use uuid::Uuid;
use self::structs:: {
    MoveType,
    Status,
    PokeType,
    BasePokemon,
    Move, Pokemon, StatEnum, PokemonNotFoundError, DataLockError,
};

use self::data::{Data, Pokedex, Typedex, build_type, build_pokemon};

pub mod structs;
pub mod data;

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

fn get_effective(attack_type: &PokeType, target: &BasePokemon) -> i32 {
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
    if let Some(type2) = &target.type2 {
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

fn is_crit(attacker: &BasePokemon) -> bool {
    let mut rng = rand::thread_rng();
    let rng = rng.gen_range(0..255);
    println!("{} <= {} ({} / 2)", rng, attacker.base_speed / 2, attacker.base_speed);
    return rng <= attacker.base_speed / 2;
}

pub fn attack(attacker: &BasePokemon, defender: &BasePokemon, pokemove: &Move) -> (i32,i32,bool) {
    let mut rng = rand::thread_rng();
    let mut stab = if pokemove.move_type == attacker.type1 {150} else {100};
    if let Some(type2) = &attacker.type2 {
        stab = if pokemove.move_type == *type2 {150} else {stab};
    }
    let effective: i32 = get_effective(&pokemove.move_type, &defender);
    let random = rng.gen_range(217..255);
    let crit = is_crit(&attacker);
    //let level = if crit {attacker.level * 2} else {attacker.level};
    let level = 100;
    println!("level: {}, stab: {}, effective: {}, random: {}", level, stab, effective, random);
    return (dmg_calculator(level, pokemove.power, attacker.base_attack, defender.base_defense, stab, effective, random),effective,crit)
}

fn get_iv (method: StatEnum, iv: u16) -> i32 {
    match method {
        StatEnum::Attack => return ((iv & 0xF000) >> 12)  as i32,
        StatEnum::Defense => return ((iv & 0x0F00) >> 8) as i32,
        StatEnum::Speed => return ((iv & 0x00F0) >> 4) as i32,
        StatEnum::Special => return (iv & 0x000F) as i32,
        StatEnum::Hp => return (((iv & 0x1000) >> 9) + ((iv & 0x0100) >> 6) + ((iv & 0x0010) >> 3) + ((iv & 0x0001))) as i32,
    }
}

pub fn create_pokemon(id: u8, data: Data) -> Result<Pokemon, Box<dyn Error>> {
    let pokedex = match data.pokedex.try_lock() {
        Ok(p) => p,
        Err(_) => return Err(DataLockError.into()),

    };
    let ivs: u16 = 0xffff;
    let level = 100;
    pokedex.get(&id).ok_or_else(|| PokemonNotFoundError.into()).and_then(|base_pokemon| {
        Ok(Pokemon {
            base: base_pokemon.clone(),
            trainer_id: 1337,
            nickname: "TestMon".to_string(),
            lvl: 100,
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
            move1: None,
            move2: None,
            move3: None,
            move4: None,
            status: Status::Healthy,
            current_hp: hp_calculator(base_pokemon.base_hp, get_iv(StatEnum::Hp, ivs), ivs as i32, level),
            guid: Uuid::new_v4(),
        })
    })
}

pub fn init_engine(data: HashMap<&str, serde_json::Value>) -> Data {
    let pokedex = Pokedex::new(Mutex::new(HashMap::new()));
    let typedex = Typedex::new(Mutex::new(HashMap::new()));

    if let Some(config) = data.get("conf") {
        for poketypes in config["types"].as_array().unwrap() {
            match build_type(poketypes) {
                Ok(new_type) => _ = typedex.lock().unwrap().insert(new_type.index, new_type),
                Err(e) => println!("{} was the error", e),
            };
        }
    }

    if let Some(pokemon) = data.get("pokemon") {
        for pokemon_json in pokemon.as_array().unwrap() {
            if let Some(new_mon) = build_pokemon(pokemon_json, typedex.clone()) {
                pokedex.lock().unwrap().insert(new_mon.pokedex, new_mon);
            }
        }
    }

    return Data { pokedex, typedex };
}




