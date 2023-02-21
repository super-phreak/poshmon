use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "fn", rename_all = "snake_case")]
pub enum Datagram {
    CreateGame {
        
    },
    SubmitTeam {
        session_id: String, 
        client_id: String, 
        name: String, 
        team: Vec<i64>
    },
    SendMove {
        session_id: String,
        client_id: String,
        pokemon_guid: String,
        move_id: i32,
    },
    GetTeam {
        session_id: String, 
        client_id: String, 
        name: String, 
        //team: Vec<i64>, 
    },
    Awk {
        session_id: String, 
        cmd_response: String
    },
    BattleResult {
        //gamestate: GameStateModel, 
        client_id: String, 
        session_id: String
    }
    //Chat {client_id: String, recipient: String, chat_msg: String}
}