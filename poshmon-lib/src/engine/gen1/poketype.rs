#[derive(Debug, PartialEq, Clone)]
pub struct PokeType {
    pub index: u8,
    pub name: String,
    pub category: MoveType,

    pub strong: Vec<u8>,
    pub weak: Vec<u8>,
    pub no_effect: Vec<u8>,

}

#[derive(Debug, PartialEq, Clone)]
pub enum MoveType {
    Physical,
    Special
}