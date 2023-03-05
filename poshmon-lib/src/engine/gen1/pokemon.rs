use std::{
    sync::Arc, 
    collections::HashMap, cmp, fmt::Display, 
};

use rand::Rng;
use uuid::Uuid;

use crate::engine::gen1::MoveType;

use super::{PokeType, pokemove::PokeMove, graphics::Sprite, game::BattleMessage};

pub type Movedex = Arc<HashMap<u8,Arc<PokeMove>>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PermStatus {
    Healthy,
    Fainted,
    Paralyzed,
    Poisoned,
    Burned,
    Sleep {turn: i32},
    Frozen,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VolatileStatus {
    None,
    Confused {turn: i32},
    BadlyPoisoned {turn: i32},
    Seeded,
    Bound {turn: i32},
    Flinch,
}

pub enum BattleStatus {
    Charging,
    GettingPumped,
    Mimic,
    Recharging,
    Subsitute,
    SemiInvulnerable,
    Transformed,

}
 
pub enum Stat {
    Hp,
    Attack,
    Defense,
    Speed,
    Special,
}

#[derive(Debug)]
pub enum EvolutionInfo {
    None,
    LevelUp { level: u8, index: u8 },
    Item { item_id: u8, index: u8},
    Trade { index: u8 },
}

#[derive(Debug, Clone)]
pub struct BasePokemon {
    pub index: u8,
    pub pokedex: u8,
    pub name: String,
    pub catch_rate: u8,
    pub front_sprite: Sprite,
    pub back_sprite: Sprite,

    pub base_hp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_speed: i32,
    pub base_special: i32,

    pub type1: Arc<PokeType>,
    pub type2: Option<Arc<PokeType>>,

    pub learned_moves: Movedex,
    pub default_moves: Arc<Vec<Arc<PokeMove>>>,
    pub taught_moves: Arc<Vec<Arc<PokeMove>>>,

    pub pokedex_entry: String,
    pub species: String,
    pub height: u16,
    pub weight: u16,

    pub evolution_info: Arc<Vec<EvolutionInfo>>,
}

#[derive(Debug)]
pub struct Pokemon {
    basemon: Arc<BasePokemon>,

    pub guid: Uuid,

    pub name: String,

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

    pub move1: Option<Arc<PokeMove>>,
    pub move2: Option<Arc<PokeMove>>,
    pub move3: Option<Arc<PokeMove>>,
    pub move4: Option<Arc<PokeMove>>,

    pub status: (PermStatus, VolatileStatus),
    pub current_hp: i32,

}

pub enum Health {
    Full,
    Percent(i32),
    Add(i32),
    AddPercent(i32),
    Subtract(i32),
    SubtractPercent(i32),
    Zero,
}

#[derive(Debug)]
enum Effective {
    NoEffect,
    DoubleResist,
    Resist,
    Normal,
    Super,
    DoubleSuper,
}

pub enum StatXP {
    Max,
    Zero,
    HP(i32),
    Attack(i32),
    Defense(i32),
    Speed(i32),
    Special(i32),
    All(i32,i32,i32,i32,i32)
}



/*
_SuperEffectiveText::
	text "It's super"
	line "effective!"
	prompt

_NotVeryEffectiveText::
	text "It's not very"
	line "effective..."
	prompt
 */

impl Pokemon {
    pub fn new(base: Arc<BasePokemon>, ivs: Option<u16>, level: Option<i32>, nickname: Option<String>, stat_xp: StatXP) -> Self {
        let level = match level {
            Some(l) => l,
            None => 100,
        };

        let nickname = match nickname {
            Some(s) => s,
            None => base.name.clone()
        };

        let ivs = match ivs {
            Some(i) => i,
            None => 0xffff,
        };

        let stat_xp = match stat_xp {
            StatXP::Max => vec!(0xffff, 0xffff, 0xffff, 0xffff, 0xffff),
            StatXP::HP(h) => vec!(h,0,0,0,0),
            StatXP::Attack(a) => vec!(0,a,0,0,0),
            StatXP::Defense(d) => vec!(0,0,d,0,0),
            StatXP::Speed(s) => vec!(0,0,0,s,0),
            StatXP::Special(s) => vec!(0,0,0,0,s),
            StatXP::All(h, a, d, s, sp) => vec!(h,a,d,s,sp),
            StatXP::Zero => vec!(0,0,0,0,0),
        };        

        Pokemon {
            basemon: base.clone(),
        
            name: nickname.clone(),
            level,
            xp: 0,
            hp: Self::hp_calculator(base.base_hp, Self::get_iv(Stat::Hp, ivs), stat_xp[0], level),
            attack: Self::stat_calculator(base.base_attack, Self::get_iv(Stat::Attack, ivs), stat_xp[1], level),
            defense: Self::stat_calculator(base.base_defense, Self::get_iv(Stat::Defense, ivs), stat_xp[2], level),
            speed: Self::stat_calculator(base.base_speed, Self::get_iv(Stat::Speed, ivs), stat_xp[3], level),
            special: Self::stat_calculator(base.base_special, Self::get_iv(Stat::Special, ivs), stat_xp[4], level),
            iv: ivs,
            hp_ev: stat_xp[0],
            attack_ev: stat_xp[1],
            defense_ev: stat_xp[2],
            speed_ev: stat_xp[3],
            special_ev: stat_xp[4],
            move1: base.default_moves.get(0).map_or_else(|| None, |v| Some(v.clone())),
            move2: base.default_moves.get(1).map_or_else(|| None, |v| Some(v.clone())),
            move3: base.default_moves.get(2).map_or_else(|| None, |v| Some(v.clone())),
            move4: base.default_moves.get(3).map_or_else(|| None, |v| Some(v.clone())),
            status: (PermStatus::Healthy, VolatileStatus::None),
            current_hp: Self::hp_calculator(base.base_hp, Self::get_iv(Stat::Hp, ivs), stat_xp[0], level),
            guid: Uuid::new_v4(),
        }
    }

    pub fn get_stat(&self, stat: Stat) -> i32 {
        match stat {
            Stat::Hp => self.hp,
            Stat::Attack => self.attack,
            Stat::Defense => self.defense,
            Stat::Speed => self.speed,
            Stat::Special => self.special,
        }
    }

    fn get_iv (mask: Stat, iv: u16) -> i32 {
        match mask {
            Stat::Attack => return ((iv & 0xF000) >> 12)  as i32,
            Stat::Defense => return ((iv & 0x0F00) >> 8) as i32,
            Stat::Speed => return ((iv & 0x00F0) >> 4) as i32,
            Stat::Special => return (iv & 0x000F) as i32,
            Stat::Hp => return (((iv & 0x1000) >> 9) + ((iv & 0x0100) >> 6) + ((iv & 0x0010) >> 3) + ((iv & 0x0001))) as i32,
        }
    }

    fn stat_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
        let statxp: i32 = (statxp as f32).sqrt().ceil() as i32;
        let statxp: i32 = cmp::min(statxp, 255);
        return (((((base+iv) * 2) + (statxp/4))*level)/100)+5;
    }
    
    fn hp_calculator(base: i32, iv: i32, statxp: i32, level: i32) -> i32 {
        return Self::stat_calculator(base, iv, statxp, level) + level + 5;
    }

    pub fn set_hp(&mut self, hp: Health) -> PermStatus {
        self.current_hp = match hp {
            Health::Full => self.hp,
            Health::Percent(per) => (self.hp*per)/100,
            Health::Add(heal) => cmp::min(self.current_hp+heal,self.hp),
            Health::Subtract(dmg) => cmp::max(self.current_hp-dmg,0),
            Health::AddPercent(per) => cmp::min(self.current_hp+(self.hp*per)/100,self.hp),
            Health::SubtractPercent(per) => cmp::max(self.current_hp-(self.hp*per)/100,0),
            Health::Zero => 0,
        };
        if self.current_hp == 0 {
            self.status = (PermStatus::Fainted, VolatileStatus::None)
        }
        return self.status.0;
    }

    pub fn get_status(&self) -> (PermStatus, VolatileStatus) {
        self.status
    }

    fn get_effective(attack_type: &PokeType, target: &Pokemon) -> Effective {
        match (&target.basemon.type1, &target.basemon.type2) {
            (t1,_) if attack_type.no_effect.contains(&t1.index) => Effective::NoEffect,
            (_,Some(t2)) if attack_type.no_effect.contains(&t2.index) => Effective::NoEffect,
            (t1,Some(t2)) if attack_type.weak.contains(&t1.index) && attack_type.weak.contains(&t2.index) => Effective::DoubleResist,
            (t1,_) if attack_type.weak.contains(&t1.index) => Effective::Resist,
            (_,Some(t2)) if attack_type.weak.contains(&t2.index) => Effective::Resist,
            (t1,Some(t2)) if attack_type.strong.contains(&t1.index) && attack_type.strong.contains(&t2.index) => Effective::DoubleSuper,
            (t1,_) if attack_type.strong.contains(&t1.index) => Effective::Super,
            (_,Some(t2)) if attack_type.strong.contains(&t2.index) => Effective::Super,
            (_,_) => Effective::Normal
        }
    }

    fn dmg_calc(crit: bool, level: i32, power: i32, attack: i32, defense: i32, stab: bool, effective: &Effective, random: i32) -> i32 {
        
        let (attack, defense) = match (attack, defense) {
            (att,def) if att > 255 || def > 255 => (att/4, def/4),
            (_,_) => (attack, defense),
        };

        let attack = match attack {
            att if att < 1 => 1,
            _ => attack,
        };
        
        let crit = match crit {
            true => 2,
            false => 1,
        };

        let dmg = (((crit*2*level)/5)+2) * power * attack;
        let dmg = dmg.checked_div(defense);
        let dmg = match dmg {
            Some(d) => (d / 50) + 2,
            None => panic!("DIV BY ZERO"),
        };

        let dmg = match stab {
            true => dmg * 3 / 2,
            false => dmg,
        };

        let dmg = match effective {
            Effective::NoEffect => dmg*0,
            Effective::DoubleResist => dmg/4,
            Effective::Resist => dmg/2,
            Effective::Normal => dmg,
            Effective::Super => dmg*2,
            Effective::DoubleSuper => dmg*4,
        };
        dmg * random / 255
    }

    pub fn attack(&mut self, defender: &Pokemon, pokemove: &PokeMove) -> (Vec<BattleMessage>, i32) {
        let mut rng = rand::thread_rng();

        let stab: bool = self.basemon.type1 == pokemove.move_type || self.basemon.type2.as_ref() == Some(&pokemove.move_type);

        let crit: bool = rng.gen_range(0..=255) <= self.basemon.base_speed/2;

        let effective: Effective = Self::get_effective(&pokemove.move_type, &defender);

        let (attack, defense) = match pokemove.move_type.category {
            MoveType::Physical => (self.attack, defender.defense),
            MoveType::Special => (self.special, defender.special),
        };

        let random = rng.gen_range(217..=255);

        let dmg = Self::dmg_calc(crit, self.level, pokemove.power, attack, defense, stab, &effective, random);

        
        let did_hit = rng.gen_range(0..=255) < pokemove.accuracy;

        let mut messages: Vec<BattleMessage> = Vec::new();
        match did_hit {
            true => {
                match crit {
                    true => messages.push(BattleMessage::CriticalHit),
                    false => (),
                };

                match effective {
                    Effective::NoEffect => messages.push(BattleMessage::NoEffect),
                    Effective::DoubleResist | Effective::Resist => messages.push(BattleMessage::NotVeryEffective),
                    Effective::Normal => (),
                    Effective::Super | Effective::DoubleSuper => messages.push(BattleMessage::SuperEffective),
                };
            },
            false => messages.push(BattleMessage::Missed),
        };

        return (messages, dmg);
    }

    pub fn debug_pkmn_structure(&self, trainer_id: i32) -> String {
        let type2_index = match &self.basemon.type2 {
            Some(type2) => type2.index,
            None => self.basemon.type1.index,
        };
        let (move1_index, move1_pp) = match &self.move1 {
            Some(move1) => (move1.id, move1.pp),
            None => (0,0),
        };
        let (move2_index, move2_pp) = match &self.move2 {
            Some(mov) => (mov.id, mov.pp),
            None => (0,0),
        };
        let (move3_index, move3_pp) = match &self.move3 {
            Some(mov) => (mov.id, mov.pp),
            None => (0,0),
        };        
        let (move4_index, move4_pp) = match &self.move4 {
            Some(mov) => (mov.id, mov.pp),
            None => (0,0),
        };
        
        format!("{:02x}{:04x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:04x}{:06x}{:04x}{:04x}{:04x}{:04x}{:04x}{:04x}{:02x}{:02x}{:02x}{:02x}{:02x}{:04x}{:04x}{:04x}{:04x}{:04x}", 
            self.basemon.index,
            self.current_hp,
            self.level,
            00, // Zero for degub purposes means healthy. todo!()
            self.basemon.type1.index,
            type2_index,
            self.basemon.catch_rate,
            move1_index,
            move2_index,
            move3_index,
            move4_index,
            trainer_id,
            self.xp,
            self.hp_ev,
            self.attack_ev,
            self.defense_ev,
            self.speed_ev,
            self.special_ev,
            self.iv,
            move1_pp,
            move2_pp,
            move3_pp,
            move4_pp,
            self.level,
            self.hp,
            self.attack,
            self.defense,
            self.speed,
            self.special
        )
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (move1_name, move1_power): (String, String) = match &self.move1 {
            Some(pokemove) => (pokemove.name.clone(), pokemove.power.to_string()),
            None => ("None".to_owned(),"-".to_owned()),
        };
        let (move2_name, move2_power): (String, String) = match &self.move2 {
            Some(pokemove) => (pokemove.name.clone(), pokemove.power.to_string()),
            None => ("None".to_owned(),"-".to_owned()),
        };
        let (move3_name, move3_power): (String, String) = match &self.move3 {
            Some(pokemove) => (pokemove.name.clone(), pokemove.power.to_string()),
            None => ("None".to_owned(),"-".to_owned()),
        };
        let (move4_name, move4_power): (String, String) = match &self.move4 {
            Some(pokemove) => (pokemove.name.clone(), pokemove.power.to_string()),
            None => ("None".to_owned(),"-".to_owned()),
        };
        write!(f, "Name: {},\n  level:\t{},\n  hp:\t\t{}/{},\n  attack:\t{}\n  defense:\t{}\n  special:\t{}\n  speed:\t{}\nMoves:\n  {}:\t{}\n  {}:\t{}\n  {}:\t{}\n  {}:\t{}", 
                self.basemon.name,
                self.level,
                self.current_hp,
                self.hp,
                self.attack,
                self.defense,
                self.special,
                self.speed,
                move1_name,
                move1_power,
                move2_name,
                move2_power,
                move3_name,
                move3_power,
                move4_name,
                move4_power,
        )
    }
}