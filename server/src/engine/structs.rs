use std::{fmt, error::Error, cell::Cell, sync::Arc};

use rand::Rng;
use uuid::Uuid;

use super::data::Movedex;

#[derive(Debug, Clone)]
pub struct Move {
    pub id: u8,
    pub name: String,
    pub effect: i32,
    pub power: i32,
    pub move_type: Arc<PokeType>,
    pub accuracy: i32,
    pub pp: i32
}

#[derive(Debug, Clone)]
pub struct BasePokemon {
    pub index: u8,
    pub pokedex: u8,
    pub name: String,
    //pub sprite: Sprite,

    pub base_hp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_speed: i32,
    pub base_special: i32,

    pub type1: Arc<PokeType>,
    pub type2: Option<Arc<PokeType>>,

    pub learned_moves: Movedex,
    pub default_moves: Arc<Vec<Arc<Move>>>,
    pub taught_moves: Arc<Vec<Arc<Move>>>,
}

#[derive(Debug)]
pub struct Pokemon {
    pub base: Arc<BasePokemon>,

    pub trainer_id: u16,
    pub guid: Uuid,

    pub nickname: String,

    pub level: i32,
    pub xp: u32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
    pub iv: u16,

    pub hp_ev: i32,
    pub attack_ev: i32,
    pub defense_ev: i32,
    pub speed_ev: i32,
    pub special_ev: i32,

    pub move1: Option<Arc<Move>>,
    pub move2: Option<Arc<Move>>,
    pub move3: Option<Arc<Move>>,
    pub move4: Option<Arc<Move>>,

    pub status: Status,
    pub current_hp: i32,

}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub struct PokeType {
    pub index: u8,
    pub name: String,
    pub category: MoveType,

    pub strong: Vec<u8>,
    pub weak: Vec<u8>,
    pub no_effect: Vec<u8>,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum MoveType {
    Physical,
    Special
}

#[derive(Debug,Clone,Copy)]
pub enum MoveStatus {
    Error,
    Hit,
    SuperEffective,
    NotVeryEffective,
    CriticalHit,
    SuperEffectiveCriticalHit,
    NotVeryEffectiveCriticalHit,
    Missed,
    NoEffect,
    Fainted,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Status {
    Healthy,
    Fainted,
    //TO DO IMPLEMENT THESE
    // Paralyzed,
    // Poisoned,
    // Burned,
    // Sleep,
    // Freeze,
    // Confused,
    // Seeded,
    // Bound,
}
 
pub enum StatEnum {
    Hp,
    Attack,
    Defense,
    Speed,
    Special,
}

#[derive(Debug)]
pub struct GameState {
    pub player1_team: Vec<Pokemon>,
    pub player2_team: Vec<Pokemon>,

    pub active1: usize,
    pub active2: usize,

    pub last_fight: Cell<FightResult>,

    pub player1_ready: Cell<bool>,
    pub player2_ready: Cell<bool>,
}

impl GameState {
    pub fn fight(&mut self, player1_move: &Move, player2_move: &Move) {
        let mon1: &mut Pokemon = &mut self.player1_team[self.active1];
        let mon2: &mut Pokemon = &mut self.player2_team[self.active2];

        let player1_movestatus;
        let player2_movestatus;

        let mut rng = rand::thread_rng();

        if mon1.speed > mon2.speed {
            let result1 = Self::attack(mon1, mon2, player1_move);
            if result1.0 >= mon2.current_hp {
                mon2.current_hp = 0;
                mon2.status = Status::Fainted;
                player2_movestatus = MoveStatus::Fainted;
            } else {
                let result2 = Self::attack(mon2, mon1, player2_move);
                player2_movestatus = result2.1;
            }
            player1_movestatus = result1.1;

        } else if mon1.speed < mon2.speed {
            player1_movestatus = MoveStatus::Missed;
            player2_movestatus = MoveStatus::Missed;
        } else  if rng.gen_bool(0.5) {
            player1_movestatus = MoveStatus::Missed;
            player2_movestatus = MoveStatus::Missed;
        } else {
            player1_movestatus = MoveStatus::Missed;
            player2_movestatus = MoveStatus::Missed;
        }
        self.last_fight.replace(FightResult{ player1_movestatus, player2_movestatus });
    }

    fn dmg_calculator(level: i32, power: i32, attack: i32, defense: i32, stab: i32, effective: i32, random: i32) -> i32 {
        return (((((2*level/5)+2) * power * attack / defense) / 50) + 2) * stab * effective * random / 2550000;
    }
    
    fn get_effective(attack_type: &PokeType, target: &Pokemon) -> i32 {
        let mut effective: i32 = 100;
        if attack_type.strong.contains(&target.base.type1.index) {
            effective = effective << 1;
        }
        if attack_type.weak.contains(&target.base.type1.index) {
            effective = effective >> 1;
        }
        if attack_type.no_effect.contains(&target.base.type1.index) {
            effective = 0;
        }
        if let Some(type2) = &target.base.type2 {
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
        let rng = rng.gen_range(0..=255);
        println!("{} <= {} ({} / 2)", rng, attacker.speed / 2, attacker.speed);
        return rng <= attacker.speed / 2;
    }
    
    fn attack(attacker: &Pokemon, defender: &Pokemon, pokemove: &Move) -> (i32,MoveStatus) {
        let mut rng = rand::thread_rng();
        let mut stab = if &pokemove.move_type == &attacker.base.type1 {150} else {100};
        if let Some(type2) = &attacker.base.type2 {
            stab = if &pokemove.move_type == type2 {150} else {stab};
        }
        let hit_roll = rng.gen_range(0..=255);
        let effective: i32 = Self::get_effective(&pokemove.move_type, &defender);
        let random = rng.gen_range(217..=255);
        let crit = Self::is_crit(&attacker);
        let level = if crit {attacker.level * 2} else {attacker.level};
        println!("level: {}, stab: {}, effective: {}, random: {}", level, stab, effective, random);
        let status = match (effective, crit, hit_roll) {
            (0,_, _) => MoveStatus::NoEffect,
            (_,_,x) if x > pokemove.accuracy => MoveStatus::Missed,
            (x,true, _) if x < 1 => MoveStatus::NotVeryEffectiveCriticalHit,
            (x,false, _) if x < 1 => MoveStatus::NotVeryEffective,
            (x,true, _) if x > 1 => MoveStatus::SuperEffectiveCriticalHit,
            (x,false, _) if x > 1 => MoveStatus::SuperEffective,
            (1,true, _) => MoveStatus::CriticalHit,
            (1,false, _) => MoveStatus::Hit,
            (_,_,_) => MoveStatus::Error,
        };
        return (Self::dmg_calculator(level, pokemove.power, attacker.attack, defender.defense, stab, effective, random),status)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FightResult {
    pub player1_movestatus: MoveStatus,
    pub player2_movestatus: MoveStatus,
}

//Errors
#[derive(Debug, Clone)]
pub struct PokemonNotFoundError;

impl fmt::Display for PokemonNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon not found")
    }
}

impl Error for PokemonNotFoundError {}

#[derive(Debug, Clone)]
pub struct DataFieldNotFoundError;

impl fmt::Display for DataFieldNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data field could not be found")
    }
}

impl Error for DataFieldNotFoundError {}

#[derive(Debug, Clone)]
pub struct DataLockError;

impl fmt::Display for DataLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data could not be locked")
    }
}

impl Error for DataLockError {}