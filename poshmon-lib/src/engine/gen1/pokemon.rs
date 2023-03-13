use std::{
    sync::Arc, 
    collections::HashMap, cmp, fmt::Display, 
};

use rand::Rng;
use uuid::Uuid;

use crate::engine::gen1::MoveType;

use super::{
    PokeType, 
    pokemove::PokeMove, 
    graphics::{
        Sprite, 
        Viewport
    }, 
    game::BattleMessage
};

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

#[derive(Clone)]
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

impl BasePokemon {
    pub fn debug_graphic(&self) -> String {
        let base = self.debug_info();

        let mut output = String::new();

        let (canvas_width, canvas_height): (usize, usize) = match term_size::dimensions() {
            Some(size) => (size.0, (self.front_sprite.get_bounds().1*4) as usize),
            None => (45,45),
        };
        let viewport = Viewport::new(canvas_width-45, canvas_height, 0, 0);

        let front_sprite = self.front_sprite.draw_sprite(false, Some(viewport));

        let sprite: Vec<&str> = front_sprite.lines().collect();
        let print_len = cmp::max(base.len(), sprite.len() as usize);
        output.push_str(format!("{:<54}{}", base[0], sprite[0]).as_str());
        for i in 1..print_len as usize {
            let sprite_line = match sprite.get(i) {
                Some(line) => line,
                None => "",
            };

            let stat_line = match base.get(i) {
                Some(line) => line,
                None => "",
            };
            
            output.push_str(format!("\n{:<54}{}", stat_line, sprite_line).as_str());
        }
        output
    }

    fn debug_info(&self) -> Vec<String> {
        let mut base: Vec<String> = Vec::new();
        let header: String = format!("{} <p{:03}|i{:03}>", self.species, self.pokedex, self.index);
        base.push(format!("{:^50}", self.name.clone()));
        base.push(format!("{:^50}", header));
        let type2: String = match &self.type2 {
            Some(poketype) => poketype.name.clone(),
            None => "NONE".to_owned(),
        };
        //base.push(format!("{:<8}{:<7}{:<9}  {:<7}{}", "", "Type1:", self.type1.name, "Type2:", type2));
        //base.push("".to_string());
        base.push(format!("{:<4}{}{:12}{}", "", "-----STATS-----", "", "-----INFO-----"));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{}{:>9}", "", "HP:", self.base_hp, "", "Typ1:", self.type1.name));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{}{:>9}", "", "Attack:", self.base_attack, "", "Typ2:", type2));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{:<10}{:>4}", "", "Defense", self.base_defense, "", "CatchRate:", self.catch_rate));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{:<10}{:>4}", "", "Speed:", self.base_speed, "", "Weight:", self.weight));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{:<10}{:>4}", "", "Special:", self.base_special, "", "Feet:", self.height/12));
        base.push(format!("{:<4}{:<11}{:>4}{:12}{:<10}{:>4}", "", "Total:", self.base_hp + self.base_attack + self.base_defense + self.base_speed + self.base_special, "", "Inches:", self.height%12));
        
        
        base.push("".to_string());
        base.push(format!("{:^50}", "-----DEFAULT MOVES-----"));
        let default_move1 = match self.default_moves.get(0) {
            Some(pokemove) => pokemove.name.clone(),
            None => "  ---".to_owned(),
        };
        let default_move2 = match self.default_moves.get(1) {
            Some(pokemove) => pokemove.name.clone(),
            None => "  ---".to_owned(),
        };
        let default_move3 = match self.default_moves.get(2) {
            Some(pokemove) => pokemove.name.clone(),
            None => "  ---".to_owned(),
        };
        let default_move4 = match self.default_moves.get(3) {
            Some(pokemove) => pokemove.name.clone(),
            None => "  ---".to_owned(),
        };

        base.push(format!("{:<9}{:^14}{:<2}{:^14}", "", default_move1, "", default_move3));
        base.push(format!("{:<9}{:^14}{:<2}{:^14}", "", default_move2, "", default_move4));


        
        base.push("".to_string());
        base.push(format!("{:^50}", "-----LEARNED MOVES-----"));
        
        let mut columns: [Vec<String>; 3] = [Vec::new(), Vec::new(), Vec::new()];

        let mut index = 0;
        for (level, pokemove) in self.learned_moves.iter()  {
            columns[index%3].push(format!("{:2}: {}", level, pokemove.name.clone()));
            index = (index + 1) % 3;
        }

        let rows = cmp::max(cmp::max(columns[0].len(), columns[1].len()), columns[2].len());
        for row in 0..rows {
            let val0 = columns[0].get(row).unwrap_or(&"".to_string()).clone();
            let val1 = columns[1].get(row).unwrap_or(&"".to_string()).clone();
            let val2 = columns[2].get(row).unwrap_or(&"".to_string()).clone();

            base.push(format!("{:<18}{:<18}{:<18}", val0, val1, val2))
        }

        base.push("".to_string());
        base.push(format!("{:^50}", "------TAUGHT MOVES-----"));

        let mut columns: [Vec<String>; 3] = [Vec::new(), Vec::new(), Vec::new()];

        let mut index = 0;
        for pokemove in self.taught_moves.iter()  {
            columns[index%3].push(pokemove.name.clone());
            index = (index + 1) % 3;
        }

        let rows = cmp::max(cmp::max(columns[0].len(), columns[1].len()), columns[2].len());
        for row in 0..rows {
            let val0 = columns[0].get(row).unwrap_or(&"".to_string()).clone();
            let val1 = columns[1].get(row).unwrap_or(&"".to_string()).clone();
            let val2 = columns[2].get(row).unwrap_or(&"".to_string()).clone();

            base.push(format!("{:<18}{:<18}{:<18}", val0, val1, val2))
        }
        base
    }
}

