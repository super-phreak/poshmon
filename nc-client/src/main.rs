#![crate_name = "poshmon_nclient"]

pub mod graphics;

extern crate poshmon_lib;

use std::{sync::Arc, collections::HashMap};
use graphics::{Canvas, Sprite};
use poshmon_lib::engine::{gen1::{BasePokemon, Pokemon, StatXP}, generics::SpriteData};
use std::io::{self};

// use poshmon_lib::engine::gen1::{PermStatus, VolatileStatus};

fn main() -> io::Result<()>{
    use graphics::{Viewport, Pallets, BulitinPallets};
    let mut stdout = io::stdout();
    let pallets = Pallets::init();
    let viewport = Viewport::new(160, 144, 0, 0);
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        // let viewport = Viewport::new(56, 28, 0, 0, 8, pallets[BulitinPallets::GbClassicColors].clone());
        // viewport.draw_canvas();
    } else {
        println!("Unable to get term size :(")
    }

    use poshmon_lib::engine::gen1::{PokeType, MoveType, PokeMove};

    let ground_type = Arc::new(PokeType {
        index: 4,
        name: "GROUND".to_owned(),
        category: MoveType::Physical,
        strong: vec!(20,23,3,5),
        weak: vec!(22,7),
        no_effect: vec!(2),
    });

    let rock_type = Arc::new(PokeType {
        index: 5,
        name: "ROCK".to_owned(),
        category: MoveType::Physical,
        strong: vec!(20,25,2,7),
        weak: vec!(1,4),
        no_effect: vec!(),
    });

    let normal_type = Arc::new(PokeType{
        index: 0,
        name: "NORMAL".to_owned(),
        category: MoveType::Physical,
        strong: vec!(),
        weak: vec!(5),
        no_effect: vec!(8),
    });

    let ice_type = Arc::new(PokeType{
        index: 25,
        name: "ICE".to_owned(),
        category: MoveType::Special,
        strong: vec!(22,4,2,26),
        weak: vec!(21,25),
        no_effect: vec!(),
    });

    let earthquake = Arc::new(PokeMove {
        id: 89,
        name: "EARTHQUAKE".to_owned(),
        effect: 0,
        power: 100,
        move_type: ground_type.clone(),
        accuracy: 255,
        pp: 10,
        priority: 0,
    });

    let ice_beam = Arc::new(PokeMove {
        id: 58,
        name: "ICE BEAM".to_owned(),
        effect: 5,
        power: 95,
        move_type: ice_type.clone(),
        accuracy: 255,
        pp: 10,
        priority: 0,
    });

    let comet_punch = Arc::new(PokeMove {
        id: 4,
        name: "COMET PUNCH".to_owned(),
        effect: 29,
        power: 18,
        move_type: normal_type.clone(),
        accuracy: 216,
        pp: 15,
        priority: 0,
    });

    let rhydon_front_sprite_data = SpriteData::new(7,7,4, 8,"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACoAAAAAqgAAAAAAAAAAJcAAAAoHAAAAAAAAAAA1bAAAIFsAAAAAAAAAADVrwALL7AAAAAAAAAAADWr//y6sAAAAAAAAAAANr4Aduq8AAAAAAAAAACvgAEXqqwALwAAAAAAAxVARFVamyvBwAAAAADw+qkRVVVV8CrAAAAAAwzf9VVVVVVupwAAAAADA9alVVRVVVeV/AAAAAOQ1qVVQW1VVdVrwAAAAOQ1lVQW5V1VdpVwAAAA5QwVQJtJV9V6V/AAAAA5QwACXQJVtV18AAAAADpIwAZ4A1bVXqsAAAAAO+AwGf4OV/1XlwAAAAAulCwZL+VUA3d8AAAAAD6lmFe8FVVLX+cAAAAAN6pVV8BVar166wAAAAA3/VVVVVWr5WW/wAAAAPXqWVVVWqq1VVqwAAACNXqlVVb/+q5lVawAAAgdAFVVbQ/q+ZlVawAACB4LW1W0P+umqVVbAAAgF8BVW/z9/6qq6lbAACBWvFb//1f+qq8VmsAAIVqv/6qV//qqsBV6wAA1aqvqVV//qqvAFV7AAM1q9b///+qarMBVXsAA3/81Vaqqlmqx5VWvAAA8cNVVVVVZmq/5aqwAAAPA1VVVVVZmqw6quwAAAACVVv//+ZqrG+rnAAAAA1VvAAFfqqr8O/wAAAADVbAAAVb65Wx//wAAAAPVwAAFWqwEV+v6wAAACVbAAAVasAEVeurAAAA0FsAABWrABFVe6rAAACAa/////wABG6uqrAAAgBrVVVVrQARs+6v7AADAWtAAFa1VEW0vrqrAANVq5AAVrVVVrkb/qLAA1Wq0AFatVVqrVYCCcACVqrlVVq5vavuVhWWsADqqr////sGADuXVZWwAOqqq1VWrAgBX61V1bAAOqqq+VagHAVv7WtmsAAOqqq///BsBav+q5qwAAPDsPq8M///68P7qsAAAwHAW8AP////AA//AAADBwF8AAAP//wAAAAAAAD//8AAAAAAAAAAAAA==".to_owned(), "rhydon_front_sprite".to_owned());
    let rhydon_back_sprite_data = SpriteData::new(4,4, 4, 8, "AAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAAAAAAAMwAAAAAPwADDAPwAADAz8wcPAwAADEwcGvAcAAAMbBe/AFwAAAOwFunRcAAAAuBa5VWwAAAMDVuVqrAAAAwHrlW7wAAAMAa5Va0wAA/AFuVVuUwAMH1W5VVqTAAMF1rVVVr7AAMF6tVVW6nAA0W7VVVWv4ADVrtVVVmEwADq7lVValDAAz9rVVv6g4APQWuVVV+sADdBq5VVVuwAG1Wu5VVWsAAa//blVVWsACsVVrVVVawAKwVauVVWrAArAGq6VVqrAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_owned(), "rhydon_back_sprite".to_owned());

    let rhydon_front_sprite: Sprite = rhydon_front_sprite_data.clone().try_into().unwrap();
    //let kangaskhan_front_sprite = Sprite::new(7,7, 2, 8, "AAAAC/AAAAAAAAAAAAAAAAAxbAAAAAAAAAAAAAAAAMF7AAAAAAAAAAAAAAADm//+AAAAAAAAAAAAAAP0BVX8AAAAAAAAAAAAD8ABVVfAAAAAAAAAAAA9AAVVVbwP/8AAAAAAANVAVVVVv+QAMAAAAAALVVVVVVW5AADAAAAAADlVVVVVVsAADgAAAAAA1VVVVVVbwAAwAAAAAANVVVVv/q9AAcAAAAAACVVVVvlT+lALAAAAAAANVVVvVAOqVXwAAAAAAA5VVbLgDVuV8AAAAAAAM+qq4+ANVv+8AAAAAAAwP9v7wLVWqrfAAAAAADAVVb/r1ZWqpsAAAAAADXlVVVVVlaqrAAAAAAAOVVVVVVbVqqwAAAAAAAP/5VVVW1arrAAAAAAAAAHDv/6uVqrv8AAAAAAAAMP/gPlqqrysAAAAAAAAz//w16qv8GwAAAAAA/x7/qzWq/1prAAAAAAPA7fqrd6sA6rDwAAAAAwAt6qrXrAD/rHAAAAAA/h3qql6fV8DpcAAAwAMA+mqrenD8AenwAAJgDAA3Wv16cDBV6n8AA3AMB+fVVelcP++qq/AIcAl+l7V//Vf9qqarzA1wA/9ba9ar/V1WpWtTIXAMA/1dWv9TX1Vlb1M1DAwfHXa/QMDV1VVuW8UHArMd39D8P9XlVa+rF1sAA13AA88bW//6u6xXawADedAB/1teVW/6/1X8AAD5f9AFb/lBVu/w9WwAALl9fVf99AVV8BxdbAAAO8eX7G90BVcAW11gAAA6xulwXXQFVwFbrbAAADLG//Ft0VVXAWu2sAAAwtbQNrXVVVXFqdrAAADGv1AP1Nf6lX+q6sAAAMa1QABQjAV1Vf/rAAADGnUAAAFwVW1Va+4AAAM9bwAAF/FVbVVq+AAAA8FqwVX+sVVtVargAAADxaqv/1qtVbVWqvAAADw+qquFqquq6b/6+AAAwBf6q/qqq/+rAB+8AAMAVV6r///8D6wAVf4AAwVVV6//AAAA8AFVXwAA/////8AAAAA/////AAAAAAAAAAAAAAAAAAAAA==".to_owned());
    let kangaskhan_back_sprite = Sprite::new(4,4, 4, 8, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPwP/wD8AAAPG/BV/wPAADAbAVawBbAAMW1VWsAWsAANdVWqwVrAAAPf+qrFXwAAAPCvq3XwAAAAwFr9X9wAAAIBVVVVzwAAA8TFVVV/AAADMxlVVV7AAAwHKVVVW8AAMBnmxVVqwADBblsBV+sAAMauVwFYvAAAOvpcFtvwAAA/qVxa38AAADalV6ufAAAAMGVW+58AAADFZVauWwAAAxV1VqpawAADFtWaqWqwAADrVqu9arwAA7uqruWqrAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_owned()).unwrap();
    let kangaskhan_back_sprite = kangaskhan_back_sprite.scale_sprite(2).unwrap();

    let default_moves_rhydon = Arc::new(vec!(earthquake.clone()));

    let taught_moves_rhydon = Arc::new(vec!(earthquake.clone()));

    let mut learned_moves_rhydon: HashMap<u8, Arc<PokeMove>> = HashMap::new();
    learned_moves_rhydon.insert(10, earthquake.clone());
    

    // let default_moves_kangaskhan = Arc::new(vec!(comet_punch.clone(), ice_beam.clone()));

    // let taught_moves_kangaskhan = Arc::new(vec!(comet_punch.clone(), ice_beam.clone()));

    let mut learned_moves_kangaskhan: HashMap<u8, Arc<PokeMove>> = HashMap::new();
    learned_moves_kangaskhan.insert(10, comet_punch.clone());
    learned_moves_kangaskhan.insert(15, ice_beam.clone());

    let r_vbuff = rhydon_front_sprite.to_vbuff( false).unwrap();
    let k_vbuff = kangaskhan_back_sprite.to_vbuff(false).unwrap();

    let mut canvas = Canvas::new(160, 140, 8, pallets[BulitinPallets::GbClassicColors].clone(), viewport);
    canvas.print_pallet();
    canvas.add_tile_to_canvas(20-rhydon_front_sprite.width as usize, 0, &(rhydon_front_sprite.width as usize), &(rhydon_front_sprite.height as usize), &r_vbuff);
    canvas.add_tile_to_canvas(1, 5, &(kangaskhan_back_sprite.width as usize), &(kangaskhan_back_sprite.height as usize), &k_vbuff);
    canvas.render(&mut stdout)?;

    let base_rhydon = Arc::new(BasePokemon {
        index: 1,
        pokedex: 112,
        name: "RHYDON".to_owned(),
        catch_rate: 60,
        front_sprite: rhydon_front_sprite_data,
        back_sprite: rhydon_back_sprite_data,
        base_hp: 105,
        base_attack: 130,
        base_defense: 120,
        base_speed: 40,
        base_special: 45,
        type1:ground_type.clone(),
        type2: Some(rock_type.clone()),
        learned_moves: Arc::new(learned_moves_rhydon),
        default_moves: default_moves_rhydon,
        taught_moves: taught_moves_rhydon,
        pokedex_entry: "Protected by an<armor-like hide,<it is capable of^living in molten<lava of 3,600<degrees}".to_owned(),
        species: "DRILL".to_owned(),
        height: 6*12+3,
        weight: 2650,
        evolution_info: Arc::new(vec!()),
    });

    let mut _rhydon = Pokemon::new(base_rhydon.clone(), None, Some(10), Some("BREAKPOINT".to_owned()), StatXP::Max);
    // rhydon.set_status(Some(PermStatus::Paralyzed), Some(VolatileStatus::BadlyPoisoned { turn: 10 }));

    Ok(())
}
