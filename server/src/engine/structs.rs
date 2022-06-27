use std::{fmt, error::Error};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Move {
    pub id: u8,
    pub name: String,
    pub effect: i32,
    pub power: i32,
    pub move_type: PokeType,
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

    pub type1: PokeType,
    pub type2: Option<PokeType>,
}

#[derive(Debug)]
pub struct Pokemon {
    pub base: BasePokemon,

    pub trainer_id: u16,
    pub guid: Uuid,

    pub nickname: String,

    pub lvl: i32,
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

    pub move1: Option<Move>,
    pub move2: Option<Move>,
    pub move3: Option<Move>,
    pub move4: Option<Move>,

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
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Status {
    Healthy,
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