use std::sync::Arc;

use super::PokeType;

#[derive(Debug)]
pub struct PokeMove {
    pub id: u8,
    pub name: String,
    pub effect: i32,
    pub power: i32,
    pub move_type: Arc<PokeType>,
    pub accuracy: i32,
    pub pp: i32,
    pub priority: i32,
}