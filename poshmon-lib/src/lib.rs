pub mod networking;
pub mod engine;

#[cfg(test)]
mod tests {
    use crate::networking::{Packet, Communication, SessionToken, Datagram};    

    #[test]
    fn packet_test() {
        let token = SessionToken::new("testuser".to_owned());
        let team = Datagram::Awk {session_id: token.session_id.to_string(), cmd_response: "Failure to submit team".to_string()};
        let packet = Packet::new(token.clone(), team);
        println!("{}", packet.to_json_str());
        let token = base64::encode(token.session_key);
        assert!(packet.verify(&token).is_ok())

    }
}
