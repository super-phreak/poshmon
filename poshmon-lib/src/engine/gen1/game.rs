use std::fmt::Display;

use rand::Rng;

use super::{trainer::Trainer, PokeMove, pokemon::PermStatus};

pub struct GameState {
    trainer1: Trainer,
    trainer2: Trainer,

    trainer1_active: usize,
    trainer2_active: usize,
}

impl GameState {
    pub fn fight(&mut self, player1_move: &PokeMove, player2_move: &PokeMove) -> Result<Vec<BattleMessage>, GameStateErrors> {
        let mut rng = rand::thread_rng();
        
        let mut team1 = self.trainer1.get_team_as_mut();
        let mut team2 = self.trainer2.get_team_as_mut();

        let p1_pkmn = team1.get_mut(self.trainer1_active).ok_or(GameStateErrors::ActivePokemonNotFound(1, self.trainer1_active))?;
        let p2_pkmn = team2.get_mut(self.trainer2_active).ok_or(GameStateErrors::ActivePokemonNotFound(2, self.trainer2_active))?;

        let mut battle_msg: Vec<BattleMessage> = Vec::new();

        match Self::get_priority(player1_move, player2_move, p1_pkmn.speed, p2_pkmn.speed, rng.gen_bool(0.5)) {
            Priority::Player1First => {
                let (mut p1msg, p1dmg) = p1_pkmn.attack(&p2_pkmn, player1_move);
                battle_msg.append(&mut p1msg);
                if p2_pkmn.set_hp(super::pokemon::Health::Subtract(p1dmg)) != PermStatus::Fainted {
                    let (mut p2msg, p2dmg) = p2_pkmn.attack(&p1_pkmn, player2_move);
                    battle_msg.append(&mut p2msg);
                    p1_pkmn.set_hp(super::pokemon::Health::Percent(p2dmg));
                };
            },
            Priority::Player2First => {
                let (mut p2msg, p2dmg) = p2_pkmn.attack(&p1_pkmn, player2_move);
                battle_msg.append(&mut p2msg);
                if p1_pkmn.set_hp(super::pokemon::Health::Subtract(p2dmg)) != PermStatus::Fainted {
                    let (mut p1msg, p1dmg) = p1_pkmn.attack(&p2_pkmn, player1_move);
                    battle_msg.append(&mut p1msg);
                    p2_pkmn.set_hp(super::pokemon::Health::Percent(p1dmg));
                };
            },
        };

        Ok(battle_msg)
    }

    fn get_priority(player1_move: &PokeMove, player2_move: &PokeMove, p1_speed: i32, p2_speed: i32, tie_break: bool) -> Priority {
        if player1_move.priority > player2_move.priority {
            Priority::Player1First
        } else if player1_move.priority < player2_move.priority {
            Priority::Player2First
        } else {
            if p1_speed > p2_speed {
                Priority::Player1First
            } else if p1_speed < p2_speed {
                Priority::Player2First
            } else {
                match tie_break {
                    true => Priority::Player1First,
                    false => Priority::Player2First,
                }
            }
        }
    }
}

pub enum BattleMessage {
    Missed,
    NotVeryEffective,
    SuperEffective,
    NoEffect,
    CriticalHit,
}

enum Priority {
    Player1First,
    Player2First,
}

#[derive(Debug)]
pub enum GameStateErrors {
    DefaultError,
    ActivePokemonNotFound(usize, usize),
}

impl Display for GameStateErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStateErrors::DefaultError => write!(f, "{}", "Generic Game State Error"),
            GameStateErrors::ActivePokemonNotFound(team, index) => write!(f, "Active Pokemon at team ({}) index {} not found", team, index),
        }
    }
}

impl std::error::Error for GameStateErrors { }