//#[derive(Debug)]
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

    pub move1: Option<InstantiatedMove>,
    pub move2: Option<InstantiatedMove>,
    pub move3: Option<InstantiatedMove>,
    pub move4: Option<InstantiatedMove>,

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

pub struct InstantiatedMove {
    pub data: Arc<PokeMove>,
    pub current_pp: i32,
}

impl InstantiatedMove {
    pub fn new(data: Arc<PokeMove>, current_pp: i32) -> Self {
        InstantiatedMove { data, current_pp }
    }
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
            move1: base.default_moves.get(0).map_or_else(|| None, |v| Some(InstantiatedMove::new(v.clone(), v.pp))),
            move2: base.default_moves.get(1).map_or_else(|| None, |v| Some(InstantiatedMove::new(v.clone(), v.pp))),
            move3: base.default_moves.get(2).map_or_else(|| None, |v| Some(InstantiatedMove::new(v.clone(), v.pp))),
            move4: base.default_moves.get(3).map_or_else(|| None, |v| Some(InstantiatedMove::new(v.clone(), v.pp))),
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
            MoveType::Error => (1,1)
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

    pub fn set_status(&mut self, perm: Option<PermStatus>, volatile: Option<VolatileStatus>) {
        match perm {
            Some(stat) => self.status.0 = stat,
            None => ()
        };

        match volatile {
            Some(stat) => self.status.1 = stat,
            None => ()
        }
    }

