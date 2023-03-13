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
    Special,
    Error,
}

impl Default for PokeType {
    fn default() -> Self {
        PokeType { index: 255, name: "NONE".to_string(), category: MoveType::Error, strong: Vec::new(), weak: Vec::new(), no_effect: Vec::new() }
    }
}
