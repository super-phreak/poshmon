use std::cmp;
use rand::Rng;
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