    pub fn debug_pkmn_structure(&self, trainer_id: i32) -> String {
        let type2_index = match &self.basemon.type2 {
            Some(type2) => type2.index,
            None => self.basemon.type1.index,
        };
        let (move1_index, move1_pp) = match &self.move1 {
            Some(move1) => (move1.data.id, move1.current_pp),
            None => (0,0),
        };
        let (move2_index, move2_pp) = match &self.move2 {
            Some(mov) => (mov.data.id, mov.current_pp),
            None => (0,0),
        };
        let (move3_index, move3_pp) = match &self.move3 {
            Some(mov) => (mov.data.id, mov.current_pp),
            None => (0,0),
        };        
        let (move4_index, move4_pp) = match &self.move4 {
            Some(mov) => (mov.data.id, mov.current_pp),
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

    pub fn print_battle_stats(&self, player: bool) -> String {
        let sprite = match player {
            true => &self.basemon.back_sprite,
            false => &self.basemon.front_sprite,
        };
        

        let (canvas_width, canvas_height): (usize, usize) = match term_size::dimensions() {
            Some(size) => match player {
                true => (size.0, (sprite.get_bounds().1*4-1) as usize),
                false => (size.0, (sprite.get_bounds().1*4) as usize),
            }
            None => (45,45),
        };
        let viewport = Viewport::new(canvas_width-45, canvas_height, 0, 0);
        let sprite = sprite.draw_sprite(false, Some(viewport));
        let sprite: Vec<&str> = sprite.lines().collect();
        let mut stats: Vec<String> = Vec::new();
        let mut output: String = "".to_owned();

        if self.name == self.basemon.name {
            stats.push(format!("{}", self.name));
        } else {
            stats.push(format!("{:<10} ({})", self.name, self.basemon.name));
        }
        stats.push(format!("{:^45}", self.guid.to_string()));
        stats.push(format!("  {:<15}{:>3}", "Level:", self.level));
        stats.push(format!("  {:<15}", "---Status---"));
        stats.push(format!("    {:<13}{:?}", "Perm:", self.status.0));
        stats.push(format!("    {:<13}{:?}", "Volatile:", self.status.1));
        stats.push("  ---Stats---".to_string());
        stats.push(format!("    {:<9}{:>3}/{:>3} (0x{:04X})", "HP:", self.current_hp, self.hp, self.hp_ev));
        stats.push(format!("    {:<13}{:>3} (0x{:04X})", "Attack:", self.attack, self.attack_ev));
        stats.push(format!("    {:<13}{:>3} (0x{:04X})", "Defence:", self.defense, self.defense_ev));
        stats.push(format!("    {:<13}{:>3} (0x{:04X})", "Speed:", self.speed, self.speed_ev));
        stats.push(format!("    {:<13}{:>3} (0x{:04X})", "Special:", self.special, self.special_ev));
        stats.push(format!("    {:<13}0x{:04X}", "IVs:", self.iv, ));
        stats.push("  ---Moves---".to_string());

        //Move info
        let mut move_data: Vec<(String, String, i32, i32, String)> = Vec::new();
        move_data.push(
            match &self.move1 {
                Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.power.to_string(), pokemove.data.pp, pokemove.current_pp, pokemove.data.move_type.name.clone()),
                None => ("None".to_owned(),"-".to_owned(),0,0,"-".to_owned()),
            }
        );
        move_data.push(
            match &self.move2 {
                Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.power.to_string(), pokemove.data.pp, pokemove.current_pp, pokemove.data.move_type.name.clone()),
                None => ("None".to_owned(),"-".to_owned(),0,0,"-".to_owned()),
            }
        );
        move_data.push(
            match &self.move3 {
                Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.power.to_string(), pokemove.data.pp, pokemove.current_pp, pokemove.data.move_type.name.clone()),
                None => ("None".to_owned(),"-".to_owned(),0,0,"-".to_owned()),
            }
        );
        move_data.push(
            match &self.move4 {
                Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.power.to_string(), pokemove.data.pp, pokemove.current_pp, pokemove.data.move_type.name.clone()),
                None => ("None".to_owned(),"-".to_owned(),0,0,"-".to_owned()),
            }
        );
        for moves in move_data {
            stats.push(format!("    {:<13}{:>2}/{:>2}", moves.0, moves.3, moves.2));
            stats.push(format!("      Type: {:<8} PWR: {}", moves.4, moves.1));
        }
        
        
        
        let print_len = cmp::max(stats.len(), sprite.len() as usize);
        output.push_str(format!("{:^45}{}", stats[0], sprite[0]).as_str());
        for i in 1..print_len as usize {
            let sprite_line = match sprite.get(i) {
                Some(line) => line,
                None => "",
            };

            let stat_line = match stats.get(i) {
                Some(line) => line,
                None => "",
            };
            
            output.push_str(format!("\n{:<45}{}", stat_line, sprite_line).as_str());
        }
        output
    }

    pub fn to_model(&self, reduced: bool) -> PokemonModel {
        match reduced {
            true => PokemonModel::Reduced { 
                index: self.basemon.index, 
                pokedex: self.basemon.pokedex, 
                name: self.basemon.name.clone(), 
                front_sprite: self.basemon.front_sprite.get_sprite_data(), 
                back_sprite: self.basemon.back_sprite.get_sprite_data(), 
                
                guid: self.guid.to_string(), 
                nickname: self.name.clone(), 
                level: self.level, 
                current_hp: (self.current_hp*48/self.hp), 
                max_hp: 48,
                perm_status: self.status.0,
                volatile_status: self.status.1  
            },
            false => {
                let mut move_data: Vec<(String, String, i32, i32, i32)> = Vec::new();
                move_data.push(
                    match &self.move1 {
                        Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.move_type.name.clone(), pokemove.data.power, pokemove.data.pp, pokemove.current_pp),
                        None => ("None".to_owned(),"-".to_owned(),0,0,0),
                    }
                );
                move_data.push(
                    match &self.move2 {
                        Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.move_type.name.clone(), pokemove.data.power, pokemove.data.pp, pokemove.current_pp),
                        None => ("None".to_owned(),"-".to_owned(),0,0,0),
                    }
                );
                move_data.push(
                    match &self.move3 {
                        Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.move_type.name.clone(), pokemove.data.power, pokemove.data.pp, pokemove.current_pp),
                        None => ("None".to_owned(),"-".to_owned(),0,0,0),
                    }
                );
                move_data.push(
                    match &self.move4 {
                        Some(pokemove) => (pokemove.data.name.clone(), pokemove.data.move_type.name.clone(), pokemove.data.power, pokemove.data.pp, pokemove.current_pp),
                        None => ("None".to_owned(),"-".to_owned(),0,0,0),
                    }
                );
                PokemonModel::Full { 
                    index: self.basemon.index, 
                    pokedex: self.basemon.pokedex, 
                    name: self.basemon.name.clone(), 
                    front_sprite: self.basemon.front_sprite.get_sprite_data(), 
                    back_sprite: self.basemon.back_sprite.get_sprite_data(), 
                    
                    guid: self.guid.to_string(), 
                    nickname: self.name.clone(), 
                    level: self.level, 
                    current_hp: self.current_hp, 
                    max_hp: self.hp,
                    perm_status: self.status.0,
                    volatile_status: self.status.1,

                    attack: self.attack, 
                    defense: self.defense, 
                    speed: self.speed, 
                    special: self.special, 
                    
                    move_data,
                }
            }
        }
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_battle_stats(false))
    }
}

