use std::sync::{Arc, RwLock, RwLockWriteGuard};

use crate::engine::generics::SpriteData;

use super::Pokemon;

pub struct Trainer {
    pub name: String,
    pub id: u16,
    pokemon_team: Arc<RwLock<Vec<Pokemon>>>,
    pub sprite: Arc<SpriteData>,
}

impl Trainer {
    pub fn get_team_as_mut(&self) -> RwLockWriteGuard<Vec<Pokemon>> {
        match self.pokemon_team.write() {
            Ok(team) => team,
            Err(_) => todo!(),
        }
    }
}
