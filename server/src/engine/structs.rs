#[derive(Debug)]
pub struct Move<'a> {
    pub id: i32,
    pub name: &'a str,
    pub effect: i32,
    pub power: i32,
    pub move_type: &'a PokeType<'a>,
    pub accuracy: i32,
    pub pp: i32
}

#[derive(Debug)]
pub struct Pokemon<'a> {
    pub index: i32,
    pub pokedex: i32,
    pub name: &'a str,
    pub nickname: &'a str,

    pub level: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,

    pub hp_ev: i32,
    pub attack_ev: i32,
    pub defense_ev: i32,
    pub speed_ev: i32,
    pub special_ev: i32,

    pub ivs: i32,
    
    pub type1: &'a PokeType<'a>,
    pub type2: Option<&'a PokeType<'a>>,

    //Persistant Battle Tracking Things
    pub current_hp: i32,
    pub status: PersistantStatus,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PokeType<'a> {
    pub index: i32,
    pub name: &'a str,
    pub category: MoveType,

    pub strong: Vec<i32>,
    pub weak: Vec<i32>,
    pub no_effect: Vec<i32>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MoveType {
    Physical,
    Special
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PersistantStatus {
    Healthy,
    Paralyzed,
    Poisoned,
    Burned,
    Sleep,
    Freeze,

}