pub enum PokemonModel {
    Reduced {
        index: u8,
        pokedex: u8,
        name: String,
        front_sprite: (i32,i32,String),
        back_sprite: (i32,i32,String),

        guid: String,
        nickname: String,
        level: i32,
        current_hp: i32,
        max_hp: i32,
        perm_status: PermStatus,
        volatile_status: VolatileStatus,
    },
    Full {
        index: u8,
        pokedex: u8,
        name: String,
        front_sprite: (i32,i32,String),
        back_sprite: (i32,i32,String),

        guid: String,
        nickname: String,
        level: i32,
        current_hp: i32,
        max_hp: i32,
        perm_status: PermStatus,
        volatile_status: VolatileStatus,

        attack: i32,
        defense: i32,
        speed: i32,
        special: i32,
        
        move_data: Vec<(String, String, i32, i32, i32)>
    }
}

impl PokemonModel {
    pub fn get_display_prefix(&self) -> Vec<String> {
        let mut stats: Vec<String> = Vec::new();
        match self {
            PokemonModel::Reduced { index, pokedex, name, front_sprite, back_sprite: _, 
                                    guid, nickname, level, current_hp, max_hp, 
                                    perm_status, volatile_status } => {
                    let header = match name == nickname {
                        true => format!("{} <{:03}|{:03}>", name, pokedex, index),
                        false => format!("{} ({}) <{:03}|{:03}>", nickname, name, pokedex, index),
                    };
                    stats.push(format!("{:^40}", header));
                    stats.push(format!("{:^40}", guid));
                    stats.push("".to_string());
                    stats.push(format!("  {:<20}{:>3}", "Level:", level));
                    stats.push(format!("  {:<16}{:>3}/{:>3} ({:.2}%)", "HP:", current_hp, max_hp, ((*current_hp as f32)/(*max_hp as f32) * 100f32)));
                    stats.push("".to_string());
                    stats.push(format!("{:^30}", "-----Status-----"));
                    stats.push(format!("  {:<13}{:?}", "Perm:", perm_status));
                    stats.push(format!("  {:<13}{:?}", "Volatile:", volatile_status));
                
                let (canvas_width, canvas_height): (usize, usize) = match term_size::dimensions() {
                    Some(size) => (size.0, (front_sprite.1*4) as usize),
                    None => (45,45),
                };
                let viewport = Viewport::new(canvas_width-45, canvas_height, 0, 0);

                stats.push(Sprite::new(front_sprite.0, front_sprite.1, front_sprite.2.clone(), nickname.clone()).draw_sprite(false, Some(viewport)));
            },
            PokemonModel::Full { index, pokedex, name, front_sprite: _, back_sprite, 
                                 guid, nickname, level, current_hp, max_hp, perm_status, 
                                 volatile_status, attack, defense, speed, special, move_data } => {
                let header = match name == nickname {
                    true => format!("{} <{:03}|{:03}>", name, pokedex, index),
                    false => format!("{} ({}) <{:03}|{:03}>", nickname, name, pokedex, index),
                };
                stats.push(format!("{:^40}", header));
                stats.push(format!("{:^40}", guid));
                stats.push("".to_string());
                stats.push(format!("  {:<20}{:>3}", "Level:", level));
                stats.push(format!("  {:<16}{:>3}/{:>3} ({:.2}%)", "HP:", current_hp, max_hp, ((*current_hp as f32)/(*max_hp as f32) * 100f32)));
                stats.push("".to_string());
                stats.push(format!("{:^30}", "-----Status-----"));
                stats.push(format!("  {:<13}{:?}", "Perm:", perm_status));
                stats.push(format!("  {:<13}{:?}", "Volatile:", volatile_status));
                stats.push("".to_string());
                stats.push(format!("{:^30}", "-----Stats-----"));
                stats.push(format!("  {:<20}{:>3}", "Attack:", attack));
                stats.push(format!("  {:<20}{:>3}", "Defence:", defense));
                stats.push(format!("  {:<20}{:>3}", "Speed:", speed));
                stats.push(format!("  {:<20}{:>3}", "Special:", special));
                stats.push("".to_string());

                //Move info
                stats.push(format!("{:^30}", "-----Moves-----"));
                for moves in move_data {
                    let power: String = match moves.2 {
                        0 => "-".to_string(),
                        x => x.to_string(),
                    };
                    stats.push(format!("  {:<20}{:>2}/{:>2}", moves.0, moves.4, moves.3));
                    stats.push(format!("    Type: {:<8} PWR: {:>3}", moves.1, power));
                }

                //(size.0, (sprite.get_bounds().1*4-1) as usize)
                let (canvas_width, canvas_height): (usize, usize) = match term_size::dimensions() {
                    Some(size) => (size.0, (back_sprite.1*4-1) as usize),
                    None => (45,45),
                };
                let viewport = Viewport::new(canvas_width-45, canvas_height, 0, 0);

                stats.push(Sprite::new(back_sprite.0, back_sprite.1, back_sprite.2.clone(), nickname.clone()).draw_sprite(false, Some(viewport)));
            }
        };
        stats
    }
}

impl Display for PokemonModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        let mut stats = self.get_display_prefix();
        let sprite = stats.pop();
        let sprite: String = match sprite {
            Some(spr) => spr,
            None => "".to_string(),
        };
        let sprite: Vec<&str> = sprite.lines().collect();
        let print_len = cmp::max(stats.len(), sprite.len() as usize);
        output.push_str(format!("{:<45}{}", stats[0], sprite[0]).as_str());
        for i in 1..print_len as usize {
            let sprite_line = match sprite.get(i) {
                Some(line) => line,
                None => "",
            };

            let stat_line = match stats.get(i) {
                Some(line) => line,
                None => "",
            };
            
            output.push_str(format!("\n{:<45}{}", stat_line, sprite_line).as_str());
        }
        write!(f, "{}", output)
    }
}

