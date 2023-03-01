pub mod networking;
pub mod engine;

#[cfg(test)]
mod tests {
    use std::{sync::Arc, collections::HashMap};

    use uuid::Uuid;

    use crate::{networking::{Packet, Communication, SessionToken, Datagram}};    

    #[test]
    fn packet_test() {
        let token = SessionToken::new("testuser".to_owned());
        let team = Datagram::Awk {session_id: token.session_id.to_string(), cmd_response: "Failure to submit team".to_string()};
        let packet = Packet::new(token.clone(), team);
        println!("{}", packet.to_json_str());
        assert!(packet.verify(token).is_ok())

    }

    #[test]
    fn pokemon_battle_test() {
        use crate::engine::gen1::{BasePokemon, Pokemon, PokeType, MoveType, PokeMove, StatXP};

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
        });

        let ice_beam = Arc::new(PokeMove {
            id: 58,
            name: "ICE BEAM".to_owned(),
            effect: 5,
            power: 95,
            move_type: ice_type.clone(),
            accuracy: 255,
            pp: 10,
        });

        let comet_punch = Arc::new(PokeMove {
            id: 4,
            name: "COMET PUNCH".to_owned(),
            effect: 29,
            power: 18,
            move_type: normal_type.clone(),
            accuracy: 216,
            pp: 15,
        });



        let default_moves_rhydon = vec!(earthquake.clone());

        let taught_moves_rhydon = vec!(earthquake.clone());

        let mut learned_moves_rhydon: HashMap<u8, Arc<PokeMove>> = HashMap::new();
        learned_moves_rhydon.insert(10, earthquake.clone());

        let default_moves_kangaskhan = vec!(comet_punch.clone(), ice_beam.clone());

        let taught_moves_kangaskhan = vec!(comet_punch.clone(), ice_beam.clone());

        let mut learned_moves_kangaskhan: HashMap<u8, Arc<PokeMove>> = HashMap::new();
        learned_moves_kangaskhan.insert(10, comet_punch.clone());
        learned_moves_kangaskhan.insert(15, ice_beam.clone());

        let base_rhydon = Arc::new(BasePokemon {
            index: 1,
            pokedex: 112,
            name: "RHYDON".to_owned(),
            catch_rate: 60,
            sprite_id: Uuid::new_v4(),
            base_hp: 105,
            base_attack: 130,
            base_defense: 120,
            base_speed: 40,
            base_special: 45,
            type1:ground_type.clone(),
            type2: Some(rock_type.clone()),
            learned_moves: learned_moves_rhydon,
            default_moves: default_moves_rhydon,
            taught_moves: taught_moves_rhydon,
            pokedex_entry: "Protected by an<armor-like hide,<it is capable of^living in molten<lava of 3,600<degrees}".to_owned(),
        });

        let mut rhydon = Pokemon::new(base_rhydon.clone(), None, None, None, StatXP::Zero);

        let base_kangaskhan = Arc::new(BasePokemon {
            index: 2,
            pokedex: 115,
            name: "KANGASKHAN".to_owned(),
            catch_rate: 45, 
            sprite_id: Uuid::new_v4(),
            base_hp: 105,
            base_attack: 95,
            base_defense: 80,
            base_speed: 90,
            base_special: 40,
            type1:normal_type.clone(),
            type2: None,
            learned_moves: learned_moves_kangaskhan,
            default_moves: default_moves_kangaskhan,
            taught_moves: taught_moves_kangaskhan,
            pokedex_entry: "The infant rarely<ventures out of<its mother\u{1e61}^protective pouch<until it is 3<years old}".to_owned(),
        });

        let mut kangaskhan = Pokemon::new(base_kangaskhan.clone(), None, Some(100), None, StatXP::Max);

        //DO5E in ram is the crit hit flag

        println!("rhydon ram_map: {}",rhydon.debug_pkmn_structure(47662));
        println!("kangaskhan ram_map: {}",kangaskhan.debug_pkmn_structure(47662));

        println!("Rhydon Model: \n{}", rhydon);
        println!("Kangaskhan Model: \n{}", kangaskhan);

        let _result_rhydon = rhydon.attack(&mut kangaskhan, &earthquake);

        //println!("The result of the battle was: {:#?}", result_rhydon);

        let _result_kangaskhan = kangaskhan.attack(&mut rhydon, &ice_beam);

        //println!("The result of the battle was: {:#?}", result_kangaskhan);
    
    }
}
