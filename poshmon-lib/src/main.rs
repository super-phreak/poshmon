use std::{sync::Arc, collections::HashMap};

use poshmon_lib::engine::gen1::{graphics, PermStatus, VolatileStatus};

fn main() {
    use poshmon_lib::engine::gen1::{BasePokemon, Pokemon, PokeType, MoveType, PokeMove, StatXP, graphics::Sprite};

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

    let rhydon_front_sprite = Sprite::new(7,7,"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACoAAAAAqgAAAAAAAAAAJcAAAAoHAAAAAAAAAAA1bAAAIFsAAAAAAAAAADVrwALL7AAAAAAAAAAADWr//y6sAAAAAAAAAAANr4Aduq8AAAAAAAAAACvgAEXqqwALwAAAAAAAxVARFVamyvBwAAAAADw+qkRVVVV8CrAAAAAAwzf9VVVVVVupwAAAAADA9alVVRVVVeV/AAAAAOQ1qVVQW1VVdVrwAAAAOQ1lVQW5V1VdpVwAAAA5QwVQJtJV9V6V/AAAAA5QwACXQJVtV18AAAAADpIwAZ4A1bVXqsAAAAAO+AwGf4OV/1XlwAAAAAulCwZL+VUA3d8AAAAAD6lmFe8FVVLX+cAAAAAN6pVV8BVar166wAAAAA3/VVVVVWr5WW/wAAAAPXqWVVVWqq1VVqwAAACNXqlVVb/+q5lVawAAAgdAFVVbQ/q+ZlVawAACB4LW1W0P+umqVVbAAAgF8BVW/z9/6qq6lbAACBWvFb//1f+qq8VmsAAIVqv/6qV//qqsBV6wAA1aqvqVV//qqvAFV7AAM1q9b///+qarMBVXsAA3/81Vaqqlmqx5VWvAAA8cNVVVVVZmq/5aqwAAAPA1VVVVVZmqw6quwAAAACVVv//+ZqrG+rnAAAAA1VvAAFfqqr8O/wAAAADVbAAAVb65Wx//wAAAAPVwAAFWqwEV+v6wAAACVbAAAVasAEVeurAAAA0FsAABWrABFVe6rAAACAa/////wABG6uqrAAAgBrVVVVrQARs+6v7AADAWtAAFa1VEW0vrqrAANVq5AAVrVVVrkb/qLAA1Wq0AFatVVqrVYCCcACVqrlVVq5vavuVhWWsADqqr////sGADuXVZWwAOqqq1VWrAgBX61V1bAAOqqq+VagHAVv7WtmsAAOqqq///BsBav+q5qwAAPDsPq8M///68P7qsAAAwHAW8AP////AA//AAADBwF8AAAP//wAAAAAAAD//8AAAAAAAAAAAAA==".to_owned(), "rhydon_front_sprite".to_owned());
    let rhydon_back_sprite = Sprite::new(4,4,"AAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAAAAAAAMwAAAAAPwADDAPwAADAz8wcPAwAADEwcGvAcAAAMbBe/AFwAAAOwFunRcAAAAuBa5VWwAAAMDVuVqrAAAAwHrlW7wAAAMAa5Va0wAA/AFuVVuUwAMH1W5VVqTAAMF1rVVVr7AAMF6tVVW6nAA0W7VVVWv4ADVrtVVVmEwADq7lVValDAAz9rVVv6g4APQWuVVV+sADdBq5VVVuwAG1Wu5VVWsAAa//blVVWsACsVVrVVVawAKwVauVVWrAArAGq6VVqrAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_owned(), "rhydon_back_sprite".to_owned());

    let kangaskhan_front_sprite = Sprite::new(7,7,"AAAAC/AAAAAAAAAAAAAAAAAxbAAAAAAAAAAAAAAAAMF7AAAAAAAAAAAAAAADm//+AAAAAAAAAAAAAAP0BVX8AAAAAAAAAAAAD8ABVVfAAAAAAAAAAAA9AAVVVbwP/8AAAAAAANVAVVVVv+QAMAAAAAALVVVVVVW5AADAAAAAADlVVVVVVsAADgAAAAAA1VVVVVVbwAAwAAAAAANVVVVv/q9AAcAAAAAACVVVVvlT+lALAAAAAAANVVVvVAOqVXwAAAAAAA5VVbLgDVuV8AAAAAAAM+qq4+ANVv+8AAAAAAAwP9v7wLVWqrfAAAAAADAVVb/r1ZWqpsAAAAAADXlVVVVVlaqrAAAAAAAOVVVVVVbVqqwAAAAAAAP/5VVVW1arrAAAAAAAAAHDv/6uVqrv8AAAAAAAAMP/gPlqqrysAAAAAAAAz//w16qv8GwAAAAAA/x7/qzWq/1prAAAAAAPA7fqrd6sA6rDwAAAAAwAt6qrXrAD/rHAAAAAA/h3qql6fV8DpcAAAwAMA+mqrenD8AenwAAJgDAA3Wv16cDBV6n8AA3AMB+fVVelcP++qq/AIcAl+l7V//Vf9qqarzA1wA/9ba9ar/V1WpWtTIXAMA/1dWv9TX1Vlb1M1DAwfHXa/QMDV1VVuW8UHArMd39D8P9XlVa+rF1sAA13AA88bW//6u6xXawADedAB/1teVW/6/1X8AAD5f9AFb/lBVu/w9WwAALl9fVf99AVV8BxdbAAAO8eX7G90BVcAW11gAAA6xulwXXQFVwFbrbAAADLG//Ft0VVXAWu2sAAAwtbQNrXVVVXFqdrAAADGv1AP1Nf6lX+q6sAAAMa1QABQjAV1Vf/rAAADGnUAAAFwVW1Va+4AAAM9bwAAF/FVbVVq+AAAA8FqwVX+sVVtVargAAADxaqv/1qtVbVWqvAAADw+qquFqquq6b/6+AAAwBf6q/qqq/+rAB+8AAMAVV6r///8D6wAVf4AAwVVV6//AAAA8AFVXwAA/////8AAAAA/////AAAAAAAAAAAAAAAAAAAAA==".to_owned(), "kangaskhan_front_sprite".to_owned());
    let kangaskhan_back_sprite = Sprite::new(4,4,"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPwP/wD8AAAPG/BV/wPAADAbAVawBbAAMW1VWsAWsAANdVWqwVrAAAPf+qrFXwAAAPCvq3XwAAAAwFr9X9wAAAIBVVVVzwAAA8TFVVV/AAADMxlVVV7AAAwHKVVVW8AAMBnmxVVqwADBblsBV+sAAMauVwFYvAAAOvpcFtvwAAA/qVxa38AAADalV6ufAAAAMGVW+58AAADFZVauWwAAAxV1VqpawAADFtWaqWqwAADrVqu9arwAA7uqruWqrAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_owned(), "kangaskhan_back_sprite".to_owned());

    graphics::print_pallet();

    let default_moves_rhydon = Arc::new(vec!(earthquake.clone()));

    let taught_moves_rhydon = Arc::new(vec!(earthquake.clone()));

    let mut learned_moves_rhydon: HashMap<u8, Arc<PokeMove>> = HashMap::new();
    learned_moves_rhydon.insert(10, earthquake.clone());
    

    let default_moves_kangaskhan = Arc::new(vec!(comet_punch.clone(), ice_beam.clone()));

    let taught_moves_kangaskhan = Arc::new(vec!(comet_punch.clone(), ice_beam.clone()));

    let mut learned_moves_kangaskhan: HashMap<u8, Arc<PokeMove>> = HashMap::new();
    learned_moves_kangaskhan.insert(10, comet_punch.clone());
    learned_moves_kangaskhan.insert(15, ice_beam.clone());

    let base_rhydon = Arc::new(BasePokemon {
        index: 1,
        pokedex: 112,
        name: "RHYDON".to_owned(),
        catch_rate: 60,
        front_sprite: rhydon_front_sprite,
        back_sprite: rhydon_back_sprite,
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

    let mut rhydon = Pokemon::new(base_rhydon.clone(), None, Some(10), Some("BREAKPOINT".to_owned()), StatXP::Max);
    rhydon.set_status(Some(PermStatus::Paralyzed), Some(VolatileStatus::BadlyPoisoned { turn: 10 }));

    let base_kangaskhan = Arc::new(BasePokemon {
        index: 2,
        pokedex: 115,
        name: "KANGASKHAN".to_owned(),
        catch_rate: 45, 
        front_sprite: kangaskhan_front_sprite,
        back_sprite: kangaskhan_back_sprite,
        base_hp: 105,
        base_attack: 95,
        base_defense: 80,
        base_speed: 90,
        base_special: 40,
        type1:normal_type.clone(),
        type2: None,
        learned_moves: Arc::new(learned_moves_kangaskhan),
        default_moves: default_moves_kangaskhan,
        taught_moves: taught_moves_kangaskhan,
        pokedex_entry: "The infant rarely<ventures out of<its mother\u{1e61}^protective pouch<until it is 3<years old}".to_owned(),
        species: "PARENT".to_owned(),
        height: 7*12+3,
        weight: 1760,
        evolution_info: Arc::new(vec!()),
    });

    let mut kangaskhan = Pokemon::new(base_kangaskhan.clone(), None, Some(100), None, StatXP::Max);

    //DO5E in ram is the crit hit flag

    println!("rhydon ram_map: {}",rhydon.debug_pkmn_structure(47662));
    println!("kangaskhan ram_map: {}",kangaskhan.debug_pkmn_structure(47662));

    println!("Rhydon Model: \n{}", rhydon);
    println!("Kangaskhan Model: \n{}", kangaskhan);

    let _result_rhydon = rhydon.attack(&mut kangaskhan, &earthquake);

    let _result_kangaskhan = kangaskhan.attack(&mut rhydon, &ice_beam);

    println!("\n{}", rhydon.to_model(true));
}